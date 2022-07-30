use crate::country::tables;
use crate::wiki::{
    self,
    entities::{InterWiki, Page},
};
use std::collections::BTreeSet;

pub struct UpdateCountries {}

impl UpdateCountries {
    pub fn new() -> Self {
        UpdateCountries {}
    }

    pub async fn execute(&self) -> Option<()> {
        let list = tables::get_all_countries("uk").ok()?;
        let filtered = list
            .into_iter()
            .filter(|c| c.wiki_id.as_ref().unwrap().contains(&c.name))
            .take(5);
        for c in filtered {
            let updated = self.get_wiki(c.name.as_str()).await;
            //tables::add_country("uk", &updated);
        }
        Some(())
    }

    async fn get_wiki(&self, name: &str) -> Page {
        let inter_wiki = InterWiki::new("en".to_string(), name.to_string());
        wiki::get_wiki(inter_wiki).await.unwrap_or(Page {
            lang: "en".to_string(),
            title: name.to_string(),
            wikidata: None,
            inter_wikis: BTreeSet::new(),
        })
    }
}
