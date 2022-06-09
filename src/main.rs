use crate::snooker::{EventLink, PlayerLink};
use csv;
use std::collections::BTreeSet;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::path::Path;
use uuid::Uuid;

mod html;
mod snooker;

fn add_player(plink: &PlayerLink) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./players/")?;
    let source_name = format!(
        "./players/ids.{:0>4}.csv",
        (100 * (plink.snooker_id / 100) + 99).to_string()
    );
    let temp_name = format!("./players/{}.csv", Uuid::new_v4().to_string());
    if Path::new(&source_name).exists() {
        {
            let mut source_reader = csv::Reader::from_path(&source_name)?;
            let mut temp_writer = csv::Writer::from_path(&temp_name)?;
            let saved = false;
            for link in source_reader.deserialize() {
                let link: PlayerLink = link?;
                if saved {
                    temp_writer.serialize(link)?;
                } else {
                    if link < plink {
                        temp_writer.serialize(link)?;
                    } else if link == plink {
                        temp_writer.serialize(plink)?;
                        saved = true;
                    } else {
                        temp_writer.serialize(plink)?;
                        temp_writer.serialize(link)?;
                        saved = true;
                    }
                }
            }
            temp_writer.flush()?;
        }
        fs::remove_file(&source_name)?;
        fs::rename(&temp_name, &source_name)?;
    } else {
        let mut source_writer = csv::Writer::from_path(&source_name)?;
        source_writer.serialize(plink)?;
        source_writer.flush()?;
    }
    Ok(())
}

fn add_event(elink: &EventLink) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./events/")?;
    let source_name = format!(
        "./events/ids.{:0>4}.csv",
        (100 * (elink.snooker_id / 100) + 99).to_string()
    );
    let temp_name = format!("{}.csv", Uuid::new_v4().to_string());
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
