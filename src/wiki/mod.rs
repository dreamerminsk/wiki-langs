use crate::app::web;
use crate::app::web::html::Extract;
use entities::{InterWiki, Page};
use std::error::Error;
use scraper::Html;

pub mod entities;

pub async fn get_wiki(inter_wiki: InterWiki) -> Result<Page, Box<dyn Error>> {
    let page = web::get(inter_wiki.to_string()).await?;

    Ok(Page {
        lang: inter_wiki.lang,
        title: page
            .extract_text("#firstHeading")
            .unwrap_or_else(|| inter_wiki.title.to_string()),
        wikidata: extract_wikidata(&page),
    })
}

fn extract_wikidata(page: &Html) -> Option<String> {
    page.extract_links()
        .into_iter()
        .filter(|l| l.title == "Wikidata item");
    None
}
