use std::hash::Hash;

use spider::hashbrown::HashSet;
use ureq;
use scraper::{Html, Selector};
use tokio;

pub async fn spider(site_zero: String) -> Result<HashSet<String>, Box<dyn std::error::Error>> {

    let body: String = ureq::get(site_zero)
    .header("Example-Header", "header value")
    .call()? 
    .body_mut()
    .read_to_string()?;
    println!("ok: {}", body);

    let document = Html::parse_document(&body);
    println!("fuck: {:?}", document);
    let selector = Selector::parse("a[href]").unwrap();
    let mut return_sites : HashSet<String> = HashSet::new();
    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            let owned_link = link.to_string();
            return_sites.insert(owned_link);
            println!("{}", link);
        }
    }
    Ok(return_sites)
}
