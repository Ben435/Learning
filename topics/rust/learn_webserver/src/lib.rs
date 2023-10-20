pub mod thread_pool {
    use std::fmt;
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;

    use log::{debug};

    pub struct PoolCreationError<'a> {
        pub reason: &'a str,
    }

    impl PoolCreationError<'_> {
        fn new<'a>(reason: &'a str) -> PoolCreationError<'a> {
            PoolCreationError { reason }
        }
    }

    impl fmt::Debug for PoolCreationError<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("PoolCreationError")
                .field(&self.reason)
                .finish()
        }
    }

    impl fmt::Display for PoolCreationError<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Reason: {}", &self.reason)
        }
    }

    pub struct ThreadPool {
        threads: Vec<Worker>,
        sender: mpsc::Sender<Message>,
    }

    impl ThreadPool {
        pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError<'static>> {
            if size <= 0 {
                let err = PoolCreationError::new("size invalid");
                return Err(err);
            }

            let (sender, receiver) = mpsc::channel();

            let receiver = Arc::new(Mutex::new(receiver));

            let mut threads = Vec::with_capacity(size);

            for id in 0..size {
                threads.push(Worker::new(id, Arc::clone(&receiver)));
            }

            Ok(ThreadPool { threads, sender })
        }

        pub fn execute<F>(&self, work: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(work);

            self.sender.send(Message::NewJob(job)).unwrap();
        }
    }

    impl Drop for ThreadPool {
        fn drop(&mut self) {
            debug!("Terminating workers");

            for _ in &self.threads {
                self.sender.send(Message::Terminate).unwrap();
            }

            for worker in &mut self.threads {
                debug!("Shutting down worker: {}", worker.id);

                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            }
        }
    }

    struct Worker {
        id: usize,
        thread: Option<thread::JoinHandle<()>>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                match job {
                    Message::NewJob(job) => {
                        debug!("Worker {} got job; executing", id);

                        job();
                    }
                    Message::Terminate => {
                        debug!("Terminating worker {}", id);

                        break;
                    }
                }
            });

            Worker {
                id,
                thread: Some(thread),
            }
        }
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;

    enum Message {
        NewJob(Job),
        Terminate,
    }
}
