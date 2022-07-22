use crate::country::tables;
use crate::wiki::{
    self,
    entities::{InterWiki, Page},
};

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
            .map(|c| c.wiki(&self.get_wiki(c.name.as_str())))
            .for_each(|c| {
                tables::add_country(&c);
            });
        Some(())
    }

    fn get_wiki(&self, name: &str) -> Page {
        let inter_wiki = InterWiki::new("en".to_string(), name.to_string());
        wiki::get_wiki(inter_wiki).ok().unwrap_or(Page {
            lang: "en".to_string(),
            title: name.to_string(),
        })
    }
}
