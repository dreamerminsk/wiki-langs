use reqwest::Client;


static APP_USER_AGENT : &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.63 Safari/537.36 Edg/102.0.1245.33";

lazy_static! {
    static ref CLIENT: Client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();
}


pub async fn get(url:String) -> Result<String, Box<dyn Error>> {
    let resp = CLIENT
        .get(url)
        .send()
        .await?;
resp.text().await?
}
