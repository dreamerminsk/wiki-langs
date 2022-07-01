use rand::Rng;
use std::error::Error;

mod html;

mod snooker;

mod tables;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let random_id: u32 = rng.gen_range(1..300);
    let player = snooker::get_player(usize::try_from(random_id)?).await?;
    tables::add_player(&player)?;
    (0..3)
        .map(|_| rng.gen_range(1..300))
        .filter_map(|r| usize::try_from(r).ok())
        .filter_map(move |id| async move { snooker::get_player(id).await.unwrap().ok() })
        .for_each(|p| {tables::add_player(&p);Ok(()));

    Ok(())
}
