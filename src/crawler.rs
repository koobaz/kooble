use std::hash::Hash;

use spider::hashbrown::HashSet;
use ureq;
use scraper::{Html, Selector};
use tokio;
use url::Url;

pub async fn spider(site_zero: String) -> Result<HashSet<String>, Box<dyn std::error::Error>> {

    let body: String = ureq::get(site_zero.clone())
    .header("Example-Header", "header value")
    .header("User-Agent", "KoobleBot/0.1")
    .call()? 
    .body_mut()
    .read_to_string()?;
    println!("ok: {}", body);

    let document = Html::parse_document(&body);
    println!("fuck: {:?}", document);
    let selector = Selector::parse("a[href]").unwrap();
    let mut return_sites : HashSet<String> = HashSet::new();
    let base = Url::parse(&site_zero)?;
    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            let owned_link = base.join(&link.to_string())?;
            return_sites.insert(owned_link.to_string());
            println!("{}", link);
        }
    }
    Ok(return_sites)
}
