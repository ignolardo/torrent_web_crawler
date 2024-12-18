use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use reqwest::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use crate::utils::thread_pool::ThreadPool;
use crate::utils::Worker;

pub struct WebCrawler {
      //proc_u: Arc<Mutex<ProcessingUnit>>,
      frontier: Arc<Mutex<CrawlerFrontier>>,
      //handle: Option<JoinHandle<()>>,
      //pool: Arc<Mutex<ThreadPool>>,
      //stop_sender: Option<Sender<()>>,
      workers: Vec<Worker>,
      active: Arc<Mutex<bool>>,

}

impl WebCrawler {
      pub fn new(seeds: Vec<&str>) -> Self {
            
            let frontier = CrawlerFrontier::new(seeds.iter().map(|s| String::from(*s)).collect());
            /* for seed in seeds {
                  frontier.push(seed.into());
            }; */

            Self {
                  //proc_u: Arc::new(Mutex::new(ProcessingUnit::new())),
                  frontier: Arc::new(Mutex::new(frontier)),
                  //handle: None,
                  //pool: Arc::new(Mutex::new(ThreadPool::new(10))),
                  //stop_sender: None,
                  workers: vec![],
                  active: Arc::new(Mutex::new(false)),
            }
      }

      pub async fn start(&mut self, threads: usize) {

            self.set_active(true).await;
            
            for i in 0..threads {
                  self.spawn_thread(i);
            }

            /* let handle = thread::spawn(move || {
                  while let Err(TryRecvError::Empty) = rx.try_recv() {
                        if let Some(_link) = frontier.lock().unwrap().next() {
                              let proc_u = proc_u.clone();
                              pool.lock().unwrap().execute(move || {
                                    // GET HTML
                                    proc_u.lock().unwrap().push(String::from("page"));
                              });     
                        };
                  }
            }); */

            //self.handle = Some(handle);
      }

      pub async fn stop(&self) {
            self.set_active(false).await;
      
            //let _ = self.handle.take().unwrap().join();
      }

      async fn set_active(&self, value: bool) {
            *self.active.lock().await = value;
      }

      fn spawn_thread(&mut self, id: usize) {
            let frontier = self.frontier.clone();
            let active = self.active.clone();
            let worker = Worker::new(id.clone(), async move {
                  'a: loop {
                        let mut frontier = frontier.lock().await;
                        if !*active.lock().await {break;}
                        let mut next = frontier.next();
                        while next.is_none() {
                              if !*active.lock().await {break 'a;}
                              next = frontier.next();
                        }
                        let url = next.unwrap();
                        println!("New value from frontier: {}, reading from thread {}", url, id.clone());
                        let page = fetch_page(url).await;
                        if page.is_err() {
                              println!("Error: {}", page.unwrap_err());
                        } else {
                              let page = page.unwrap();
                              println!("{}", page);
                        }
                  }
            });
            self.workers.push(worker);
      }

}



async fn fetch_page(url: String) -> Result<String,Error>{

      // Fetching page
      let res = reqwest::get(url).await;
      if res.is_err() {
            return Err(res.unwrap_err());
      }
      let res = res.unwrap();
      let page = res.text().await;
      if page.is_err() {
            return Err(page.unwrap_err());
      }
      Ok(page.unwrap())
}






struct ProcessingUnit {
      queue: VecDeque<String>,
      //pool: Arc<Mutex<ThreadPool>>,
      //handle: JoinHandle<()>,
}

impl ProcessingUnit {
      pub fn new() -> Self {
            /* let pool = Arc::new(Mutex::new(ThreadPool::new(10)));
            let pool_ref = pool.clone();

            let queue = Arc::new(Mutex::new(VecDeque::<String>::new()));
            let queue_ref = queue.clone();

            let handle = thread::spawn(move || loop {
                  if let Some(_page) = queue_ref.lock().unwrap().pop_front() {
                        pool_ref.lock().unwrap().execute(move || {
                              println!("New page received from processing unit");
                        });
                  }
            }); */

            Self {
                  queue: VecDeque::new(),
                  /* pool,
                  handle, */
            }
      }

      pub fn push(&mut self, page: String) {
            //self.queue.lock().unwrap().push_back(page);
      }
}

impl Drop for ProcessingUnit {
      fn drop(&mut self) {
          //self.handle.join();
      }
}

struct CrawlerFrontier {
      queue: VecDeque<String>,
}

impl CrawlerFrontier {
      pub fn new(seeds: Vec<String>) -> Self {
            Self {
                  queue: seeds.into(),
            }
      }

      pub fn push(&mut self, link: String) {
            self.queue.push_back(link);
      }

      pub async fn next_await(&mut self) -> String {
            loop {
                  if let Some(value) = self.queue.pop_front() {return value;}
            }
      }

      pub fn next(&mut self) -> Option<String> {
            self.queue.pop_front()
      }
}