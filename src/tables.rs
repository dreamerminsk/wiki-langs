use crate::snooker::{EventLink, PlayerLink};
use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub struct PlayerTable {
    folder: String,
}

pub fn add_player(plink: &PlayerLink) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./players/")?;
    let source_name = format!("./players/{}", get_id_segment(plink.snooker_id));
    let temp_name = format!("./players/{}.csv", Uuid::new_v4());
    if Path::new(&source_name).exists() {
        {
            let mut source_reader = csv::Reader::from_path(&source_name)?;
            let mut temp_writer = csv::Writer::from_path(&temp_name)?;
            let mut saved = false;
            for link in source_reader.deserialize() {
                let link: PlayerLink = link?;
                if saved {
                    temp_writer.serialize(link)?;
                } else {
                    match link.cmp(plink) {
                        Ordering::Greater => {
                            temp_writer.serialize(plink)?;
                            temp_writer.serialize(link)?;
                            saved = true;
                        }
                        Ordering::Less => temp_writer.serialize(link)?,
                        Ordering::Equal => {
                            match link.full_name.len().cmp(&plink.full_name.len()) {
                                Ordering::Greater => temp_writer.serialize(link)?,
                                _ => temp_writer.serialize(plink)?,
                            }
                            saved = true;
                        }
                    }
                }
            }
            if !saved {
                temp_writer.serialize(plink)?;
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

pub fn add_event(elink: &EventLink) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./events/")?;
    let source_name = format!("./events/{}", get_id_segment(elink.snooker_id));
    let temp_name = format!("./events/{}.csv", Uuid::new_v4());
    if Path::new(&source_name).exists() {
        {
            let mut source_reader = csv::Reader::from_path(&source_name)?;
            let mut temp_writer = csv::Writer::from_path(&temp_name)?;
            let mut saved = false;
            for link in source_reader.deserialize() {
                let link: EventLink = link?;
                if saved {
                    temp_writer.serialize(link)?;
                } else {
                    match link.cmp(elink) {
                        Ordering::Greater => {
                            temp_writer.serialize(elink)?;
                            temp_writer.serialize(link)?;
                            saved = true;
                        }
                        Ordering::Less => temp_writer.serialize(link)?,
                        Ordering::Equal => {
                            match link.title.len().cmp(&elink.title.len()) {
                                Ordering::Greater => temp_writer.serialize(link)?,
                                _ => temp_writer.serialize(elink)?,
                            }
                            saved = true;
                        }
                    }
                }
            }
            if !saved {
                temp_writer.serialize(elink)?;
            }
            temp_writer.flush()?;
        }
        fs::remove_file(&source_name)?;
        fs::rename(&temp_name, &source_name)?;
    } else {
        let mut source_writer = csv::Writer::from_path(&source_name)?;
        source_writer.serialize(elink)?;
        source_writer.flush()?;
    }
    Ok(())
}

fn get_id_segment(id: u32) -> String {
    format!("ids.{:0>4}.csv", (100 * (id / 100) + 99).to_string())
}
