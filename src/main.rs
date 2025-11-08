use crate::country::{entities::Country, tables::add_country};
use app::tasks::{DupBirthdays, DupNames, NationStats, SoRanking, UpdateCountries, UpdateReadMe};
use chrono::{Datelike, Utc};
use fern;
use log::{debug, error, info, trace, warn};
use rand::Rng;
use std::fs;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
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

static README_PATH: &str = "./README.md";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_logger()?;

    scan_players().await;

    match get_file_last_modified(README_PATH) {
        Ok(timediff) => {
             info!("README.md timediff: {}", timediff);
            if timediff >= 86400 {
                let update_readme = UpdateReadMe::new();
                update_readme.execute();
            }
        }
        Err(e) => {
            eprintln!("Error getting last modified time: {}", e);
        }
    }

    let today = Utc::now();

    if today.day() % 2 == 0 {
        let dup_names = DupNames::new();
        dup_names.execute();

        let dup_birthdays = DupBirthdays::new();
        dup_birthdays.execute();

        let nation_stats = NationStats::new();
        nation_stats.execute();
    }

    if today.day() % 3 == 0 {
        let so_ranking = SoRanking::new();
        so_ranking.execute().await?;
    }

    Ok(())
}

fn get_file_last_modified(path: &str) -> Result<u64, Box<dyn Error>> {
    let metadata = fs::metadata(path)?;
    let modified_time = metadata.modified()?;
    let now = SystemTime::now();
    let duration = if now >= modified_time {
        now.duration_since(modified_time)?.as_secs()
    } else {
        0
    };
    Ok(duration)
}

async fn scan_players() -> Result<(), Box<dyn Error>> {
    let mut next_player = NextPlayer(0);
    let mut rng = rand::rng();
    let mut updated = 0;
    for i in 0..11 {
        let random_id: u32 = rng.random_range(std::ops::Range {
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
        thread::sleep(Duration::from_secs(4));
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
