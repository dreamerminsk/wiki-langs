extern crate reqwest;
extern crate select;



#[tokio::main]
async fn main()   -> Result<(), reqwest::Error> {
    let content = reqwest::get("http://httpbin.org/range/26")
        .await?
        .text()
        .await?;
    println!("Hello, world!");
    Ok(())
}
