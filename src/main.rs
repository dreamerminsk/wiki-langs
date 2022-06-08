
use crate::snooker::{EventLink, PlayerLink};
use std::collections::BTreeSet;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::BufReader;
use std::path::Path;

mod html;
mod snooker;

fn add_player(plink: &PlayerLink) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./players/")?;
    let source_file = format!(
        "./players/ids.{:0>4}.csv",
        (100 * (plink.snooker_id / 100) + 99).to_string()
    );
    if Path::new(&source_file).exists() {
        {
            let sfile = OpenOptions::new().read(true).open(&source_file);
            let tfile = OpenOptions::new()
                .write(true)
                .create(true)
                .open(format!("{}.temp", &source_file));
        }
        fs::remove_file(&source_file)?;
        fs::rename(format!("{}.temp", &source_file), &source_file)?;
    } else {
        let f = File::create(&source_file)?;
        let buf_reader = BufReader::new(f);
    }
    Ok(())
}

fn add_event(elink: &EventLink) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./events/")?;
    let source_file = format!(
        "./events/ids.{:0>4}.csv",
        (100 * (elink.snooker_id / 100) + 99).to_string()
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get(format!("{}{}", snooker::HOST, snooker::UPCOMING_MATCHES)).await?;
    println!("{:#?}", resp.url().to_string());

    let text = resp.text().await?;

    let urls = html::parse_links(&text);

    let purls = urls
        .iter()
        .filter_map(|u| PlayerLink::try_from(u).ok())
        .collect::<BTreeSet<PlayerLink>>();

    purls.iter().for_each(|p| {
        add_player(p);
    });

    let eurls = urls
        .iter()
        .filter_map(|u| EventLink::try_from(u).ok())
        .collect::<BTreeSet<EventLink>>();

    eurls.iter().for_each(|e| {
        add_event(e);
    });

    Ok(())
}
