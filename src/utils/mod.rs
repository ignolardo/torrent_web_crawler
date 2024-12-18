use std::future::Future;

pub mod thread_pool;


pub struct Worker {
      id: usize,
      thread: Option<tokio::task::JoinHandle<()>>,
}

impl Worker
{
      pub fn new(id: usize, job: impl Future<Output = ()> + Send + 'static) -> Self
      {
            Self {
                  id,
                  thread: Some(tokio::spawn(job)),
            }
      }
}

impl Drop for Worker {
      fn drop(&mut self) {
            if let Some(thread) = self.thread.take() {
                  thread.abort();
            }
      }
}