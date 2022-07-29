use crate::country::{entities::Country, tables::add_country};
use app::tasks::{UpdateCountries, UpdateReadMe};
use rand::Rng;
use std::{
    error::Error,
    fs::{read_to_string, write},
};

mod app;

mod country;

mod snooker;

mod tables;

mod wiki;

struct NextPlayer(usize);
impl NextPlayer {
    fn get(&self) -> usize {
        self.0 = read_to_string("./next-player.csv")
            .and_then(|t| t.parse::<usize>())
            .unwrap_or_default();
        self.0
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    for i in 0..7 {
        let random_id: u32 = rng.gen_range(std::ops::Range {
            start: 300 * i,
            end: 300 * (i + 1),
        });
        let pid = i + 29;
        let player = snooker::get_player(usize::try_from(pid)?).await?;
        tables::add_player(&player)?;
        if !player.nation.is_empty() {
            let country = Country::from(player.nation);
            //add_country(&country)?;
        }
    }
    vec!["State of Palestine"]
        .into_iter()
        .map(|it| Country::from(it.to_string()))
        .for_each(|it| add_country(&it).ok().unwrap_or_default());

    let update_countries = UpdateCountries::new();
    update_countries.execute().await;

    let update_readme = UpdateReadMe::new();
    update_readme.execute();

    Ok(())
}
