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
            // empty string
            // let message = "\n";
            let message = b"Hello, Rust!";
            let message = b"0123456789";
            let message = b"GET / HTTP/1.1\r\nHost: www.rust-lang.org\r\n\r\n";
            // let message = "close";
            stream.write(message).unwrap();
            // stream.write(message.as_bytes()).unwrap();  // Write the message to the stream (same as b"&str")

            let mut buffer = [0; 1024];
            let bytes = stream.read(&mut buffer).unwrap();
            let response = String::from_utf8_lossy(&buffer[0..bytes]);
            println!("Response: {}", response);
        },
        Err(e) => eprintln!("Failed to connect: {}", e),
    }

}
