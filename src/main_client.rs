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
    print_app_data();
    rlog::RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    // ^ Actual code: --------------------------------------------------------------------------------------------------------

    let server_address = format!("{}:{}", config::SERVER_IP, config::SERVER_PORT);
    println!("Server address: {}", server_address);

    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            let message = "Hello, Rust!";
            stream.write(message.as_bytes()).unwrap();

            let mut buffer = [0; 1024];
            let bytes = stream.read(&mut buffer).unwrap();
            let response = String::from_utf8_lossy(&buffer[0..bytes]);
            println!("Received: {}", response);
        },
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
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
