extern crate reqwest;
extern crate select;
#[tokio::main]
fn main() {
    let content = reqwest::get("http://httpbin.org/range/26")
        .await?
        .text()
        .await?;
    println!("Hello, world!");
}
