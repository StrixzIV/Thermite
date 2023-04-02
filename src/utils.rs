use std::{thread, sync::{mpsc, Arc, Mutex}};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum ThreadSignal {
    New(Job),
    Terminate
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<ThreadSignal>
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
        self.sender.send(ThreadSignal::New(job)).unwrap();
    }

}

impl Drop for ThreadPool {

    fn drop(&mut self) {

        println!("Sending terminate signal to all workers...");

        for _ in &self.workers {
            self.sender.send(ThreadSignal::Terminate).unwrap();
        }

        println!("Shutting down all workers...");
        
        for worker in &mut self.workers {

            println!("Shutting down worker #{}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }

        }

    }

}


impl Worker {
    
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<ThreadSignal>>>) -> Worker {

        let thread = thread::spawn(move || loop {

            let signal = receiver.lock().unwrap().recv().unwrap();

            match signal {
                
                ThreadSignal::New(job) => {
                    println!("Worker #{} got a job. Executing...", id);
                    job();
                },
                
                ThreadSignal::Terminate => {
                    println!("Worker #{} getting terminate signal.", id);
                    break;
                }

            }

        });

        Worker { id, thread: Some(thread) }
    
    }

}