use std::{thread, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Mensaje>,
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

enum Mensaje {
    NewJob(Job),
    Finish
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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

        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender}
    }

    pub fn ejecuta<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);  
        self.sender.send(Mensaje::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Enviando mensaje de finalización a todos los hilos");

        for _ in &self.workers {
            self.sender.send(Mensaje::Finish).unwrap();
        }

        println!("Apagando todos los hilos...");

        for worker in &mut self.workers {
            println!("Apagando hilo {}", worker.id);

            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

impl Worker {

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Mensaje>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let msg = receiver.lock().unwrap().recv().unwrap();

            match msg {
                Mensaje::NewJob(job) => {
                    println!("El hilo {} tiene trabajo!, Ejecutando...", id);
                    job();
                },
                Mensaje::Finish => {
                    println!("El hilo {} va a apagarse", id);
                    break;
                }
            }
        });
        Worker {id, handle: Some(thread)}
    }
}
