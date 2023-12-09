use std::sync::{mpsc, Arc, Mutex};

use crate::threadpool::worker::Worker;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Pool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl Pool {
    /// Create a new ThreadPool.
    /// The size is the number of threads in the pool.
    /// The `new` function will error if the size is zero.
    pub fn new(size: usize) -> Result<Pool, String> {
        if size == 0 {
            return Err("Size for cannot be zero.".into());
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        Ok(Pool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
