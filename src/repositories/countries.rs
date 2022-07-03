use crate::snooker::{EventLink, Player, PlayerLink};
use chrono::{Datelike, NaiveDate};
use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use std::path::Path;
use uuid::Uuid;
use crate::domain::entities::Country;




pub fn add_country(country: &Country) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./countries/")?;
    let source_name = format!("./countries/{}","names.eng.csv");
    if Path::new(&source_name).exists() {
        update_country_segment(&source_name, country)?;
    } else {
        let mut source_writer = csv::Writer::from_path(&source_name)?;
        source_writer.serialize(country)?;
        source_writer.flush()?;
    }
    Ok(())
}

fn update_country_segment(segment: &str, country: &Country) -> Result<(), Box<dyn Error>> {
    let temp_name = format!("./countries/{}.csv", Uuid::new_v4());
    {
        let mut source_reader = csv::Reader::from_path(&segment)?;
        let mut temp_writer = csv::Writer::from_path(&temp_name)?;
        let mut saved = false;
        for c in source_reader.deserialize() {
            let c: Country = c?;
            if saved {
                temp_writer.serialize(c)?;
            } else {
                match c.cmp(country) {
                    Ordering::Greater => {
                        temp_writer.serialize(country)?;
                        temp_writer.serialize(c)?;
                        saved = true;
                    }
                    Ordering::Less => temp_writer.serialize(c)?,
                    Ordering::Equal => {
                        temp_writer.serialize(&c)?;
                        saved = false;
                    }
                }
            }
        }
        if !saved {
            temp_writer.serialize(country)?;
        }
        temp_writer.flush()?;
    }
    fs::remove_file(&segment)?;
    fs::rename(&temp_name, &segment)?;
    Ok(())
}
