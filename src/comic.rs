use crate::error::XKCDResult;
use serde::Deserialize;

static INFO_JSON: &str = "info.0.json";
static BASE_URL: &str = "https://xkcd.com";
static LATEST_URL: &str = "https://xkcd.com/info.0.json";

/// Returns the latest comic
pub async fn get_latest_comic() -> XKCDResult<Comic> {
    retrieve_comic(LATEST_URL.to_string()).await
}

/// Returns a comic for an ID
pub async fn get_comic(id: u32) -> XKCDResult<Comic> {
    retrieve_comic(format!("{}/{}/{}", BASE_URL, id, INFO_JSON)).await
}

async fn retrieve_comic(url: String) -> XKCDResult<Comic> {
    let response = reqwest::get(url).await?;
    let response = response.json::<Comic>().await?;

    Ok(response)
}

#[derive(Deserialize)]
pub struct Comic {
    pub day: String,
    pub month: String,
    pub year: String,
    pub num: u32,
    pub safe_title: String,
    pub transcript: String,
    pub alt: String,
    pub img: String,
    pub title: String,
}
