mod crawler;
use std::{collections::VecDeque, hash::Hash};
use cliclack;
use scraper::CaseSensitivity::CaseSensitive;
use spider::{self, hashbrown::HashSet, website::{self, Website}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut visited : HashSet<String> = HashSet::new();
    visited.insert(String::from("https://en.wikipedia.org/wiki/Thought_experiment"));
    let mut sites_crawl :VecDeque<Website> = VecDeque::new();
    sites_crawl.push_back(Website::new("https://en.wikipedia.org/wiki/Thought_experiment"));
    while let Some(url) = sites_crawl.pop_front() {
        let spinner = cliclack::spinner();
        let new_links = crawler::crawl(url).await?;
        for link in new_links {
            if visited.insert(link.to_string()) {
                link.to_string();
                println!("-{}", link);
                sites_crawl.push_back(Website::new(&link));

            }
        }


    }
    Ok(())
}