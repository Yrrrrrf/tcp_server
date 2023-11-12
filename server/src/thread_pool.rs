//! A simple thread pool implementation.
//! 
//! # Examples
//! mod thread_pool;
use std::sync::{mpsc, Mutex, Arc};
use std::thread;


// Job is a type alias for a trait object that holds the type of closure that execute will receive.
// It can be any type that implements the FnOnce trait.
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

/// A simple thread pool implementation.
#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);  // The size must be greater than 0
        let (sender, receiver) = mpsc::channel();  // Create a channel to send jobs to the workers
        // Allow the receiver to be shared among multiple threads
        let receiver = Arc::new(Mutex::new(receiver));  // Create a mutex to share the receiver among the workers
        let mut workers = Vec::with_capacity(size);
        (0..size).for_each(|id| workers.push(Worker::new(id, Arc::clone(&receiver))));
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
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        // do the same as above using a closure
        self.workers.iter().for_each(|_| self.sender.send(Message::Terminate).unwrap());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }

}


#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = Some(thread::spawn(move || {  // Spawn a new thread
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();  // Receive a message from the channel
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();  // Execute the job
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    },
                }
            }
        }));
        Worker { 
            id, 
            thread
        }
    }
}
