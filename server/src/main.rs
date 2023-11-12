//! # Rust TCP Server
//!
//! This module defines a simple TCP server in Rust that handles multiple connections using threads.
//!
//! ## Features
//!
//! - TCP server with thread pool handling incoming connections.
//! - Parses HTTP requests and generates appropriate responses.
//! - Supports basic routing and file serving.
//!
//! ## Usage
//!
//! To use this server, simply call the `main` function. The server reads configuration data from
//! the `resources/keys/keys.toml` file, including IP address and port.
//!
//! ## Dependencies
//!
//! This module relies on external crates for various utilities:
//!
//! - `log`: A flexible logging framework for Rust.
//! - `dev_utils`: A set of utility functions for development purposes.
//!
//! ## Important Notes
//!
//! - The server responds to HTTP GET requests, handling basic routing.
//! - The server currently serves a default "200 OK" response for every request.
//! - The server may exit after handling a specific number of requests (configured in the code).
//! - Logging is configured with varying levels from `Trace` to `Info`.
//!
//! ## License
//!
//! This code is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
#![allow(unused)]

// ? Module imports -----------------------------------------------------------------------------------------------------------

use std::fmt::Display;
// Standard library imports
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};
use std::path::Path;

// External crates
use log::LevelFilter;
use dev_utils::log::rlog::RLog;
use dev_utils::print_app_data;

use dev_utils::http::{*, response::HttpResponse};

mod thread_pool;

// ? Main ---------------------------------------------------------------------------------------------------------------------
// async fn main() -> Result<(), Box<dyn std::error::Error>> {  // for async main (async-std)
fn main() {
    print_app_data(file!());  // Read the Cargo.toml file and print the app data (name, version, authors)
    // RLog::init_logger(LevelFilter::Info);  // Initialize the logger with the given log level
    RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    // ^ Read the IP address and port from the keys.toml file
    let keys = dev_utils::files::toml::TomlFile::new(Path::new("resources/keys/keys.toml"));
    let section = keys.get_section_data("ip_address");
    let ip = section.unwrap().get_key_value("ip").unwrap();
    let port = section.unwrap().get_key_value("port").unwrap();

    // ^ Actual code: --------------------------------------------------------------------------------------------------------
    let server_address = format!("{}:{}", ip.1, port.1);
    log::info!("Starting server at {}\n", server_address);

    let listener = TcpListener::bind(server_address)  // Listen on this address
        .expect("Failed to bind to address.");  // If it fails, print this message

    let pool = thread_pool::ThreadPool::new(8);  // Create a thread pool with 8 threads
    // Iterate over the incoming connections
    listener.incoming().for_each(|stream| {
        match stream {  // Match the stream (connection)
            Ok(stream) => pool.execute(|| handle_client(stream)),
            Err(e) => eprintln!("Error: {}", e),
        }
    });

}


