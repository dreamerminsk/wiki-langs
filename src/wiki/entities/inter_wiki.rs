use std::fmt;





#[derive(Debug)]
pub struct InterWiki {
    pub lang: String,
    pub title: String,
}

impl InterWiki {
    fn new(lang: String, title: String) -> Self {
        InterWiki { lang, title }
    }
}








impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.longitude, self.latitude)
    }
}
