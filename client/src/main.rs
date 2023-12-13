//! # TCP Client
//! 
#![allow(unused)]

// ? Module imports -----------------------------------------------------------------------------------------------------------

// Standard library imports
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;

// External crates
use log::LevelFilter;
use dev_utils::{
    print_app_data,
    log::rlog::RLog,
    http::{
        *, 
        request::HttpRequest
    },
    files::toml::TomlFile,
};

// ? Main ---------------------------------------------------------------------------------------------------------------------
fn main() {
    print_app_data(file!());  // Read the Cargo.toml file and print the app data (name, version, authors)
    RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    // ^ Read the IP address and port from the keys.toml file
    let keys = TomlFile::new(Path::new("resources/keys/keys.toml"));
    let section = keys.get_section_data("ip_address");
    let ip = section.unwrap().get_key_value("ip").unwrap();
    let port = section.unwrap().get_key_value("port").unwrap();

    // ^ Actual code: --------------------------------------------------------------------------------------------------------
    let server_address = format!("{}:{}", ip.1, port.1);
    log::info!("Starting client to {}\n", server_address);

    match TcpStream::connect(server_address) {
        Ok(mut stream) => {  // If the connection is successful
            let request = HttpRequest::new(
                // HttpMethod::POST, 
                // HttpMethod::GET,
                HttpMethod::PUT,
                // HttpMethod::DELETE, 
                HttpVersion::Http1_1,
                // "/",
                "/about",
                // "yes",
                "Hello, Rust!",
            );
            // let request = "GET / HTTP/1.1";  // Valid HTTP request
            // let request = "GET / HTTP/2.4";  // Invalid HTTP version (505)
            // let request = "GET /fiveOfive.html HTTP/1.1";  // Not Implemented (501)

            stream.write(request.to_string().as_bytes()).unwrap();

            let mut buffer = [0; 1024];
            let bytes = stream.read(&mut buffer).unwrap();
            let response = String::from_utf8_lossy(&buffer[0..bytes]);
            // println!("Response: {}", response);

            // Parse the response
            // let response = HttpResponse::parse_response(response.to_string());

            // * Handle the response headers
            // todo: Handle the response headers

            // * Handle the response body
            // let the body be the section after the headers ON <!DOCTYPE html>
            let body = response.split("<!DOCTYPE html>").collect::<Vec<&str>>()[1];
            println!("Body: {}", body);


        },
        Err(e) => eprintln!("Failed to connect: {}", e),
    }

}

// Create the client struct. The client must send a request to the server and receive a response.
// The client must be able to send a Request and receive a Response