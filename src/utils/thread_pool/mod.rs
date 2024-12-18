use std::{sync::Arc, thread};
use tokio::sync::Mutex;

pub struct ThreadPool {
      workers: Vec<Worker>,
      //sender: Option<mpsc::Sender<Job>>,
  }
  
  type Job = Box<dyn FnOnce() + Send + 'static>;
  
  impl ThreadPool {

      pub fn new(size: usize, receiver: tokio::sync::mpsc::Receiver<String>) -> ThreadPool {
          assert!(size > 0);
  
          //let (sender, receiver) = mpsc::channel();
  
          let receiver = Arc::new(Mutex::new(receiver));
  
          let mut workers = Vec::with_capacity(size);
  
          for id in 0..size {
              workers.push(Worker::new(id, receiver.clone()));
          }
  
          ThreadPool {
              workers,
              //sender: Some(sender),
          }
      }
  
      /* pub fn execute<F>(&self, f: F)
      where
          F: FnOnce() + Send + 'static,
      {
          let job = Box::new(f);
  
          self.sender.as_ref().unwrap().send(job).unwrap();
      } */
  }
  
  impl Drop for ThreadPool {
      fn drop(&mut self) {
          //drop(self.sender.take());
  
          for worker in &mut self.workers {
              //println!("Shutting down worker {}", worker.id);
  
              if let Some(thread) = worker.thread.take() {
                  thread.abort();
              }
          }
      }
  }
  
  struct Worker {
      id: usize,
      thread: Option<tokio::task::JoinHandle<()>>,
  }
  
impl Worker {
      fn new(id: usize, receiver: Arc<Mutex<tokio::sync::mpsc::Receiver<String>>>) -> Worker {
            let thread = tokio::spawn(async move {
                  loop {
                        let message = receiver.lock().await.recv().await;
        
                        match message {
                              Some(_value) => {
                                    //println!("Worker {id} got a job; executing.");
                                    println!("String received");
                              }
                              None => {
                                    //println!("Worker {id} disconnected; shutting down.");
                                    break;
                              }
                        }
                  }
            });
  
          Worker {
              id,
              thread: Some(thread),
          }
      }
  }