use crate::services::web::entities::Link;
use scraper::{Html, Selector};
use std::collections::{BTreeSet, HashMap};
use url::Url;

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
