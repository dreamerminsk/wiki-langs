use crate::snooker::{EventLink, PlayerLink};
use chrono::{Timelike, Utc};
use rand::Rng;
use reqwest::Client;
use std::collections::BTreeSet;
use std::error::Error;

mod html;

mod snooker;

mod tables;

static APP_USER_AGENT : &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.63 Safari/537.36 Edg/102.0.1245.33";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let random_id: u32 = rng.gen_range(1..100);
    let player = snooker::get_player(usize::try_from(random_id)?).await?;
    tables::add_player(player)?;

    let client = Client::builder().user_agent(APP_USER_AGENT).build()?;

    let now = Utc::now();

    let cur_url = match (now.hour() + now.minute()) % 5 {
        0 => "http://www.snooker.org/res/index.asp?player=39&season=2019".to_string(),
        1 => "http://www.snooker.org/res/index.asp?event=1143".to_string(),
        2 => format!("{}{}", snooker::results(2019), "&numperpage=50&page=28"),
        3 => "http://www.snooker.org/res/index.asp?template=2&season=2017".to_string(),
        _ => snooker::rankings(2010),
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
        match tables::add_player_link(p) {
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
