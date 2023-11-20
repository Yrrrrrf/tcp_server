//! # TCP Client
//! 
#![allow(dead_code)]
#![allow(unused)]

// ? Module imports -----------------------------------------------------------------------------------------------------------

// Standard library imports
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::thread;

// External crates
use log::{LevelFilter, warn, info, debug, trace, error};
use dev_utils::log::rlog::RLog;
use dev_utils::print_app_data;

use dev_utils::http::{*, request::HttpRequest};


// ? Main ---------------------------------------------------------------------------------------------------------------------
fn main() {
    print_app_data(file!());  // Read the Cargo.toml file and print the app data (name, version, authors)
    RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    // ^ Read the IP address and port from the keys.toml file
    let keys = dev_utils::files::toml::TomlFile::new(Path::new(".\\resources\\keys\\keys.toml"));
    let section = keys.get_section_data("ip_address");
    let ip = section.unwrap().get_key_value("ip").unwrap();
    let port = section.unwrap().get_key_value("port").unwrap();

    // ^ Actual code: --------------------------------------------------------------------------------------------------------
    let server_address = format!("{}:{}", ip.1, port.1);
    log::info!("Starting client to {}\n", server_address);

    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            let request = HttpRequest::new(
                // HttpMethod::GET, 
                HttpMethod::POST, 
                HttpVersion::Http1_1,
                // "/",
                "/about",
                // "yes",
                "Hello, Rust!".to_string()
            );
            // let request = "GET / HTTP/1.1".to_string();  // Valid HTTP request
            // let request = "GET / HTTP/2.4".to_string();  // Invalid HTTP version (505)
            // let request = "GET /fiveOfive.html HTTP/1.1".to_string();  // Not Implemented (501)

            stream.write(request.to_string().as_bytes()).unwrap();

            let mut buffer = [0; 1024];
            let bytes = stream.read(&mut buffer).unwrap();
            let response = String::from_utf8_lossy(&buffer[0..bytes]);
            println!("Response: {}", response);
        },
        Err(e) => eprintln!("Failed to connect: {}", e),
    }

}

// Create the client struct. The client must send a request to the server and receive a response.
// The client must be able to send a Request and receive a Response