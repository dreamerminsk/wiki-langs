extern crate reqwest;


mod snooker;

use scraper::{Html, Selector};
use std::collections::HashSet;

fn parse_links(text: &str) -> HashSet<String> {
    let mut urls: HashSet<String> = HashSet::new();
    let document = Html::parse_document(&text);
    let selector = Selector::parse(r#"a"#).unwrap();
    for title in document.select(&selector) {
        let url = title
            .value()
            .attr("href")
            .expect("href not found")
            .to_string();
        if url != "/" || url != "." || url != ".." {
            urls.insert(url);
        }
    }
    urls
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(snooker::UPCOMINGMATCHES).await?;
    println!("{}", resp.url().to_string());

    let text = resp.text().await?;

    let urls = parse_links(&text);

    for url in urls {
        println!("{}", url);
    }

    Ok(())
}
