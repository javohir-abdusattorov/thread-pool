//! Basic thread pool implementation, without much of flavours or extra functionality
//! 
//! ### Usage
//! ```
//! let pool = ThreadPool::new(4);
//! let execution_result = pool.execute(|| {
//!     // Long running process / calculation / IO operation
//! });
//! 
//! assert_eq!(execution_result, Some(()))
//! ```

use std::sync::{mpsc, Arc, Mutex};

use worker::Worker;
use job::Job;
use error::ThreadPoolError;

mod worker;
mod job;
mod error;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {

    /// Creates a new ThreadPool, containing given size of workers. After initializing
    /// workers, they will be idle waiting for jobs
    /// 
    /// The size is number of threads will be in the pool
    /// 
    /// # Panics
    /// If size given is less than or equal to zero, cannot create empty or negative
    /// workers vector
    /// 
    /// # Example
    /// 
    /// ```
    /// let pool = ThreadPool::new(4);
    /// ```
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let worker = Worker::new(
                id,
                Arc::clone(&receiver),
            );

            workers.push(worker);
        }

        ThreadPool {
            workers: workers,
            sender: Some(sender),
        }
    }

    /// Executes given job. Immediately if any worker is idle, or waits for workers to
    /// finish
    /// 
    /// # Returns
    /// Result containing error enum, if sending message operation fails
    pub fn execute<F>(&self, f: F) -> Result<(), ThreadPoolError>
    where F: FnOnce() + Send + 'static 
    {
        let job = Box::new(f);

        match self.sender.as_ref() {
            Some(sender) => {
                sender.send(job).map_err(|_| ThreadPoolError::ReceiversDropeed)
            },
            None => Err(ThreadPoolError::SenderDropped)
        }
    }

}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap_or_else(|_| { panic!("Cannot join thread to main: unknown reason") });
            }
        }
    }
}