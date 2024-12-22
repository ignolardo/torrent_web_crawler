use std::thread;
use std::time::Duration;
use crawler::web_crawler::WebCrawler;

#[tokio::main]
async fn main() {
    let mut web_crawler = WebCrawler::new(vec![
        "https://google.com",
        "https://microsoft.com",
        "https://projecteuler.net/",
    ]);

    web_crawler.start(5).await;
    thread::sleep(Duration::from_secs(5));
    web_crawler.stop().await;
}
