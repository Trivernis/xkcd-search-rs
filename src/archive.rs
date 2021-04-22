use crate::error::XKCDResult;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::iter::FromIterator;

static ARCHIVE_URL: &str = "https://xkcd.com/archive/";

/// Returns the xkcd archive list
pub async fn get_archive() -> XKCDResult<HashMap<String, u32>> {
    let response = reqwest::get(ARCHIVE_URL).await?;
    let html = response.text().await?;

    parse_archive_list(html)
}

fn parse_archive_list(html: String) -> XKCDResult<HashMap<String, u32>> {
    let document = Html::parse_document(&html);
    let archive_selector = Selector::parse(r#"#middleContainer > a"#).unwrap();
    let archive = HashMap::from_iter(document.select(&archive_selector).filter_map(|element| {
        Some((
            element.inner_html(),
            element
                .value()
                .attr("href")?
                .replace("/", "")
                .parse::<u32>()
                .ok()?,
        ))
    }));

    Ok(archive)
}
