use crate::app::web::{self, html::HtmlExt};
use entities::{InterWiki, Page};
use lazy_static::lazy_static;
use regex::Regex;
use scraper::Html;
use std::{collections::BTreeSet, error::Error};

pub mod entities;

pub async fn get_wiki(inter_wiki: InterWiki) -> Result<Page, Box<dyn Error>> {
    let page = web::get(inter_wiki.to_string()).await?;

    Ok(Page {
        lang: inter_wiki.lang,
        title: page
            .extract_text("#firstHeading")
            .unwrap_or_else(|| inter_wiki.title.to_string()),
        wikidata: extract_wikidata(&page),
        inter_wikis: extract_inter_wikis(&page),
    })
}

fn extract_wikidata(page: &Html) -> Option<String> {
    page.extract_links()
        .into_iter()
        .filter(|l| l.title == "Wikidata item")
        .flat_map(|l| extract_wikidata_id(&l.url))
        .next()
}

fn extract_wikidata_id(text: &str) -> Option<String> {
    lazy_static! {
        static ref WDID_RE: Regex = Regex::new(r"EntityPage/(?P<wdid>Q[0-9]+)").unwrap();
    }
    WDID_RE
        .captures(text)
        .and_then(|cap| cap.name("wdid").map(|wdid| wdid.as_str().to_string()))
}

fn extract_inter_wikis(page: &Html) -> BTreeSet<InterWiki> {
let selector = Selector::parse(r#"a.interlanguage-link-target"#).unwrap();
        self.select(&selector)
            .map(InterWiki::from)
            .collect::<BTreeSet<InterWiki>>()
}
