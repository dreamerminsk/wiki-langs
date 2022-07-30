use super::entities::Country;
use std::{cmp::Ordering, error::Error, fs, path::Path};
use uuid::Uuid;

pub fn get_all_countries() -> Result<Vec<Country>, Box<dyn Error>> {
    let source_name = format!("./countries/codes/{}", "en.csv");
    let mut countries = vec![];
    if Path::new(&source_name).exists() {
        let mut source_reader = csv::Reader::from_path(&source_name)?;
        for c in source_reader.deserialize() {
            let c: Country = c?;
            countries.push(c);
        }
    }
    Ok(countries)
}

pub fn add_country(country: &Country) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./countries/codes/")?;
    let source_name = format!("./countries/codes/{}", "en.csv");
    if Path::new(&source_name).exists() {
        update_segment(&source_name, country)?;
    } else {
        let mut source_writer = csv::Writer::from_path(&source_name)?;
        source_writer.serialize(country)?;
        source_writer.flush()?;
    }
    Ok(())
}

fn update_segment(segment: &str, country: &Country) -> Result<(), Box<dyn Error>> {
    let temp_name = format!("./countries/codes/{}.csv", Uuid::new_v4());
    {
        let mut source_reader = csv::Reader::from_path(&segment)?;
        let mut temp_writer = csv::Writer::from_path(&temp_name)?;
        let mut saved = false;
        for c in source_reader.deserialize() {
            let c: Country = c?;
            if c.eq(&country) {
                continue;
            }
            if saved {
                temp_writer.serialize(&c)?;
            } else {
                match c.cmp(&country) {
               
                       
                    Ordering::Less => temp_writer.serialize(&c)?,
                    _ => {
                       temp_writer.serialize(&country)?;
                        temp_writer.serialize(&c)?;
                        saved = true;
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

pub fn remove_country(lang: &str, country: &Country) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./countries/codes/")?;
    let source_name = format!("./countries/codes/{}.csv", lang);
    if Path::new(&source_name).exists() {
        remove_from_segment(&source_name, country)?;
    }
    Ok(())
}

fn remove_from_segment(segment: &str, country: &Country) -> Result<(), Box<dyn Error>> {
    let temp_name = format!("./countries/codes/{}.csv", Uuid::new_v4());
    {
        let mut source_reader = csv::Reader::from_path(&segment)?;
        let mut temp_writer = csv::Writer::from_path(&temp_name)?;
        for c in source_reader.deserialize() {
            let c: Country = c?;
            if !c.eq(country) {
                temp_writer.serialize(&c)?;
            }
        }
        temp_writer.flush()?;
    }
    fs::remove_file(&segment)?;
    fs::rename(&temp_name, &segment)?;
    Ok(())
}
