#[derive(Debug)]
pub struct Page {
    pub lang: String,
    pub title: String,
    pub wikidata: Option<String>,
}

impl Page {
    fn new(lang: String, title: String, wikidata: Option<String>) -> Self {
        Page {
            lang,
            title,
            wikidata,
        }
    }
}
