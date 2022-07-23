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

    pub async fn execute(&self) -> Option<()> {
        let list = tables::get_all_countries().ok()?;
        let filtered = list.into_iter().filter(|c| c.wiki_id.is_none()).take(3);
        for c in filtered {
            let updated = c.wiki(&self.get_wiki(c.name.as_str()).await);
            tables::add_country(&updated);
        }
        Some(())
    }

    async fn get_wiki(&self, name: &str) -> Page {
        let inter_wiki = InterWiki::new("en".to_string(), name.to_string());
        wiki::get_wiki(inter_wiki).await.unwrap_or(Page {
            lang: "en".to_string(),
            title: name.to_string(),
            wikidata: None,
        })
    }
}
