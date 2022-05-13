extern crate reqwest;
extern crate select;



const SEEDINGS  :  &str   =  "http://www.snooker.org/res/index.asp?template=32&season=2021" ; 




#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let content = reqwest::get("http://httpbin.org/range/26")
        .await?
        .text()
        .await?;
    println!("Hello, world!");
    Ok(())
}
