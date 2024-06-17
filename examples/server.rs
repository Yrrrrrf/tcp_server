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

use tcp_server::{
    server::*,
    thread_pool::ThreadPool
};


// ? Main ---------------------------------------------------------------------------------------------------------------------
fn main() {
    // print_app_data(file!());  // Read the Cargo.toml file and print the app data (name, version, authors)
    RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    // ^ Read the IP address and port from the keys.toml file
    let keys = TomlFile::new(Path::new("resources/keys/keys.toml"));
    let section = keys.get_section_data("ip_address");
    let ip = section.unwrap().get_key_value("ip").unwrap();
    let port = section.unwrap().get_key_value("port").unwrap();

    let server_address = format!("{}:{}", ip.1, port.1);
    log::info!("Starting server at {}\n", server_address);

    let pool = ThreadPool::new(8);  // Create a thread pool with 8 threads
    TcpListener::bind(server_address).expect("Failed to bind to address.").incoming().for_each(|stream| {
        match stream {
            Ok(stream) => pool.execute(|| handle_client(stream)),
            Err(e) => log::error!("Failed to establish a connection: {}", e),
        }
    });
    log::warn!("Shutting down.");  // If the loop ends, print this message (the server is shutting down)
}
