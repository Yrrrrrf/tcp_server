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
use std::sync::{mpsc, Mutex, Arc};
use std::thread::{self, Builder};

// External crates
use log::{LevelFilter, warn, info, debug, trace, error};
use dev_utils::log::rlog::RLog;
use dev_utils::print_app_data;

use dev_utils::http::{*, request::HttpRequest, response::HttpResponse};


// ? Main ---------------------------------------------------------------------------------------------------------------------
// async fn main() -> Result<(), Box<dyn std::error::Error>> {  // for async main (async-std)
fn main() {
    print_app_data(file!());  // Read the Cargo.toml file and print the app data (name, version, authors)
    // RLog::init_logger(LevelFilter::Info);  // Initialize the logger with the given log level
    RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    // ^ Actual code: --------------------------------------------------------------------------------------------------------
    // let server_address = format!("{}:{}", config::SERVER_IP, config::SERVER_PORT);
    let server_address = "127.0.0.1:8080";
    log::info!("Starting server at {}\n", server_address);
    let listener = TcpListener::bind(server_address)  // Listen on this address
        .expect("Failed to bind to address.");  // If it fails, print this message

    let mut request_count = 0;
    let pool = ThreadPool::new(8);  // Create a thread pool with 8 threads

    // Iterate over the incoming connections
    listener.incoming().for_each(|stream| {
        match stream {  // Match the stream (connection)
            Ok(stream) => pool.execute(|| handle_client(stream)),
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

            // * Manage Request ---------------------------------------------------------------
            // log::info!("({} Bytes):\n{} ", size, String::from_utf8_lossy(&buffer[..size]));


            // get the first line of the request
            let request_line = String::from_utf8_lossy(&buffer[..size]).lines().next().unwrap().to_owned();
            println!("Request line: {}", request_line);

            // * Impl a better way to parse the request line
            // ^ match the HTTP method
            match request_line.split_whitespace().next() {
                Some(method) => match method {
                    "GET" => log::info!("GET request"),
                    "POST" => log::info!("POST request"),
                    "PUT" => log::info!("PUT request"),
                    "DELETE" => log::info!("DELETE request"),
                    // "CONNECT" => log::info!("CONNECT request"),
                    _ => log::error!("Invalid method: {}", method)
                }
                None => log::error!("Invalid request line: {}", request_line),
            };

            // ^ match the URL
            match request_line.split_whitespace().nth(1) {
                Some(url) => match url {
                    "/" => log::info!("Home page"),
                    "/about" => log::info!("About page"),
                    "/contact" => log::info!("Contact page"),
                    _ => log::error!("Invalid URL: {}", url)
                }
                None => log::error!("Invalid request line: {}", request_line),
            };

            // ^ match the HTTP version
            let http_version = match request_line.split_whitespace().nth(2) {
                Some(http_version) => match http_version {
                    // "HTTP/1.0" => log::info!("HTTP/1.0"),
                    "HTTP/1.1" => log::info!("HTTP/1.1"),
                    "HTTP/2.0" => log::info!("HTTP/2.0"),
                    _ => log::error!("Invalid HTTP version: {}", http_version)
                }
                None => log::error!("Invalid request line: {}", request_line),
            };


            // todo: Check if the match is necessary to close the connection
            // todo: Probably not, because the connection is closed when the client closes it
            // todo: It should be only used when the server wants to close the connection? Or not?

            
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
            match stream.flush() {  // Flush the stream to make sure that the response is sent
                Ok(_) => log::trace!("Response sent."),  // If the flush was successful
                Err(e) => log::error!("Failed to flush the stream: {}", e),
            }

        }
        Err(e) => log::error!("Failed to read from connection: {}", e),
    }

}


/// Closes the connection.
/// 
/// # Arguments
fn close_connection(mut stream: TcpStream) {
    log::warn!("Closing connection.");
    stream.write_all("Closing connection.".as_bytes()).unwrap();  // send a message to the client
    stream.shutdown(std::net::Shutdown::Both).unwrap();  // close the connection (both ways)
    std::process::exit(0);  // finalize the program
}




// ? Thread pool --------------------------------------------------------------------------------------------------------------

/// A simple thread pool implementation.
#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Creates a new ThreadPool with the specified size.
    /// The ThreadPool will have `size` threads.
    /// 
    // / For now is limited to a u8 (max 255 threads).
    // pub fn new(size: u8) -> ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // let mut threads = Vec::with_capacity(size);

        // ^ Return a Panic if the size is 0
        assert!(size > 0);  // The size must be greater than 0

        let (sender, receiver) = mpsc::channel();  // Create a channel to send jobs to the workers

        // Allow the receiver to be shared among multiple threads
        let receiver = Arc::new(Mutex::new(receiver));  // Create a mutex to share the receiver among the workers

        let mut workers = Vec::with_capacity(size.into());

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        // The line below is equivalent to the for loop above
        // todo: Check if the for loop is better than the map
        // let workers = (0..size).map(|id| Worker::new(id.into())).collect::<Vec<_>>();


        ThreadPool {
            workers,
            sender,
        }
    }

    /// Executes the provided closure in a thread from the pool.
    ///
    /// # Arguments
    ///
    /// - `f` - A closure to be executed in a separate thread.
    ///
    /// # Examples
    // todo: Fix the example (use module imports) 
    /// ```
    /// use my_thread_pool::ThreadPool;  // todo: Fix this
    ///
    /// let pool = ThreadPool::new(4);  // Spawn a pool with 4 threads
    /// pool.execute(|| {  // Execute a closure in a separate thread
    ///     println!("This closure is executed in a separate thread!");
    /// });
    /// ```
    pub fn execute<F>(&self, f: F)  // execute a closure in a separate thread
    where
        F: FnOnce() + Send + 'static
        // * FnOnce() is a closure that takes no arguments and returns nothing
        // * Send means that the closure can be sent from one thread to another
        // * 'static means that the closure does not reference anything on the stack (it can be moved to another thread)
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
        // The implementation of thread execution goes here.
    }
}


/// Spawns a new thread and returns a JoinHandle for it.
/// 
/// A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish.
/// 
/// # Arguments
///
/// * `f` - A closure representing the code to be executed in the spawned thread.
///
/// # Returns
///
/// A JoinHandle<T> where T is the type returned by the closure.
///
/// # Examples
///
/// ```
/// use my_thread_pool::spawn;
///
/// let handle = spawn(|| {
///     println!("This closure is executed in a spawned thread!");
///     42
/// });
///
/// let result = handle.join().unwrap();
/// println!("Result from the spawned thread: {}", result);
/// ```
pub fn spawn<F, T>(f: F) -> thread::JoinHandle<T>
    where
        F: FnOnce() -> T,  // a closure that takes no arguments and returns T
        F: Send + 'static,  // the closure can be sent from one thread to another
        T: Send + 'static,  // the closure does not reference anything on the stack (it can be moved to another thread)
{
    Builder::new().spawn(f).expect("Failed to spawn thread.")
}


type Job = Box<dyn FnOnce() + Send + 'static>;
// import the 'Job' struct

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {id} got a job; executing.");
                job();
            }
        });

        Worker { id, thread }
    }
}


// todo: Impl a better way to parse the request line
// todo: Understand at 100% the code above (mostly the ThreadPool struct & it's impl techniques)
// todo: Impl Shoutdown and Cleanup behavior for the ThreadPool

