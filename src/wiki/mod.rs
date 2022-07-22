use crate::app::web;
use entities::{InterWiki, Page};
use std::error::Error;

pub mod entities;

pub async fn get_wiki(inter_wiki: InterWiki) -> Result<Page, Box<dyn Error>> {
    let page = web::get(inter_wiki.to_string()).await?;

    Ok(Page {
        lang: inter_wiki.lang,
        title: page.extract_text("#firstHeading")?,
    })
}
