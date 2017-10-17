use std::thread;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker {
            id,
            thread
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// Size is the number of threads in ThreadPool
    ///
    /// # Panics
    /// The `new` function will panic if the size is 0
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool {
            workers
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}