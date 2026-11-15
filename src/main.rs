use std::collections::VecDeque;
use std::thread;
use std::time::Duration;
use cliclack::{self, intro, input};
mod crawler;
use tokio;

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sites_list_to_visit :VecDeque<String> = VecDeque::new(); //variable holds the value of the site desired to fetch first as a starter page
    let desired_site_to_fetch: String = input("What is the meaning of life?") //this is an input by user to the starter page
    .placeholder("Not sure")
    .validate(|input: &String| {
        if input.is_empty() {
            Err("Value is required!")
        } else {
            Ok(())
        }
    })
    .interact()?;
    sites_list_to_visit.push_back(desired_site_to_fetch);
    thread::sleep(Duration::from_millis(500));
    while let Some(url) = sites_list_to_visit.pop_front() {
        let sites = crawler::spider(url).await?;
        for site in sites {
            println!("-{}", site);
            sites_list_to_visit.push_back(site);
        }
    }
    Ok(())
}