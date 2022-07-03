use crate::domain::entities::Country;
use crate::repositories::countries;
use rand::Rng;
use std::error::Error;

mod domain;

mod repositories;

mod html;

mod snooker;

mod tables;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    for _i in 1..5 {
        let random_id: u32 = rng.gen_range(1..3000);
        let player = snooker::get_player(usize::try_from(random_id)?).await?;
        tables::add_player(&player)?;
        let country = Country::from(player.nation);
        countries::add_country(&country)?;
    }

    Ok(())
}
