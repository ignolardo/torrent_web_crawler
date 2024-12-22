use std::collections::VecDeque;
use reqwest::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::utils::Worker;
use crate::filter_str_by;

struct ActiveState {
      reference: Arc<Mutex<bool>>,
}

impl ActiveState {
      pub fn new(value: bool) -> Self {
            Self {
                  reference: Arc::new(Mutex::new(value)),
            }
      }

      pub async fn is_active(&self) -> bool {
            let value = *self.reference.lock().await;
            value.clone()
      }

      pub async fn set_active(&self, value: bool) {
            *self.reference.lock().await = value;
      }
}

impl Clone for ActiveState {
      fn clone(&self) -> Self {
            Self {
                  reference: self.reference.clone(),
            }
      }
}

pub struct WebCrawler {
      //proc_u: Arc<Mutex<ProcessingUnit>>,
      frontier: Arc<Mutex<CrawlerFrontier>>,
      workers: Vec<Worker>,
      active_state: ActiveState,
}

impl WebCrawler {
      pub fn new(seeds: Vec<&str>) -> Self {
            Self {
                  //proc_u: Arc::new(Mutex::new(ProcessingUnit::new())),
                  frontier: Arc::new(Mutex::new(CrawlerFrontier::from(seeds))),
                  workers: vec![],
                  active_state: ActiveState::new(false),
            }
      }

      pub async fn start(&mut self, threads: usize) {

            self.set_active(true).await;
            
            for i in 0..threads {
                  self.spawn_thread(i);
            }
      }

      pub async fn stop(&mut self) {
            self.set_active(false).await;
            //let _ = self.handle.take().unwrap().join();
      }

      async fn set_active(&mut self, value: bool) {
            self.active_state.set_active(value).await;
      }

      fn spawn_thread(&mut self, id: usize) {
            let frontier = self.frontier.clone();
            let active_state = self.active_state.clone();

            let worker = Worker::new(id.clone(), async move {

                  'a: loop {

                        if !active_state.is_active().await {break 'a;}
                        let mut url = String::default();
                        if let Err(_) = wait_next_url(&active_state, &mut url, &frontier).await {break 'a;}

                        println!("New value from frontier: {}, reading from thread {}", url, id.clone());
                        let page = fetch_page(url).await;
                        if page.is_err() {
                              println!("Error: {}", page.unwrap_err());
                        } else {
                              let page = page.unwrap();
                              println!("{:#?}", filter_str_by!(&page,"href=\"{}\""));
                        }
                  }
            });
            self.workers.push(worker);
      }

}



async fn wait_next_url(active_state: &ActiveState,  url: &mut String, frontier: &Arc<Mutex<CrawlerFrontier>>) -> Result<(), ()> {
      let mut frontier = frontier.lock().await;
      loop {
            if !active_state.is_active().await {return Err(());}

            if let Some(t_url) = frontier.next() {
                  *url=t_url;
                  return Ok(());
            }
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
      pub fn new() -> Self {
            Self {
                  queue: VecDeque::new(),
            }
      }

      pub fn push(&mut self, link: String) {
            self.queue.push_back(link);
      }

      /* pub async fn next_await(&mut self) -> String {
            loop {
                  if let Some(value) = self.queue.pop_front() {return value;}
            }
      } */

      pub fn next(&mut self) -> Option<String> {
            self.queue.pop_front()
      }
}

impl From<Vec<&str>> for CrawlerFrontier {
      fn from(value: Vec<&str>) -> Self {
            let mut frontier = CrawlerFrontier::new();
            for seed in value {
                  frontier.push(String::from(seed));
            }
            frontier
      }
}