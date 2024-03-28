use std::sync::{mpsc, Arc, Mutex};

use crate::threadpool::worker::Worker;

/// A job is a function that can be executed by a thread in the thread pool.
pub type Job = Box<dyn FnOnce() + Send + 'static>;

/// A thread pool that can execute jobs.
pub struct Pool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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
        let sender = Some(sender);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        Ok(Pool { workers, sender })
    }

    /// Execute a function (job) on the thread pool.
    pub fn execute<F>(&self, f: F) -> Result<(), String>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender
            .as_ref()
            .ok_or("Failed to get sender")?
            .send(job)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

impl Drop for Pool {
    /// Close all workers when the thread pool is dropped.
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            error!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
