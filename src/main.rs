use crate::snooker::{EventLink, PlayerLink};
use chrono::{Timelike, Utc};
use reqwest::Client;
use std::collections::BTreeSet;
use std::error::Error;

mod html;

mod snooker;

mod tables;

static APP_USER_AGENT : &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.63 Safari/537.36 Edg/102.0.1245.33";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::builder().user_agent(APP_USER_AGENT).build()?;

    let now = Utc::now();

    let cur_url = match (now.hour() + now.minute()) % 5 {
        0 => format!("{}{}", snooker::upcoming_matches(), "&numperpage=50&page=5"),
        1 => format!("{}{}", snooker::results(2019), "&numperpage=50&page=3"),
        2 => snooker::rankings(2019),
        _ => "http://www.snooker.org/res/index.asp?template=2&season=2019".to_string(),
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

    update_players(&purls);

    let eurls = urls
        .iter()
        .filter_map(|u| EventLink::try_from(u).ok())
        .collect::<BTreeSet<EventLink>>();

    println!("event links count: {:?}", eurls.len().to_string());

    update_events(&eurls);

    Ok(())
}

fn update_players(plinks: &BTreeSet<PlayerLink>) {
    plinks.iter().for_each(|p| {
        match tables::add_player(p) {
            Ok(f) => println!("Ok: {:?} - {:?}", f, p),
            Err(error) => println!("Err: {:?}", error),
        };
    });
}

fn update_events(elinks: &BTreeSet<EventLink>) {
    elinks.iter().for_each(|e| {
        match tables::add_event(e) {
            Ok(f) => println!("Ok: {:?} - {:?}", f, e),
            Err(error) => println!("Err: {:?}", error),
        };
    });
}
