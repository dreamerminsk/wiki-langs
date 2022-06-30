use crate::snooker::{EventLink, Player, PlayerLink};
use chrono::{Datelike, NaiveDate};
use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub struct PlayerTable {
    folder: String,
}

pub fn add_player(player: &Player) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./players/")?;
    let source_name = format!("./players/{}", get_year_segment(player.birthday));
    if Path::new(&source_name).exists() {
        update_player_segment(&source_name, player)?;
    } else {
        let mut source_writer = csv::Writer::from_path(&source_name)?;
        source_writer.serialize(player)?;
        source_writer.flush()?;
    }
    Ok(())
}

fn update_player_segment(segment: &str, player: &Player) -> Result<(), Box<dyn Error>> {
    let temp_name = format!("./players/{}.csv", Uuid::new_v4());
    {
        let mut source_reader = csv::Reader::from_path(&segment)?;
        let mut temp_writer = csv::Writer::from_path(&temp_name)?;
        let mut saved = false;
        for p in source_reader.deserialize() {
            let p: Player = p?;
            if saved {
                temp_writer.serialize(p)?;
            } else {
                match p.cmp(player) {
                    Ordering::Greater => {
                        temp_writer.serialize(player)?;
                        temp_writer.serialize(p)?;
                        saved = true;
                    }
                    Ordering::Less => temp_writer.serialize(p)?,
                    Ordering::Equal => {
                        temp_writer.serialize(p)?;
                        saved = true;
                    }
                }
            }
        }
        if !saved {
            temp_writer.serialize(player)?;
        }
        temp_writer.flush()?;
    }
    fs::remove_file(&segment)?;
    fs::rename(&temp_name, &segment)?;
    Ok(())
}

fn get_year_segment(date: NaiveDate) -> String {
    format!("bys.{:0>4}.csv", (date.year()).to_string())
}

pub struct PlayerLinkTable {
    folder: String,
}

pub fn add_player_link(plink: &PlayerLink) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./player-links/")?;
    let source_name = format!("./player-links/{}", get_id_segment(plink.snooker_id));
    if Path::new(&source_name).exists() {
        update_segment(&source_name, plink)?;
    } else {
        let mut source_writer = csv::Writer::from_path(&source_name)?;
        source_writer.serialize(plink)?;
        source_writer.flush()?;
    }
    Ok(())
}

fn update_segment(segment: &str, pl: &PlayerLink) -> Result<(), Box<dyn Error>> {
    let temp_name = format!("./player-links/{}.csv", Uuid::new_v4());
    {
        let mut source_reader = csv::Reader::from_path(&segment)?;
        let mut temp_writer = csv::Writer::from_path(&temp_name)?;
        let mut saved = false;
        for link in source_reader.deserialize() {
            let link: PlayerLink = link?;
            if saved {
                temp_writer.serialize(link)?;
            } else {
                match link.cmp(pl) {
                    Ordering::Greater => {
                        temp_writer.serialize(pl)?;
                        temp_writer.serialize(link)?;
                        saved = true;
                    }
                    Ordering::Less => temp_writer.serialize(link)?,
                    Ordering::Equal => {
                        match link.full_name.len().cmp(&pl.full_name.len()) {
                            Ordering::Greater => temp_writer.serialize(link)?,
                            _ => temp_writer.serialize(pl)?,
                        }
                        saved = true;
                    }
                }
            }
        }
        if !saved {
            temp_writer.serialize(pl)?;
        }
        temp_writer.flush()?;
    }
    fs::remove_file(&segment)?;
    fs::rename(&temp_name, &segment)?;
    Ok(())
}

pub struct EventTable {
    folder: String,
}

pub struct EventLinkTable {
    folder: String,
}

pub fn add_event(elink: &EventLink) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./event-links/")?;
    let source_name = format!("./event-links/{}", get_id_segment(elink.snooker_id));
    if Path::new(&source_name).exists() {
        update_esegment(&source_name, elink)?;
    } else {
        let mut source_writer = csv::Writer::from_path(&source_name)?;
        source_writer.serialize(elink)?;
        source_writer.flush()?;
    }
    Ok(())
}

fn update_esegment(segment: &str, el: &EventLink) -> Result<(), Box<dyn Error>> {
    let temp_name = format!("./event-links/{}.csv", Uuid::new_v4());
    {
        let mut source_reader = csv::Reader::from_path(&segment)?;
        let mut temp_writer = csv::Writer::from_path(&temp_name)?;
        let mut saved = false;
        for link in source_reader.deserialize() {
            let link: EventLink = link?;
            if saved {
                temp_writer.serialize(link)?;
            } else {
                match link.cmp(el) {
                    Ordering::Greater => {
                        temp_writer.serialize(el)?;
                        temp_writer.serialize(link)?;
                        saved = true;
                    }
                    Ordering::Less => temp_writer.serialize(link)?,
                    Ordering::Equal => {
                        match link.title.len().cmp(&el.title.len()) {
                            Ordering::Greater => temp_writer.serialize(link)?,
                            _ => temp_writer.serialize(el)?,
                        }
                        saved = true;
                    }
                }
            }
        }
        if !saved {
            temp_writer.serialize(el)?;
        }
        temp_writer.flush()?;
    }
    fs::remove_file(&segment)?;
    fs::rename(&temp_name, &segment)?;
    Ok(())
}

fn get_id_segment(id: usize) -> String {
    format!("ids.{:0>4}.csv", (100 * (id / 100) + 99).to_string())
}
