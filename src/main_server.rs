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
use dev_utils::log::rlog::RLog;
use dev_utils::print_app_data;

// Own modules
mod config;
use config::*;


// ? Main ---------------------------------------------------------------------------------------------------------------------
// async fn main() -> Result<(), Box<dyn std::error::Error>> {  // for async main (async-std)
fn main() {
    print_app_data(file!());  // Read the Cargo.toml file and print the app data (name, version, authors)
    // RLog::init_logger(LevelFilter::Info);  // Initialize the logger with the given log level
    RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    // ^ Actual code: --------------------------------------------------------------------------------------------------------
    let server_address = format!("{}:{}", config::SERVER_IP, config::SERVER_PORT);
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
            // log::info!("Received message ({}B):\n{} ", size, String::from_utf8_lossy(&buffer[..size]));


            if buffer.starts_with(b"GET / HTTP/1.1") {
                log::info!("Received HTTP request ({}B):\n{} ", size, String::from_utf8_lossy(&buffer[..size]));
            } else {
                log::info!("Received message ({}B):\n{} ", size, String::from_utf8_lossy(&buffer[..size]));
            }


            // todo: Check if the match is necessary to close the connection
            // todo: Probably not, because the connection is closed when the client closes it
            // todo: It should be only used when the server wants to close the connection? Or not?
            match size {
                5 => { if String::from_utf8_lossy(&buffer[..size]) == "close" {
                        log::warn!("Closing connection.");
                        stream.write_all("Closing connection.".as_bytes()).unwrap();  // send a message to the client
                        stream.shutdown(std::net::Shutdown::Both).unwrap();  // close the connection (both ways)
                        std::process::exit(0);  // finalize the program
                    }}
                _ => {},
            }

            // * Echo server (GENERATE THE RESPONSE TO THE CLIENT)
            // Create a message from the buffer and send it back to the client
            // let message = String::from_utf8_lossy(&buffer[..size]);
            // stream.write_all(message.chars().rev().collect::<String>().as_bytes()).unwrap();

            let contents = std::fs::read_to_string("html\\200.html").unwrap();  
            let response = "HTTP/1.1 200 OK\r\n\r\n".to_string() + &contents;

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

