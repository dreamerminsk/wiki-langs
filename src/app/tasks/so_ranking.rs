use log::info;
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::time::Duration;

const APP_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.63 Safari/537.36 Edg/102.0.1245.33";
const NATION_REPORT_PATH: &str = "./RANKING/CURRENT.NATION.csv";

pub struct SoRanking {
    client: Client,
}

#[derive(Debug, Clone)]
pub struct RankingItem {
    position: String,
    player: String,
    player_id: String,
    nation: String,
    sum: usize,
    change: isize,
}

#[derive(Debug, Clone)]
pub struct NationStats {
    sum: usize,
    change: isize,
    players: usize,
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
        let mut nation_stats: HashMap<String, NationStats> = HashMap::new();

        for row in document.select(&table_selector) {
            let ranking_item = self.parse_rank_item(&row)?;
            ranking_items.push(ranking_item.clone());

            let stats = nation_stats
                .entry(ranking_item.nation.clone())
                .or_insert(NationStats { sum: 0, change: 0, players: 0 });
            stats.sum += ranking_item.sum;
            stats.change += ranking_item.change;
             stats.players += 1;
        }

        for item in &ranking_items {
            info!(
                "Position: {}, Player: {}, ID: {}, Nationality: {}, Sum: {}, Change: {}",
                item.position, item.player, item.player_id, item.nation, item.sum, item.change
            );
        }

        self.save_nation_report(&nation_stats)?;

        Ok(())
    }

    fn parse_rank_item(&self, row: &ElementRef) -> Result<RankingItem, Box<dyn Error>> {
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
            .ok_or("Invalid Player ID")?
            .to_string();

        let nation = row
            .select(&Selector::parse(".nationality")?)
            .next()
            .ok_or("Nationality not found")?
            .text()
            .collect::<Vec<_>>()
            .join("");

        let sum_text_node = row
            .select(&Selector::parse(".sum")?)
            .next()
            .ok_or("Sum not found")?
            .inner_html();
        let sum = sum_text_node
            .trim()
            .parse::<usize>()
            .map_err(|_| "Failed to parse sum value")?;

        let change_text_node = row
            .select(&Selector::parse(".change")?)
            .nth(2)
            .ok_or("Sum change not found")?
            .inner_html();
        let change = change_text_node
            .trim()
            .parse::<isize>()
            .map_err(|_| "Failed to parse change value")?;

        Ok(RankingItem {
            position,
            player,
            player_id,
            nation,
            sum,
            change,
        })
    }

    fn save_nation_report(
        &self,
        nation_stats: &HashMap<String, NationStats>,
    ) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all("./RANKING/")?;
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(true)
            .open(NATION_REPORT_PATH)?;

        let mut content = String::from("Nation, Sum, Change, Players\n");

        let mut sorted_nation_stats: Vec<(String, NationStats)> = nation_stats
            .iter()
            .map(|(nation, stats)| (nation.clone(), stats.clone()))
            .collect();

        sorted_nation_stats.sort_by(|a, b| b.1.sum.cmp(&a.1.sum));

        for (nation, stats) in sorted_nation_stats {
            content.push_str(&format!("{}, {}, {}, {}\n", nation, stats.sum, stats.change, stats.players));
        }

        file.write_all(content.as_bytes())?;
        file.flush()?;

        Ok(())
    }
}
