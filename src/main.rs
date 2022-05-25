extern crate reqwest;

extern crate scraper;

mod snooker;

use scraper::{Html, Selector};
use std::collections::HashSet;

pub struct Link {
    url: String,
    title: String,
}

fn parse_links(text: &str) -> HashSet<String> {
    let mut urls: HashSet<String> = HashSet::new();
    let document = Html::parse_document(&text);
    let selector = Selector::parse(r#"a"#).unwrap();
    for a in document.select(&selector) {
        let url = a.value().attr("href").expect("href not found").to_string();
        let title = a.text().nth(0).unwrap();
        if url != "/" || url != "." || url != ".." {
            urls.insert(url);
        }
    }
    urls
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(snooker::UPCOMING_MATCHES).await?;
    println!("{}", resp.url().to_string());

    let text = resp.text().await?;

    let urls = parse_links(&text);

    for url in urls {
        println!("{}", url);
    }

    Ok(())
}
