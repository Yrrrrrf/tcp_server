//! # TCP Client
//! 
#![allow(dead_code)]
#![allow(unused)]

// ? Module imports -----------------------------------------------------------------------------------------------------------


// Standard library imports
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;

// External crates
use log::{LevelFilter, warn, info, debug, trace, error};
use dev_utils::log::rlog::RLog;
use dev_utils::print_app_data;

// Own modules
pub mod http_methods;
pub mod http_request;
use http_request::*;

use crate::http_methods::HttpMethod;
// mod config;
// use config::*;

// ? Main ---------------------------------------------------------------------------------------------------------------------
fn main() {
    print_app_data(file!());  // Read the Cargo.toml file and print the app data (name, version, authors)
    RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    // ^ Actual code: --------------------------------------------------------------------------------------------------------
    // let server_address = format!("{}:{}", config::SERVER_IP, config::SERVER_PORT);
    let server_address = "127.0.0.1:8080";
    log::info!("Starting client to {}\n", server_address);

    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            let message = b"Hello, Rust!";

            let request = Request::new_1_1(
                HttpMethod::POST, 
                // "/index.html".to_string(), 
                "/", 
                message.iter().map(|&x| x as char).collect::<String>(),
            );
            println!("{}", request);
            
            
            stream.write(request.to_string().as_bytes()).unwrap();
            // stream.write(request.body.as_bytes()).unwrap();



            let mut buffer = [0; 1024];
            let bytes = stream.read(&mut buffer).unwrap();
            let response = String::from_utf8_lossy(&buffer[0..bytes]);
            // println!("Response: {}", response);
        },
        Err(e) => eprintln!("Failed to connect: {}", e),
    }

}


// Create the client struct. The client must send a request to the server and receive a response.
// The client must be able to send a request with a body and receive a response with a body.
