use std::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    fmt,
    hash::Hash,
};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InterWiki {
    pub lang: String,
    pub title: String,
}

impl InterWiki {
    pub fn new(lang: String, title: String) -> Self {
        InterWiki { lang, title }
    }
}

impl fmt::Display for InterWiki {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "https://{}.wikipedia.org/wiki/{}", self.lang, self.title)
    }
}
