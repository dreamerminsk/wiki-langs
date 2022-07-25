use std::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    fmt,
    hash::Hash,convert::From,
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





impl<'a> From<ElementRef<'a>> for InterWiki {
    fn from(item: ElementRef<'a>) -> Self {
        InterWiki {
            lang: item.value().attr("lang").unwrap_or_default().to_string(),
            title: extract_title(item.value().attr("title").unwrap_or_default()),
        }
    }
fn extract_title(text:&str)->String{
}
}
