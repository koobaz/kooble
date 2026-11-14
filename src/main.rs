use cliclack::{self, intro, input};
mod crawler;
use tokio;

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let desired_site_to_fetch: String = input("What is the meaning of life?")
    .placeholder("Not sure")
    .validate(|input: &String| {
        if input.is_empty() {
            Err("Value is required!")
        } else {
            Ok(())
        }
    })
    .interact()?;
    let sites = crawler::spider(desired_site_to_fetch).await?;
    intro("Sites found");
    for site in sites {
        println!("-{}", site)
    }
    Ok(())
}