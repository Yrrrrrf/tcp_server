//! # Rust TCP Server
//! 
//! This is a simple TCP server written in Rust.
//! 
//! Handles multiple connections using threads.
//! 
#![allow(unused)]

// ? Module imports -----------------------------------------------------------------------------------------------------------

// Standard library imports
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};
use std::thread;

// External crates
use log::{LevelFilter, warn, info, debug, trace, error};
use dev_utils::log::rlog::RLog;
use dev_utils::print_app_data;

// Own modules
pub mod http_status;
use http_status::*;
pub mod http_response;
use http_response::*;
// mod config;
// use config::*;


// ? Main ---------------------------------------------------------------------------------------------------------------------
// async fn main() -> Result<(), Box<dyn std::error::Error>> {  // for async main (async-std)
fn main() {
    print_app_data(file!());  // Read the Cargo.toml file and print the app data (name, version, authors)
    // RLog::init_logger(LevelFilter::Info);  // Initialize the logger with the given log level
    RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    // ^ Actual code: --------------------------------------------------------------------------------------------------------
    // let server_address = format!("{}:{}", config::SERVER_IP, config::SERVER_PORT);
    let server_address = "127.0.0.1:8080";
    log::info!("Starting server at {}\n", server_address);
    let listener = TcpListener::bind(server_address)  // Listen on this address
        .expect("Failed to bind to address.");  // If it fails, print this message

    let mut request_count = 0;

    // Iterate over the incoming connections
    listener.incoming().for_each(|stream| {
        match stream {
            Ok(stream) => {
                match request_count {
                    // 0..=4 => {
                    //     thread::spawn(|| {handle_client(stream);});
                    //     request_count += 1;
                    // },
                    _ => {thread::spawn(|| {handle_client(stream);});},
                }

            }
            Err(e) => eprintln!("Error: {}", e),
        }
    });

}


fn handle_client(mut stream: TcpStream) {
    log::trace!("New connection from {}", stream.peer_addr().unwrap());
    // * Buffer reads the data from the stream and stores it in the buffer
    let mut buffer = [0; 1024];  // 1 KB buffer (1024 bytes)
    // let mut buffer = [0; 32768];  // 32 KB buffer (32768 bytes)
    // let mut buffer = [0; 131072];  // 128 KB buffer (131072 bytes)

    match stream.read(&mut buffer) {  // Read the data from the stream and store it in the buffer
        Ok(size) => {  // If the read was successful

            // * Manage Request ---------------------------------------------------------------
            // log::info!("({} Bytes):\n{} ", size, String::from_utf8_lossy(&buffer[..size]));


            // get the first line of the request
            let request_line = String::from_utf8_lossy(&buffer[..size]).lines().next().unwrap().to_owned();
            println!("Request line: {}", request_line);

            // match the HTTP method
            let method = match request_line.split_whitespace().next() {
                Some(method) => method,
                None => {
                    log::error!("Invalid request line: {}", request_line);
                    return;
                }
            };

            match method {
                "GET" => log::info!("GET request"),
                "POST" => log::info!("POST request"),
                "PUT" => log::info!("PUT request"),
                "DELETE" => log::info!("DELETE request"),
                // "CONNECT" => log::info!("CONNECT request"),
                _ => log::error!("Invalid method: {}", method)
            }

            // match the URL
            let url = match request_line.split_whitespace().nth(1) {
                Some(url) => url,
                None => {
                    log::error!("Invalid request line: {}", request_line);
                    return;
                }
            };

            match url {
                "/" => log::info!("Home page"),
                "/about" => log::info!("About page"),
                "/contact" => log::info!("Contact page"),
                _ => log::error!("Invalid URL: {}", url)
            }

            // match the HTTP version
            let http_version = match request_line.split_whitespace().nth(2) {
                Some(http_version) => http_version,
                None => {
                    log::error!("Invalid request line: {}", request_line);
                    return;
                }
            };

            match http_version {
                // "HTTP/1.0" => log::info!("HTTP/1.0"),
                "HTTP/1.1" => log::info!("HTTP/1.1"),
                "HTTP/2.0" => log::info!("HTTP/2.0"),
                _ => log::error!("Invalid HTTP version: {}", http_version)
            }




            // todo: Check if the match is necessary to close the connection
            // todo: Probably not, because the connection is closed when the client closes it
            // todo: It should be only used when the server wants to close the connection? Or not?
            // match size {
            //     5 => { if String::from_utf8_lossy(&buffer[..size]) == "close" {
            //             log::warn!("Closing connection.");
            //             stream.write_all("Closing connection.".as_bytes()).unwrap();  // send a message to the client
            //             stream.shutdown(std::net::Shutdown::Both).unwrap();  // close the connection (both ways)
            //             std::process::exit(0);  // finalize the program
            //         }}
            //     _ => {},
            // }

            // * Echo server (GENERATE THE RESPONSE TO THE CLIENT)
            // Create a message from the buffer and send it back to the client
            // let message = String::from_utf8_lossy(&buffer[..size]);
            // stream.write_all(message.chars().rev().collect::<String>().as_bytes()).unwrap();

            // * Manage Response --------------------------------------------------------------
            let response = HttpResponse::new(
                http_status::HttpStatus::_200,
                http_response::HttpVersion::Http1_1,
                std::fs::read_to_string("resources\\html\\200.html").unwrap()  // Contents of the file
            ).to_string();

            stream.write(response.as_bytes()).unwrap();  // Write the 'response' as bytes to the client's connection.
            stream.flush().unwrap();  // Flush the stream to ensure data is sent to the client.

        }
        Err(e) => log::error!("Failed to read from connection: {}", e),
    }

    // Read the HTTP request from the client and store it in 'request'.
    // let mut request = String::new();
    // stream.read_to_string(&mut request).unwrap();

    // // Check if the request starts with "GET /file/". If so, extract the filename from the request.
    // let response = if request.starts_with("GET /file/") {
    //     let filename = &request[10..].split_whitespace().next().unwrap();
    //     // Try to read the file's contents. If successful, create an HTTP response with a 200 OK status.
    //     match std::fs::read_to_string(filename) {
    //         Ok(contents) => format!("HTTP/1.1 200 OK\r\n\r\n{}", contents),
    //         // If the file is not found, create an HTTP response with a 404 Not Found status.
    //         Err(_) => "HTTP/1.1 404 Not Found\r\n\r\nFile not found".to_string(),
    //     }
    // } else {
    //     // If the request is not valid (e.g., doesn't start with "GET /file/"), create a 400 Bad Request response.
    //     "HTTP/1.1 400 Bad Request\r\n\r\nInvalid request".to_string()
    // };

    // // Write the 'response' as bytes to the client's connection.
    // stream.write(response.as_bytes()).unwrap();
    // // Flush the stream to ensure data is sent to the client.
    // stream.flush().unwrap();
}

