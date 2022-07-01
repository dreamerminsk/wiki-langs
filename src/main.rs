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

    Ok(())
}
