use crate::country::{entities::Country, tables::add_country};
use app::tasks::{UpdateCountries, UpdateReadMe};
use fern;
use log::{debug, error, info, trace, warn};
use rand::Rng;
use std::{
    error::Error,
    fs::{read_to_string, write},
    io,
};

mod app;

mod country;

mod cuetracker;

mod players;

mod snooker;

mod tables;

mod wiki;

struct NextPlayer(u32);
impl NextPlayer {
    fn get(&mut self) -> u32 {
        if self.0 == 0 {
            self.0 = read_to_string("./next-player.csv")
                .ok()
                .and_then(|t| t.parse::<u32>().ok())
                .unwrap_or_default();
        }
        self.0
    }
    fn put(&mut self, np: u32) -> io::Result<()> {
        self.0 = np;
        write("./next-player.csv", self.0.to_string())?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_logger()?;

    let update_countries = UpdateCountries::new();
    update_countries.execute().await;

    let update_readme = UpdateReadMe::new();
    update_readme.execute();

    Ok(())
}

async fn scan_players() -> Result<(), Box<dyn Error>> {
    let mut next_player = NextPlayer(0);
    let mut rng = rand::thread_rng();
    let mut updated = 0;
    for i in 0..7 {
        let random_id: u32 = rng.gen_range(std::ops::Range {
            start: 300 * i,
            end: 300 * (i + 1),
        });
        let pid = i + next_player.get();
        let player = snooker::get_player(usize::try_from(pid)?).await;
        if player.is_ok() {
            let pl = player.unwrap();
            tables::add_player(&pl)?;
            updated += 1;
            if !pl.nation.is_empty() {
                let country = Country::from(pl.nation);
                //add_country(&country)?;
            }
        }
    }
    let next_id = next_player.get();
    next_player.put(next_id + updated)?;
    //vec!["Aruba"]
    //    .into_iter()
    //    .map(|it| Country::from(it.to_string()))
    //    .for_each(|it| add_country(&it).ok().unwrap_or_default());
    Ok(())
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("main.log")?)
        .apply()?;
    Ok(())
}
