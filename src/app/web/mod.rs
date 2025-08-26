use lazy_static::lazy_static;
use log::info;
use reqwest::Client;
use scraper::Html;
use std::{error::Error, time::Duration};

pub mod entities;

pub mod html;

static APP_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.63 Safari/537.36 Edg/102.0.1245.33";

lazy_static! {
    static ref CLIENT: Client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .connect_timeout(Duration::from_secs(60))
        .build()
        .unwrap();
}

pub async fn get(url: String) -> Result<Html, Box<dyn Error>> {
    let resp = CLIENT.get(&url).send().await?;
    info!(
        "get('{}') -> {},{}",
        &url,
        resp.status().as_str(),
        resp.status().canonical_reason().unwrap()
    );
    let content = resp.text().await?;
    Ok(Html::parse_document(&content))
}