fn handle_client(mut stream: TcpStream)  {
    // * Buffer reads the data from the stream and stores it in the buffer
    let mut buffer = [0; 1024];  // 1 KB buffer (1024 bytes)
    // let mut buffer = [0; 32768];  // 32 KB buffer (32768 bytes)
    // let mut buffer = [0; 131072];  // 128 KB buffer (131072 bytes)

    match stream.read(&mut buffer) {  // Read the data from the stream and store it in the buffer
        Ok(size) => {  // If the read was successful
            // log::info!("({} Bytes):\n{} ", size, String::from_utf8_lossy(&buffer[..size]));

            let request_line = String::from_utf8_lossy(&buffer[..size])
                .lines()  // Split the string into lines
                .next()  // Get the first line
                .unwrap()  // Unwrap the Option
                .to_owned();  // Convert the &str to String
            // println!("Request line: {:?}", request_line);

            //  ^ Avoid the favicon request (a browser will always request the favicon)
            match request_line.split_whitespace().nth(1) == Some("/favicon.ico") {
                true => {  // If the request is for the favicon, return (close the connection)
                    log::warn!("Favicon requested. POSSIBLY IS A Repeated request?");
                    return
                },
                false => {  // * If the request is not for the /favicon.ico, then continue
                    log::trace!("New connection from {}", stream.peer_addr().unwrap());
                }
            }

            // * Same as above, but with map_or_else
            // request_line.split_whitespace().nth(1).map_or_else(
            //     || log::warn!("Invalid request line: {}", request_line),
            //     |url| {
            //         if url == "/favicon.ico" {
            //             log::warn!("Favicon requested. POSSIBLY IS A Repeated request?");
            //             return
            //         } else {
            //             log::trace!("New connection from {}", stream.peer_addr().unwrap());
            //         }
            //     }
            // );


            // * Manage Response --------------------------------------------------------------
            // If the request is not for the favicon, then match the request line...
            match match_request_line(request_line) {
                Some(mut response) => {
                    // log::info!("Response: {}", response.to_string());
                    stream.write(response.to_string().as_bytes()).unwrap();
                },
                None => {
                    log::error!("Invalid request line.");
                    // stream.write("Invalid request line.".as_bytes()).unwrap();
                },
            };

           
            // * To be sure that the response is sent, flush the stream
            // match stream.flush() {  // Flush the stream to make sure that the response is sent
            //     Ok(_) => log::trace!("Response sent."),  // If the flush was successful
            //     Err(e) => log::error!("Failed to flush the stream: {}", e),
            // }
        }
        Err(e) => {
            log::error!("Failed to read from connection: {}", e);
        },
    }

}


// /// Close the connection.
// /// 
// /// This function is not used because the connection is closed by the client.
// /// 
// /// This function is only used for testing.
// /// 
// /// # Arguments
// /// 
// /// - `stream` - The TcpStream to close.
// fn close_connection(mut stream: TcpStream) {
//     log::warn!("Closing connection.");
//     stream.write_all("Closing connection.".as_bytes()).unwrap();  // send a message to the client
//     stream.shutdown(std::net::Shutdown::Both).unwrap();  // close the connection (both ways)
//     std::process::exit(0);  // finalize the program
// }


fn match_request_line(request_line: String) -> Option<HttpResponse> {
    let mut parts = request_line.split_whitespace();

    let (method, url, http_version) = (
        HttpMethod::from_str(parts.next().unwrap()), 
        parts.next(),
        HttpVersion::from_str(parts.next().unwrap())
    );

    // * If at least 1 of the values is None, then return None
    if method.is_none() || url.is_none() || http_version.is_none() {
        log::error!("Invalid request line: {}", request_line);
        return None;
    } else {
        log::debug!("Method: {:?}", method.unwrap());
        log::debug!("Url: {}", url.unwrap());
        log::debug!("Http version: {}", http_version.unwrap());
        return Some(handle_service(url.unwrap()));  // If the request line is valid, return the response
    }

}


// todo: Impl a better routing system
// options for routing:
// - match the url with a regex
// - match the url with a hashmap
// - match the url with a match statement (same as now) ... but better
fn handle_service(url: impl Into<String> + Display + Copy) -> HttpResponse { 
    let mut html = std::fs::read_to_string("resources\\html\\index.html").unwrap();
    match url.into().as_str() {
        "/" => {
            log::info!("Home page");
            
            HttpResponse::new(HttpStatus::_200, HttpVersion::Http1_1, html
                .replace("{TITLE}", "OK")
                .replace("{CODE}" , "200")
                .replace("{STATUS}", "OK")
                .replace("{MESSAGE}", "Everything is OK"),
            )
        },
        _ => {
            log::warn!("501 Not Implemented ({}).", url);
            HttpResponse::new(
                HttpStatus::_501,  // 501 Not Implemented
                HttpVersion::Http1_1,
                "".to_string()
            )
        },
    }
}
