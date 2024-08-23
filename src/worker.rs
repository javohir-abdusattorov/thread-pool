use std::{sync::{mpsc, Arc, Mutex}, thread};
use crate::job::Job;


pub struct Worker {
    pub handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || Worker::listen(id, receiver));

        println!("[WORKER-{id}] Initialized");
        Worker {
            handle: Some(thread),
        }
    }

    fn listen(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) {
        loop {
            match receiver.lock() {
                Ok(lock) => {
                    let message = lock.recv();
                    drop(lock);

                    match message {
                        Ok(job) => {
                            println!("[WORKER-{}] got a job, executing", id);
    
                            job();
                            println!("[WORKER-{}] execution done, going idle", id);
                        },
                        Err(error) => {
                            println!("[WORKER-{}] disconnected, shutting down: {}", id, error);
                            break;
                        }
                    }
                },
                Err(error) => {
                    println!("[WORKER-{}] mutex lock poisoned, shutting down: {}", id, error);
                    panic!("[WORKER-{}] Mutex lock poisoned", id);
                }
            }
        }
    }
}