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
use std::io::{Read, Write};
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

// Internal modules
mod thread_pool;

// ? Main ---------------------------------------------------------------------------------------------------------------------
// async fn main() -> Result<(), Box<dyn std::error::Error>> {  // for async main (async-std)
fn main() {
    print_app_data(file!());  // Read the Cargo.toml file and print the app data (name, version, authors)
    // RLog::init_logger(LevelFilter::Info);  // Initialize the logger with the given log level
    RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    // ^ Read the IP address and port from the keys.toml file
    let keys = TomlFile::new(Path::new("resources/keys/keys.toml"));
    let section = keys.get_section_data("ip_address");
    let ip = section.unwrap().get_key_value("ip").unwrap();
    let port = section.unwrap().get_key_value("port").unwrap();

    let server_address = format!("{}:{}", ip.1, port.1);
    log::info!("Starting server at {}\n", server_address);

    let listener = TcpListener::bind(server_address).expect("Failed to bind to address.");  // If it fails, print this message
    let pool = thread_pool::ThreadPool::new(8);  // Create a thread pool with 8 threads
    // Iterate over the incoming connections
    for stream in listener.incoming() {
    // for stream in listener.incoming().take(3) {
        match stream {
            Ok(stream) => pool.execute(|| handle_client(stream)),
            Err(e) => log::error!("Failed to establish a connection: {}", e),
        }
    }
    log::info!("Shutting down.");  // If the loop ends, print this message (the server is shutting down)
}


/// Handles a client connection by reading the request, checking for favicon requests, and generating a response.
///
/// # Arguments
///
/// - `stream`: The `TcpStream` representing the client connection.
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

            // get the body (the rest of the request) as a &str
            let body = String::from_utf8_lossy(&buffer[..size])
                .lines()  // Split the string into lines
                .skip(1)  // Skip the first line (the request line)
                .collect::<Vec<&str>>()  // Collect the lines into a vector of &str
                .join("\n");  // Join the lines with a newline character

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
                    match_method_and_url(method, url, &body);
                    
                    let response = handle_service(url);
                    stream.write(response.to_string().as_bytes()).unwrap();
                    log::debug!("Response sent.");
                },
                None => {  // Handle a bad request line
                    log::error!("Invalid request line: {}", request_line);
                    stream.write(  // A web browser will never send an invalid request line but a client can.
                        // For example, a client can send: "GET / HTTP/2.4"
                        // That's why this do not include a html file as body. It is just a plain text.
                    HttpResponse::new(HttpStatus::_400, HttpVersion::Http1_1, "Invalid ".to_string())
                        .to_string().as_bytes()).unwrap();
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


// todo: rename this method
// todo: extract the url validation (for file serving)
fn match_method_and_url(method: HttpMethod, url: &str, body: &str) {
    let path = ".\\resources\\temp\\";  // the path where the files will be created
    let filename = format!("{}.txt", url.trim_start_matches('/'));

    //^ This can look a little bit weird but it is just a match inside a match .____.
    match match method {  //* Match the request method to perform the appropriate operation
        HttpMethod::POST => crud::create_file(&path, &filename, body),
        HttpMethod::GET => crud::read_file(&path, &filename),
        HttpMethod::PUT => crud::update_file(&path, &filename, body),
        HttpMethod::DELETE => crud::delete_file(&path, &filename),
    } {  //* Match the result of the CRUD operation
        Ok(ok) => log::info!("{ok}"),
        Err(e) => log::error!("{e}"),
    };
}


// ? Modified functions (to add behavior depending on the URL & request method) -----------------------------------------------


/// Handles a service based on the provided URL and generates an appropriate HTTP response.
///
/// This function implements a basic routing system where specific URLs are matched to generate custom responses.
/// It logs debugging information and returns an `HttpResponse` based on the matched URL.
///
/// # Arguments
///
/// - `url`: The URL to handle.
///
/// # Returns
///
/// An `HttpResponse` representing the server's response.
fn handle_service(url: &str) -> HttpResponse { 
    log::debug!("Generating response for {}", url);
    match url {
        "/" => generate_response("/", HttpStatus::_200),
        "/about" => generate_response("Not impl yet", HttpStatus::_501),
        "/contact" => generate_response("Not impl yet", HttpStatus::_501),
        // 
        _ => generate_response("Unknown Service", HttpStatus::_404),
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
