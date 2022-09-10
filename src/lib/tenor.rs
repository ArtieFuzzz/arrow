use crate::HTTP;
use color_eyre::{eyre::eyre, Result};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

fn parse_url<'a>(tags: &'a str) -> Result<String> {
    // skipcq: RS-W1015
    let api_key = match env::var("ARROW_TENOR_KEY") {
        Ok(key) => key,
        Err(_why) => return Err(eyre!("Environment variable ARROW_TENOR_KEY is not set")),
    };

    Ok(format!("https://tenor.googleapis.com/v2/search?q={tags}&key={api_key}&client_key=arrow&limit=50&media_filter=gif"))
}

pub async fn grab<'a>(tags: &'a str) -> Result<String> {
    let req_url = parse_url(tags)?;

    let res: TenorResponse = HTTP.get(req_url).send().await?.json().await?;
    let url = &res.results.choose(&mut rand::thread_rng()).unwrap().media_formats["gif"].url;

    Ok(url.to_string())
}

#[derive(Serialize, Deserialize)]
pub struct TenorResponse {
    #[serde(rename = "results")]
    results: Vec<Results>,

    #[serde(rename = "next")]
    next: String,
}

#[derive(Serialize, Deserialize)]
pub struct Results {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "media_formats")]
    media_formats: HashMap<String, MediaFormat>,

    #[serde(rename = "created")]
    created: f64,

    #[serde(rename = "content_description")]
    content_description: String,

    #[serde(rename = "itemurl")]
    itemurl: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "tags")]
    tags: Vec<String>,

    #[serde(rename = "flags")]
    flags: Vec<Option<serde_json::Value>>,

    #[serde(rename = "hasaudio")]
    hasaudio: bool,
}

#[derive(Serialize, Deserialize)]
pub struct MediaFormat {
    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "duration")]
    duration: f64,

    #[serde(rename = "preview")]
    preview: String,

    #[serde(rename = "dims")]
    dims: Vec<i64>,

    #[serde(rename = "size")]
    size: i64,
}
