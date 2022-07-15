use scraper::{ElementRef, Html, Selector};
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::convert::From;
use std::hash::{Hash, Hasher};
use url::Url;
use crate::services::web::entities::Link;

fn query(u: Url) -> HashMap<String, String> {
    u.query_pairs().into_owned().collect()
}

pub trait Extract {
    fn extract_links(&self) -> BTreeSet<Link>;
    fn extract_text(&self, selector: &str) -> Option<String>;
}

impl Extract for Html {
    fn extract_links(&self) -> BTreeSet<Link> {
        let selector = Selector::parse(r#"a"#).unwrap();
        self.select(&selector)
            .map(Link::from)
            .collect::<BTreeSet<Link>>()
    }

    fn extract_text(&self, selector: &str) -> Option<String> {
        let selector = Selector::parse(selector).unwrap();
        self.select(&selector)
            .map(|t| t.text().collect::<String>())
            .next()
    }
}
