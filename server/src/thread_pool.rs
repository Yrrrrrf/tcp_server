//! A simple thread pool implementation.
//! 
//! # Examples
//! mod thread_pool;


use std::sync::{mpsc, Mutex, Arc};
use std::thread::{self, Builder};


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
/// A `JoinHandle<T>` where T is the type returned by the closure.
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

