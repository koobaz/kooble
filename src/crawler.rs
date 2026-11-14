use std::hash::Hash;
use spider::{self, CaseInsensitiveString, hashbrown::HashSet, website::Website};
use tokio;

pub async fn crawl( 
    mut website_zero: Website,
) -> Result<HashSet<CaseInsensitiveString>, Box<dyn std::error::Error>> {
    website_zero.crawl().await;
    let links = website_zero.set_on_link_find();
    println!("size {:?}" , links);
    Ok(links)
}

