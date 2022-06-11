use crate::snooker::{EventLink, PlayerLink};
use reqwest::Client;
use std::collections::BTreeSet;
use std::error::Error;
use chrono::Utc;

mod html;

mod snooker;

mod tables;

static APP_USER_AGENT : &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.63 Safari/537.36 Edg/102.0.1245.33";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::builder().user_agent(APP_USER_AGENT).build()?;

let now = Utc::now();

let cur_url = if 0==now.seconds%2 {
snooker::upcoming_matches()
} else {
snooker::results(2021)
};

    let resp = client.get(cur_url).send().await?;

    println!("{:#?}", resp.url().to_string());

    let text = resp.text().await?;

    let urls = html::parse_links(&text);

    let purls = urls
        .iter()
        .filter_map(|u| PlayerLink::try_from(u).ok())
        .collect::<BTreeSet<PlayerLink>>();

    println!("player links count: {:?}", purls.len().to_string());

    purls.iter().for_each(|p| {
        match tables::add_player(p) {
            Ok(f) => println!("Ok: {:?} - {:?}", f, p),
            Err(error) => println!("Err: {:?}", error),
        };
    });

    let eurls = urls
        .iter()
        .filter_map(|u| EventLink::try_from(u).ok())
        .collect::<BTreeSet<EventLink>>();

    println!("event links count: {:?}", eurls.len().to_string());

    eurls.iter().for_each(|e| {
        match tables::add_event(e) {
            Ok(f) => println!("Ok: {:?} - {:?}", f, e),
            Err(error) => println!("Err: {:?}", error),
        };
    });

    Ok(())
}
