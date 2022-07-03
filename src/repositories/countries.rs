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
                        temp_writer.serialize(&p)?;
                        match p.snooker_id.cmp(&player.snooker_id) {
                            Ordering::Equal => saved = true,
                            _ => saved = false,
                        }
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
