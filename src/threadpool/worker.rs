use crate::threadpool::pool::Job;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// A worker is a thread that listens for jobs on a receiver as part of a thread pool.
pub struct Worker {
    pub(super) id: usize,
    pub(super) thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Create a new worker. The worker will listen for jobs on the receiver.
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // TODO: Remove unwraps
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => job(),
                Err(e) => {
                    error!("Worker message not a job: {e}");
                    break;
                }
            }
        });

        let thread = Some(thread);

        Worker { id, thread }
    }
}
