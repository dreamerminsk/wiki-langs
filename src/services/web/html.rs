use scraper::{ElementRef, Html, Selector};
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::convert::From;
use std::hash::{Hash, Hasher};
use url::Url;

fn query(u: Url) -> HashMap<String, String> {
    u.query_pairs().into_owned().collect()
}
impl for Html{
pub fn extract_links(&self) -> BTreeSet<Link> {
    let selector = Selector::parse(r#"a"#).unwrap();
    self
        .select(&selector)
        .map(Link::from)
        .collect::<BTreeSet<Link>>()
}

pub fn extract_text(&self, selector: &str) -> Option<String> {
    let selector = Selector::parse(selector).unwrap();
    self
        .select(&selector)
        .map(|t| t.text().collect::<String>())
        .next()
}
}

#[derive(Debug)]
pub struct Link {
    pub url: String,
    pub title: String,
}

impl Ord for Link {
    fn cmp(&self, other: &Self) -> Ordering {
        self.url.cmp(&other.url)
    }
}

impl PartialOrd for Link {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

impl Eq for Link {}

impl Hash for Link {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

impl<'a> From<ElementRef<'a>> for Link {
    fn from(item: ElementRef<'a>) -> Self {
        Link {
            url: item.value().attr("href").unwrap_or("#").to_string(),
            title: item.text().next().unwrap_or("").to_string(),
        }
    }
}
