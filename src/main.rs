extern crate reqwest;
extern crate select;

const RANKINGS: &str = "http://www.snooker.org/res/index.asp?template=31&season=2021";
const SEEDINGS: &str = "http://www.snooker.org/res/index.asp?template=32&season=2021";
const SEASONPOINTS: &str = "http://www.snooker.org/res/index.asp?template=33&season=2021";

const PLAYER: &str = "http://www.snooker.org/res/index.asp?player=";
const EVENT: &str = "http://www.snooker.org/res/index.asp?event=";

const RESULTS: &str = "http://www.snooker.org/res/index.asp?template=22&season=2021";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let content = reqwest::get(SEASONPOINTS).await?.text().await?;
    println!("{:?}", content);
    Ok(())
}
