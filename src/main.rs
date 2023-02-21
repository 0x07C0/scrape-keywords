use anyhow::Result;
use futures::StreamExt;
use scrape_keywords::BookScraper;
use voyager::{CrawlerConfig, Collector};

#[tokio::main]
async fn main() -> Result<()> {
  let conf = CrawlerConfig::default().allow_domain("pragprog.com");
  let mut collector = Collector::new(BookScraper::default(), conf);
  collector.crawler_mut().visit("https://pragprog.com/titles/");
  while let Some(data) = collector.next().await {
    if let Ok(item) = data {
      if item.title.contains("Rust") {
        println!(
          "Book:\nTitle: {}\nImage: {}\nLink: {}",
          &item.title,
          &item.image_url,
          &item.link
        );
      }
    }
  }
  Ok(())
}
