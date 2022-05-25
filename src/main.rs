mod snooker;

use scraper::{ElementRef, Html, Selector};
use std::collections::HashSet;

use std::convert::From;

#[derive(Debug)]
pub struct Link {
    url: String,
    title: String,
}

impl From<ElementRef> for Link {
    fn from(item: &ElementRef) -> Self {
        Link {
            url: item.value().attr("href").unwrap().to_string(),
            title: item.text().nth(0).unwrap().to_string(),
        }
    }
}

fn parse_links(text: &str) -> HashSet<String> {
    let mut urls: HashSet<Link> = HashSet::new();
    let document = Html::parse_document(&text);
    let selector = Selector::parse(r#"a"#).unwrap();
    for a in document.select(&selector) {
        urls.insert(Link::from(&a));
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
