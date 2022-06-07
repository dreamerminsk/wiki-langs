use html::Link;
use scraper::{Html, Selector};
use snooker::PlayerLink;
use std::collections::BTreeSet;
use std::error::Error;

mod html;

mod snooker;

fn parse_links(text: &str) -> BTreeSet<Link> {
    let document = Html::parse_document(text);
    let selector = Selector::parse(r#"a"#).unwrap();
    document
        .select(&selector)
        .map(Link::from)
        .collect::<BTreeSet<Link>>()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get(format!("{}{}", snooker::HOST, snooker::UPCOMING_MATCHES)).await?;
    println!("{:#?}", resp.url().to_string());

    let text = resp.text().await?;

    let urls = parse_links(&text);

    let purls = urls
        .filter_map(|u| PlayerLink::try_from(u).ok())
        .collect::<BTreeSet<PlayerLink>>();

    for url in urls {
        println!("{:#?}", url);
    }

    Ok(())
}
