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
#![allow(unused)]


// ? Module imports -----------------------------------------------------------------------------------------------------------

// Standard library imports
use std::io::{Read, Write, Error};
use std::net::{TcpStream, TcpListener};
use std::path::Path;

// External crates
use log::LevelFilter;
use dev_utils::{
    print_app_data,
    log::rlog::RLog,
    http::{*, response::HttpResponse},
    files::{
        toml::TomlFile,
        crud,
    },
};


/// Handles a client connection by reading the request, checking for favicon requests, and generating a response.
///
/// # Arguments
///
/// - `stream`: The `TcpStream` representing the client connection.
pub fn handle_client(mut stream: TcpStream)  {
    // * Buffer reads the data from the stream and stores it in the buffer
    const BUFFER_SIZE: usize = 32;  // 32 KB
    let mut buffer = [0; 1024*BUFFER_SIZE];

    match stream.read(&mut buffer) {  // Read the data from the stream and store it in the buffer
        Ok(size) => {  // If the read was successful
            let buffer_str = String::from_utf8_lossy(&buffer[..size]);  // Convert the buffer to a String
            let lines: Vec<&str> = buffer_str.lines().collect();  // Split the String into lines

            let request_line = lines.get(0).unwrap_or(&"").to_string();  // Get the first line (the request line)
            let body = lines[1..].join("\n");  // Get the body (the rest of the lines)
            // println!("Request line: {}", request_line);
            // println!("Body: {}", body);

            //^ Avoid the favicon request (a browser will always request the favicon)
            match request_line.split_whitespace().nth(1) == Some("/favicon.ico") {
                true => {log::warn!("Favicon requested. POSSIBLY IS A Repeated request?");
                    return  // If the request is for the favicon, return (close the connection)
                }
                false =>  log::info!("New connection from {}", stream.peer_addr().unwrap()),
            }

            // * Manage Response --------------------------------------------------------------
            match match_request_line(&request_line) {
                Some((method, url, http_version)) => {
                    // log::trace!("Http v: {}", http_version);
                    // log::trace!("Method: {:?}", method);
                    // log::trace!("Url: {}", url);

                    let response = match_method_and_url(method, url, &body);
                    stream.write(response.to_string().as_bytes()).unwrap();
                    log::debug!("Response sent.");
                },
                None => {  // Handle a bad request line
                    log::error!("Invalid request line: {}", request_line);
                    stream.write(
                        // A web browser will never send an invalid request line but a client can.
                        // For example, a client can send: "GET / HTTP/2.4"
                        // That's why this do not include a html file as body. It is just a plain text.
                    HttpResponse::new(HttpStatus::_400, HttpVersion::Http1_1, "Invalid").to_string().as_bytes()
                    ).unwrap();
                },
            };

        }
        Err(e) => log::error!("Failed to read from connection: {}", e),
    }

}

/// Matches and parses the components of an HTTP request line. Then, it generates an appropriate response.
///
/// This function extracts the HTTP method, URL, and HTTP version from the request line. If any of these
/// components is missing, it logs an error and returns `None`. Otherwise, it generates a response based
/// on the parsed components.
///
/// # Arguments
///
/// - `request_line`: The HTTP request line to parse.
///
/// # Returns
///
/// An `Option` containing the parsed components of the request line.
fn match_request_line(request_line: &String) -> Option<(HttpMethod, &str, HttpVersion)> {
    let mut parts = request_line.split_whitespace();
    // the order of the parts is important because the iterator is consumed
    Some((  // parse the request line (method, url, http_version)
        HttpMethod::from_str(parts.next()?)?,  // the method is a HttpMethod enum
        parts.next()?,  // the url is a String
        HttpVersion::from_str(parts.next()?)?  // the http_version is a HttpVersion enum
    ))  // return the tuple (method, url, http_version)
}

fn match_method_and_url(method: HttpMethod, url: &str, body: &str)  -> HttpResponse {
    let path = ".\\resources\\temp\\";  // the path where the files will be created
    let filename = format!("{}.txt", url.trim_start_matches('/'));

    //^ This can look a little bit weird but it is just a match inside a match .____.
    match match method {  //* Match the request method to perform the appropriate operation
        HttpMethod::POST => crud::create_file(&path, &filename, body),
        HttpMethod::GET => crud::read_file(&path, &filename),
        // HttpMethod::PUT => crud::update_file(&path, &filename, body),
        HttpMethod::PUT => crud::append_file(&path, &filename, body),
        HttpMethod::DELETE => crud::delete_file(&path, &filename),
        _ => {
            log::error!("Method not implemented: {:?}", method);
            return generate_response("Unknown Http Method Request", HttpStatus::_501)
            // 501 Not Implemented
        },
    } {  //* Match the result of the CRUD operation
        Ok(ok) => {
            log::info!("{}", &ok.as_str());
            log::trace!("Generating response for {}", url);
            // println!("File content: {}", std::fs::read_to_string(format!("{}{}", path, filename)).unwrap());

            match url {
                "/" => generate_response(HttpStatus::_200.message(), HttpStatus::_200),
                "/about" => generate_response(&ok, HttpStatus::_201),
                "/contact" => generate_response(&ok, HttpStatus::_501),

                // ^ Add more routes here...
                _ => generate_response("Unknown Service", HttpStatus::_503),  // 503 Service Unavailable
            }
        },
        Err(e) => {
            log::error!("{}", &e);
            generate_response(&e.to_string(), HttpStatus::_502)  // 502 Bad Gateway
        },
    }
}

/// Generates an HTTP response for a given message and status code.
///
/// This function reads an HTML file, replaces placeholders with specific values, and constructs
/// an `HttpResponse` with the provided status code and modified HTML content.
///
/// # Arguments
///
/// - `message`: The message to include in the response.
/// - `status`: The HTTP status code for the response.
///
/// # Returns
///
/// An `HttpResponse` representing the server's response.
fn generate_response(message: &str, status: HttpStatus) -> HttpResponse {
    HttpResponse::new(
        status.clone(), 
        HttpVersion::Http1_1, 
        std::fs::read_to_string("resources\\html\\index.html").unwrap()
            .replace("{TITLE}", status.message())
            .replace("{CODE}", &status.code().to_string())
            .replace("{STATUS}", &status.message())
            .replace("{MESSAGE}", message)
    )
}
