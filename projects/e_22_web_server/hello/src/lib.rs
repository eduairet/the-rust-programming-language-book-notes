use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>; // This is a type alias for a trait object that holds the type of closure that execute will receive.

#[derive(Debug)]
pub struct PoolCreationError;

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); // This closes the sending end of the channel.

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap(); // We call join on the thread to make sure that the worker thread has finished its job before we exit the program.
            }
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver)); // We wrap the receiver in an Arc and a Mutex to make it thread safe.
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            Ok(ThreadPool::new(size))
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap(); // We send the job down the sending end of the channel.
                                                          // We call as_ref on the Option<mpsc::Sender<Job>> to get a reference to the mpsc::Sender<Job> without taking ownership of the Option.
                                                          // This way, the Option<mpsc::Sender<Job>> is still available for the next call to execute.
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>, // We store the JoinHandle in the Worker struct so that the thread won’t be dropped when the Worker goes out of scope.
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            // This will block until a job is available.
            // This is a good thing because it means that the worker threads will wait for jobs rather than consuming CPU cycles while idle.
            // Never use while let because it will cause the thread to spin and consume CPU cycles.

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} shutting down."); // If the receiving end of the channel is dropped, recv will return an error.
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread), // Using Some here means that the thread field will be a Some variant of an Option<T>
        }
    }
}
