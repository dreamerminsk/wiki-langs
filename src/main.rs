mod snooker;

use scraper::{ElementRef, Html, Selector};
use std::collections::{BTreeSet, HashSet};
use std::convert::From;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Link {
    url: String,
    title: String,
}

impl PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}
impl Eq for Link {}

impl<'a> From<ElementRef<'a>> for Link {
    fn from(item: ElementRef<'a>) -> Self {
        Link {
            url: item.value().attr("href").unwrap().to_string(),
            title: item.text().nth(0).unwrap().to_string(),
        }
    }
}

fn parse_links(text: &str) -> BTreeSet<Link> {
    let document = Html::parse_document(&text);
    let selector = Selector::parse(r#"a"#).unwrap();
    document
        .select(&selector)
        .map(|x| Link::from(x))
        .collect::<BTreeSet<Link>>()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(snooker::UPCOMING_MATCHES).await?;
    println!("{:#?}", resp.url().to_string());

    let text = resp.text().await?;

    let urls = parse_links(&text);

    for url in urls {
        println!("{:#?}", url);
    }

    Ok(())
}
