use crate::country::entities::Country;
use crate::country::tables::add_country;
use app::tasks::UpdateCountries;
use app::tasks::UpdateReadMe;
use rand::Rng;
use std::error::Error;

mod app;

mod country;

mod snooker;

mod tables;

mod wiki;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    for i in 0..7 {
        let random_id: u32 = rng.gen_range(std::ops::Range {
            start: 300 * i,
            end: 300 * (i + 1),
        });
        let player = snooker::get_player(usize::try_from(random_id)?).await?;
        tables::add_player(&player)?;
        if player.nation.len() > 0 {
            let country = Country::from(player.nation);
            //add_country(&country)?;
        }
    }
    //vec!["Australia"]
    //.into_iter()
    //.map(|it| Country::from(it.to_string()))
    //.for_each(|it| add_country(&it).ok().unwrap_or_default());

    let update_countries = UpdateCountries::new();
    update_countries.execute().await;

    let update_readme = UpdateReadMe::new();
    update_readme.execute();

    Ok(())
}
