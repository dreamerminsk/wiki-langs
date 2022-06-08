use crate::html::Link;
use crate::snooker::{EventLink, PlayerLink};
use scraper::{Html, Selector};
use std::collections::BTreeSet;
use std::error::Error;
use std::fs::{self, OpenOptions,File};
use std::path::Path;

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

fn add_player(plink: &PlayerLink) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./players/")?;
    let source_file = format!(
        "./players/ids.{:0>4}.csv",
        (100 * (plink.snooker_id / 100) + 99).to_string()
    );
    if Path::new(source_file).exists() {
let file = OpenOptions::new().read(true).open(source_file);
    } else {
        let f = File::create(source_file)?;
        let mut buf_reader = BufReader::new(f);
    }
    Ok(())
}

fn add_event(elink: &EventLink) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./events/")?;
    let source_file = format!(
        "./events/ids.{:0>4}.csv",
        (100 * (elink.snooker_id / 100) + 99).to_string()
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get(format!("{}{}", snooker::HOST, snooker::UPCOMING_MATCHES)).await?;
    println!("{:#?}", resp.url().to_string());

    let text = resp.text().await?;

    let urls = parse_links(&text);

    let purls = urls
        .iter()
        .filter_map(|u| PlayerLink::try_from(u).ok())
        .collect::<BTreeSet<PlayerLink>>();

    purls.iter().for_each(|p| {
        add_player(p);
    });

    let eurls = urls
        .iter()
        .filter_map(|u| EventLink::try_from(u).ok())
        .collect::<BTreeSet<EventLink>>();

    eurls.iter().for_each(|e| {
        add_event(e);
    });

    Ok(())
}
