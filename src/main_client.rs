//! # TCP Client
//! 
#![allow(dead_code)]
#![allow(unused)]

// ? Module imports -----------------------------------------------------------------------------------------------------------

// Standard library imports
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

// External crates
use log::{LevelFilter, warn, info, debug, trace, error};
use dev_utils::*;  // ^ My own crate :)

// Own modules
mod config;
use config::*;

// ? Main ---------------------------------------------------------------------------------------------------------------------
fn main() {
    dev_utils::print_app_data();
    rlog::RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    // ^ Actual code: --------------------------------------------------------------------------------------------------------

    // Create a TCP stream and connect to the server
    let mut stream = TcpStream::connect(format!("{}:{}", config::SERVER_IP, config::SERVER_PORT))
        .expect("Failed to connect to server.");

    // Send a message to the server
    let message = "Hello, Rust!";
    let message = "0000000000";
    stream.write_all(message.as_bytes()).expect("Failed to send data.");

    // Receive and print the connection confirmation from the server
    // let mut confirmation = String::new();
    // stream.read_to_string(&mut confirmation).expect("Failed to read confirmation");
    // println!("Confirmation from server: {}", confirmation);

    // Sleep for 10 seconds

    // let mut response = String::new();
    // stream.read_to_string(&mut response).expect("Failed to read response from server.");
    // println!("Received from server: {}", response);

}
