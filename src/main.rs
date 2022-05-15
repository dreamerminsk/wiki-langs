extern crate reqwest;
extern crate select;

mod snooker;

use scraper::{Html, Selector};
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.wireshark.org/download/").await?;
    println!("{}", resp.url().to_string());


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let content = reqwest::get(snooker::UPCOMINGMATCHES).await?.text().await?;
    println!("{:?}", content);
    Ok(())
}
