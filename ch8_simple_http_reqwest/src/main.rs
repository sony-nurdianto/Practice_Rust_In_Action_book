use std::error::Error;

use reqwest::{self, Response};

async fn get_reqwest() -> Result<String, Box<dyn Error>> {
    let url = "https://dtnbot.online/welcome";
    let respone: Response = reqwest::get(url).await?;
    let content = respone.text().await?;
    Ok(content)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let content = get_reqwest().await?;

    println!("{}", content);
    Ok(())
}
