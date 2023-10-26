//! # Rust TCP Server
//! 
//! This is a simple TCP server written in Rust.
//! 
//! Handles multiple connections using threads.
//! 
#![allow(dead_code)]
#![allow(unused)]

// ? Module imports -----------------------------------------------------------------------------------------------------------

// Standard library imports
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};
use std::thread;

// External crates
use log::{LevelFilter, warn, info, debug, trace, error};
use dev_utils::*;  // ^ My own crate :)

// Own modules
mod config;
use config::*;


// ? Main ---------------------------------------------------------------------------------------------------------------------
// async fn main() -> Result<(), Box<dyn std::error::Error>> {  // for async main (async-std)
fn main() {
    dev_utils::print_app_data();
    rlog::RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    // ^ Actual code: --------------------------------------------------------------------------------------------------------

    let listener = TcpListener::bind(format!("{}:{}", SERVER_IP, SERVER_PORT)).expect("Failed to bind.");

    // Iterate over the incoming connections
    listener.incoming().for_each(|stream| {
        match stream {
            Ok(stream) => {thread::spawn(|| {handle_client(stream);});}
            Err(e) => eprintln!("Error: {}", e),
        }
    });

}


fn handle_client(mut stream: TcpStream) {
    println!("\nNew connection: {:?}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];  // 1 KB buffer (1024 bytes)
    // * Buffer reads the data from the stream and stores it in the buffer

    match stream.read(&mut buffer) {
        Ok(size) => {
            match size {
                0 => println!("Client disconnected."),
                // 10 => println!("Received 10 bytes."),
                _ => {
                    let message = String::from_utf8_lossy(&buffer[..size]);
                    println!("Received message from client: {}", message);

                    let reversed_message = message.chars().rev().collect::<String>();
                    stream.write_all(reversed_message.as_bytes()).unwrap();
                }
            }
        }
        Err(e) => println!("Failed to read from clieht: {}", e)
    }

}
