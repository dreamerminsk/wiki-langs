use scraper::{ElementRef, Html, Selector};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::convert::From;
use std::hash::{Hash, Hasher};

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
            url: item.value().attr("href").unwrap().to_string(),
            title: item.text().next().unwrap().to_string(),
        }
    }
}

fn parse_links(text: &str) -> BTreeSet<Link> {
    let document = Html::parse_document(text);
    let selector = Selector::parse(r#"a"#).unwrap();
    document
        .select(&selector)
        .map(Link::from)
        .collect::<BTreeSet<Link>>()
}
