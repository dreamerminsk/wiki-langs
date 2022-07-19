use crate::country::entities::Country;
use crate::repositories::countries;
use anyhow::Result;
use rand::Rng;
use std::error::Error;
mod country;

mod repositories;

mod snooker;

mod tables;

mod services;

mod wiki;

#[tokio::main]
async fn main() -> Result<()> {
    let mut rng = rand::thread_rng();
    for i in 0..7 {
        let random_id: u32 = rng.gen_range(std::ops::Range {
            start: 200 * i,
            end: 200 * (i + 1),
        });
        let player = snooker::get_player(usize::try_from(random_id)?).await?;
        tables::add_player(&player)?;
        let country = Country::from(player.nation);
        countries::add_country(&country)?;
    }
    vec!["Australia"]
        .into_iter()
        .map(|it| Country::from(it.to_string()))
        .for_each(|it| countries::add_country(&it).ok().unwrap_or_default());

    Ok(())
}
