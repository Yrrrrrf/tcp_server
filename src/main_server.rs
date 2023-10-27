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
use dev_utils::{rlog, terminal};  // ^ My own crate :)

// Own modules
mod config;
use config::*;


// ? Main ---------------------------------------------------------------------------------------------------------------------
// async fn main() -> Result<(), Box<dyn std::error::Error>> {  // for async main (async-std)
fn main() {
    print_app_data();
    rlog::RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    // ^ Actual code: --------------------------------------------------------------------------------------------------------

    let server_address = format!("{}:{}", config::SERVER_IP, config::SERVER_PORT);
    println!("Server address: {}", server_address);

    let listener = TcpListener::bind(server_address)  // Listen on this address
        .expect("Failed to bind to address.");  // If it fails, print this message

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










pub fn print_app_data() {
    // Imports for reading the Cargo.toml file
    use std::fs;
    use std::io::{self, BufRead};
    use std::path::Path;

    let mut name = String::new();
    let mut version = String::new();
    let mut authors = String::new();

    print!("{}[2J{}[1;1H", 27 as char, 27 as char);  // Clear the terminal
    match fs::File::open(&Path::new("Cargo.toml")) {
        Ok(file) => {
            let mut lines = io::BufReader::new(file).lines().peekable();
            while let Some(Ok(line)) = lines.next() {  // While there are still lines to read
                if line.starts_with("[package]") {  // If the line starts with "[package]"
                    while let Some(Ok(line)) = lines.next() {
                        match line {
                            _ if line.starts_with("name =") => name = terminal::set_fg(&line.split('=').nth(1).unwrap().trim().replace("\"", "").to_uppercase(), 'g'),
                            _ if line.starts_with("version =") => version = terminal::set_fg(&format!("V{}", line.split('=').nth(1).unwrap().trim().replace("\"", "")), 'b'),
                            _ if line.starts_with("authors =") => authors = line.split('=').nth(1).unwrap().trim().replace("[\"", "").replace("\"]", ""),
                            _ if line.starts_with("[") => break,  // Stop when the next section starts
                            _ => (),  // Else do nothing
                        }
                    }
                    break;
                }
            }
            println!("{} {}\nAuthors: {}\n", name, version, authors);
        }
        Err(e) => eprintln!("Failed to open 'Cargo.toml': {}", e),
    };
}
