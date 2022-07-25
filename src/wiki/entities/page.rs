use crate::wiki::entities::InterWiki;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Page {
    pub lang: String,
    pub title: String,
    pub wikidata: Option<String>,
    pub inter_wikis: BTreeSet<InterWiki>,
}

impl Page {
    fn new(
        lang: String,
        title: String,
        wikidata: Option<String>,
        inter_wikis: BTreeSet<InterWiki>,
    ) -> Self {
        Page {
            lang,
            title,
            wikidata,
            inter_wikis,
        }
    }
}
