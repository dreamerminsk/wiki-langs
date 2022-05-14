extern crate reqwest;
extern crate select;

mod snooker;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let content = reqwest::get(snooker::SEASONPOINTS).await?.text().await?;
    println!("{:?}", content);
    Ok(())
}
