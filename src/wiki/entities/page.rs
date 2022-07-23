#[derive(Debug)]
pub struct Page {
    pub lang: String,
    pub title: String,
    pub wikidata: String,
}

impl Page {
    fn new(lang: String, title: String, wikidata: String) -> Self {
        Page {
            lang,
            title,
            wikidata,
        }
    }
}
