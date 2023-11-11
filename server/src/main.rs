//! # Rust TCP Server
//! 
//! This is a simple TCP server written in Rust.
//! 
//! Handles multiple connections using threads.
//! 
#![allow(unused)]

// ? Module imports -----------------------------------------------------------------------------------------------------------

// Standard library imports
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};
use std::path::Path;

// External crates
use log::LevelFilter;
use dev_utils::log::rlog::RLog;
use dev_utils::print_app_data;

use dev_utils::http::{*, request::HttpRequest, response::HttpResponse};

mod thread_pool;

// ? Main ---------------------------------------------------------------------------------------------------------------------
// async fn main() -> Result<(), Box<dyn std::error::Error>> {  // for async main (async-std)
fn main() {
    print_app_data(file!());  // Read the Cargo.toml file and print the app data (name, version, authors)
    // RLog::init_logger(LevelFilter::Info);  // Initialize the logger with the given log level
    RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    // ^ Read the IP address and port from the keys.toml file
    let keys = dev_utils::files::toml::TomlFile::new(Path::new("resources/keys/keys.toml"));
    let section = keys.get_section_data("ip_address");
    let ip = section.unwrap().get_key_value("ip").unwrap();
    let port = section.unwrap().get_key_value("port").unwrap();

    // ^ Actual code: --------------------------------------------------------------------------------------------------------
    let server_address = format!("{}:{}", ip.1, port.1);
    log::info!("Starting server at {}\n", server_address);

    let listener = TcpListener::bind(server_address)  // Listen on this address
        .expect("Failed to bind to address.");  // If it fails, print this message

    let pool = thread_pool::ThreadPool::new(8);  // Create a thread pool with 8 threads
    // Iterate over the incoming connections
    listener.incoming().for_each(|stream| {
        match stream {  // Match the stream (connection)
            Ok(stream) => pool.execute(|| handle_client(stream)),
            Err(e) => eprintln!("Error: {}", e),
        }
    });

}


fn handle_client(mut stream: TcpStream)  {
    // * Buffer reads the data from the stream and stores it in the buffer
    let mut buffer = [0; 1024];  // 1 KB buffer (1024 bytes)
    // let mut buffer = [0; 32768];  // 32 KB buffer (32768 bytes)
    // let mut buffer = [0; 131072];  // 128 KB buffer (131072 bytes)

    match stream.read(&mut buffer) {  // Read the data from the stream and store it in the buffer
        Ok(size) => {  // If the read was successful
            // log::info!("({} Bytes):\n{} ", size, String::from_utf8_lossy(&buffer[..size]));

            let request_line = String::from_utf8_lossy(&buffer[..size])
                .lines()  // Split the string into lines
                .next()  // Get the first line
                .unwrap()  // Unwrap the Option
                .to_owned();  // Convert the &str to String
            // println!("Request line: {:?}", request_line);

            //  ^ Avoid the favicon request (a browser will always request the favicon)
            match request_line.split_whitespace().nth(1) == Some("/favicon.ico") {
                true => {  // If the request is for the favicon, return (close the connection)
                    log::warn!("Favicon requested. POSSIBLY IS A Repeated request?");
                    return
                },
                false => {  // * If the request is not for the favicon, then continue
                    log::trace!("New connection from {}", stream.peer_addr().unwrap());
                    match_request_line(request_line);
                }
            }

            // * Manage Response --------------------------------------------------------------

            let response = HttpResponse::new(
                HttpStatus::_200,
                HttpVersion::Http1_1,
                std::fs::read_to_string("resources\\html\\200.html").unwrap()  // Contents of the file
            ).to_string();

            // The difference between .write and .write_all is that .write_all writes the entire buffer
            // So .write will write the first 5 bytes of the buffer
            // And .write_all will write the entire buffer
            stream.write(response.as_bytes()).unwrap();  // Write the 'response' as bytes to the client's connection.
            // ? Return the response in reverse (for testing)
            // stream.write_all(String::from_utf8_lossy(&buffer[..size]).chars().rev().collect::<String>().as_bytes()).unwrap();
            
            // * To be sure that the response is sent, flush the stream
            // match stream.flush() {  // Flush the stream to make sure that the response is sent
            //     Ok(_) => log::trace!("Response sent."),  // If the flush was successful
            //     Err(e) => log::error!("Failed to flush the stream: {}", e),
            // }

            // Ok(())  // Confirm that the function was executed successfully
        }
        Err(e) => {
            log::error!("Failed to read from connection: {}", e);
            // Err(())
        },
    }

}


// /// Close the connection.
// /// 
// /// This function is not used because the connection is closed by the client.
// /// 
// /// This function is only used for testing.
// /// 
// /// # Arguments
// /// 
// /// - `stream` - The TcpStream to close.
// fn close_connection(mut stream: TcpStream) {
//     log::warn!("Closing connection.");
//     stream.write_all("Closing connection.".as_bytes()).unwrap();  // send a message to the client
//     stream.shutdown(std::net::Shutdown::Both).unwrap();  // close the connection (both ways)
//     std::process::exit(0);  // finalize the program
// }


fn match_request_line(request_line: String) {
    match request_line.split_whitespace().nth(0) {  // * Http method
        Some(method) => log::info!("{}", HttpMethod::from_str(method).unwrap().as_str()),
        None => log::error!("Invalid request line: {}", request_line),
    }
    match request_line.split_whitespace().nth(2) {  // * Http version
        Some(http_version) => log::info!("{}", HttpVersion::from_str(http_version).unwrap()),
        None => log::error!("Invalid request line: {}", request_line),
    }
    match request_line.split_whitespace().nth(1) {  // * Url
        Some(url) => log::info!("{}", url),
        None => log::error!("Invalid request line: {}", request_line),
    }
}

// // SAME AS ABOVE BUT USING MAP_OR_ELSE
// todo: Bench both functions and see which one is faster
// fn match_request_line(request_line: String) {
//     // match http method (GET, POST, PUT, DELETE, etc.)
//     request_line.split_whitespace().nth(0).map_or_else(  // * Http method
//         || log::error!("Invalid request line: {}", request_line),
//         |method| log::info!("{}", HttpMethod::from_str(method).unwrap().as_str()),
//     );
//     // match http version (HTTP/1.1, HTTP/2.0, etc.)
//     request_line.split_whitespace().nth(2).map_or_else(  // * Http version
//         || log::error!("Invalid request line: {}", request_line),
//         |http_version| log::info!("{}", HttpVersion::from_str(http_version).unwrap()),
//     );
//     // match url services (/, /about, /contact, etc.)
//     request_line.split_whitespace().nth(1).map_or_else(  // * Url
//         || log::error!("Invalid request line: {}", request_line),
//         |url| log::info!("{}", url),
//     );
// }
