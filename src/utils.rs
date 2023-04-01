use std::{thread, sync::{mpsc, Arc, Mutex}};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    
    /*  
        Creates new ThreadPool

        # Params
        * **size**: number of threads in the pools.

        # Panics
        * Calling `new` with size is equal to 0 will result in a panic. 
    */

    pub fn new(size: usize) -> ThreadPool {

        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0 .. size { 
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }

    }

    pub fn execute<F>(&self, f: F) 
    where 
        F: FnOnce() + Send + 'static 
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

}


impl Worker {
    
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {

        let thread = thread::spawn(move || loop {

            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker #{} got a job. Executing...", id);
            job();

        });

        Worker { id, thread }
    
    }

}