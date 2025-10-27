use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    io::Write,
};

pub struct SoRanking {}

impl SoRanking {
    pub fn new() -> Self {
        SoRanking {}
    }

    pub fn execute(&self) -> Option<()> {
        let url = "https://www.snooker.org/res/index.asp?template=31";
        let response = get(url).ok()?.text().ok()?;

        let document = Html::parse_document(&response);
        let table_selector = Selector::parse("#currentmoneyrankings tbody tr").unwrap();

        for row in document.select(&table_selector) {
            let position = row
                .select(&Selector::parse(".position").unwrap())
                .next()
                .unwrap()
                .inner_html();
            let player_element = row
                .select(&Selector::parse(".player a").unwrap())
                .next()
                .unwrap();
            let player = player_element.inner_html();
            let player_id = player_element
                .value()
                .attr("href")
                .unwrap()
                .split('=')
                .last()
                .unwrap();
            let nationality = row
                .select(&Selector::parse(".nationality").unwrap())
                .next()
                .unwrap()
                .inner_html();
            let sum = row
                .select(&Selector::parse(".sum").unwrap())
                .next()
                .unwrap()
                .inner_html();
            let sum_change = row
                .select(&Selector::parse(".change").unwrap())
                .next()
                .unwrap()
                .inner_html();

            println!(
                "Position: {}, Player: {}, ID: {}, Nationality: {}, Sum: {}, Sum Change: {}",
                position, player, player_id, nationality, sum, sum_change
            );
        }

        Some(())
    }
}
