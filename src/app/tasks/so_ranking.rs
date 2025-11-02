use log::info;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Duration;

const APP_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.63 Safari/537.36 Edg/102.0.1245.33";
const NATIONALITY_REPORT_PATH: &str = "./nationality_report.txt";

pub struct SoRanking {
    client: Client,
}

#[derive(Debug)]
pub struct RankingItem {
    position: String,
    player: String,
    player_id: String,
    nation: String,
    sum: u64,
    sum_change: i64,
}

impl SoRanking {
    pub fn new() -> Self {
        SoRanking {
            client: Client::builder()
                .user_agent(APP_USER_AGENT)
                .connect_timeout(Duration::from_secs(60))
                .build()
                .expect("Failed to build HTTP client"),
        }
    }

    pub async fn execute(&self) -> Result<(), Box<dyn Error>> {
        info!("SoRankingTask");
        let url = "https://www.snooker.org/res/index.asp?template=31";
        let response = self.client.get(url).send().await?.text().await?;

        let document = Html::parse_document(&response);
        let table_selector = Selector::parse("#currentmoneyrankings tbody tr")?;

        let mut ranking_items: Vec<RankingItem> = Vec::new();
        let mut nation_ranking: HashMap<String, u64> = HashMap::new();

        for row in document.select(&table_selector) {
            let ranking_item = self.parse_rank_item(row);
            ranking_items.push(ranking_item);

            *nation_ranking.entry(ranking_item.nation).or_insert(0) += ranking_item.sum;
        }

        for item in &ranking_items {
            info!(
                "Position: {}, Player: {}, ID: {}, Nationality: {}, Sum: {}, Sum Change: {}",
                item.position, item.player, item.player_id, item.nation, item.sum, item.sum_change
            );
        }
        self.save_nation_report(&nation_ranking)?;

        Ok(())
    }

    fn parse_rank_item(&self, row: &ElementRef) {
        let position = row
            .select(&Selector::parse(".position")?)
            .next()
            .ok_or("Position not found")?
            .inner_html();
        let player_element = row
            .select(&Selector::parse(".player a")?)
            .next()
            .ok_or("Player element not found")?;
        let player = player_element.inner_html();
        let player_id = player_element
            .value()
            .attr("href")
            .ok_or("Player ID not found")?
            .split('=')
            .last()
            .ok_or("Invalid Player ID")?;
        let nation = row
            .select(&Selector::parse(".nationality")?)
            .next()
            .ok_or("Nationality not found")?
            .inner_html();
        let sum_text_node = row
            .select(&Selector::parse(".sum")?)
            .next()
            .ok_or("Sum not found")?
            .inner_html();
        let sum = sum_text_node
            .trim()
            .parse::<u64>()
            .ok_or("Failed to parse sum value");
        let change_text_node = row
            .select(&Selector::parse(".change")?)
            .nth(2)
            .ok_or("Sum change not found")?
            .inner_html();
        let change = change_text_node
            .trim()
            .parse::<i64>()
            .ok_or("Failed to parse change value");

        RankingItem {
            position,
            player,
            player_id: player_id.to_string(),
            nation,
            sum,
            change,
        }
    }

    fn save_nation_report(
        &self,
        nationality_count: &HashMap<String, usize>,
    ) -> Result<(), Box<dyn Error>> {
        let file = File::create(NATIONALITY_REPORT_PATH)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "Nationality, Count")?;

        for (nationality, count) in nationality_count {
            writeln!(writer, "{}, {}", nationality, count)?;
        }

        Ok(())
    }
}
