use crate::wiki::{self, InterWiki};
use country::tables;

pub struct UpdateCountries {}

impl UpdateCountries {
    pub fn new() -> Self {
        UpdateCountries {}
    }

    pub fn execute(&self) -> Option<()> {
        let list = tables::get_all_countries().ok()?;
        list.into_iter()
            .filter(|c| c.wiki_id.is_none())
            .take(3)
            .filter_map(|c| wiki.get_wiki(InterWiki::new("en", c.name)).ok());
        Some(())
    }
}

