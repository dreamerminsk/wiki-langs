use std::cmp;
use std::fmt;

#[derive(Debug, cmp::Ord, cmp::PartialOrd, cmp::Eq, cmp::PartialEq)]
pub struct InterWiki {
    pub lang: String,
    pub title: String,
}

impl InterWiki {
    fn new(lang: String, title: String) -> Self {
        InterWiki { lang, title }
    }
}

impl fmt::Display for InterWiki {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "https://{}.wikipedia.org/wiki/{}", self.lang, self.title)
    }
}
