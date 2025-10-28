use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::time::Duration;

static APP_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.63 Safari/537.36 Edg/102.0.1245.33";

pub struct SoRanking {
    client: Client,
}

impl SoRanking {
    pub fn new() -> Self {
        SoRanking {
            client : Client::builder()
                .user_agent(APP_USER_AGENT)
                .connect_timeout(Duration::from_secs(60))
                .build()
                .expect("Failed to build HTTP client"),
        }
    }

    pub async fn execute(&self) -> Result<(), Box<dyn Error>> {
        let url = "https://www.snooker.org/res/index.asp?template=31";
        let response = self.client.get(url).send().await?.text().await?;

        let document = Html::parse_document(&response);
        let table_selector = Selector::parse("#currentmoneyrankings tbody tr")?;

        for row in document.select(&table_selector) {
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
            let nationality = row
                .select(&Selector::parse(".nationality")?)
                .next()
                .ok_or("Nationality not found")?
                .inner_html();
            let sum = row
                .select(&Selector::parse(".sum")?)
                .next()
                .ok_or("Sum not found")?
                .inner_html();
            let sum_change = row
                .select(&Selector::parse(".change")?)
                .next()
                .ok_or("Sum change not found")?
                .inner_html();

            println!(
                "Position: {}, Player: {}, ID: {}, Nationality: {}, Sum: {}, Sum Change: {}",
                position, player, player_id, nationality, sum, sum_change
            );
        }

        Ok(())
    }
}
