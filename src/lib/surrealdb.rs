use super::types;
use crate::HTTP;
use color_eyre::{eyre::eyre, Result};
use reqwest::RequestBuilder;
use std::env;

fn get_info() -> Result<(String, String, String)> {
    // skipcq: RS-W1015
    let db_url = match env::var("ARROW_SURREALDB_URL") {
        Ok(url) => url,
        Err(_why) => return Err(eyre!("Environment variable ARROW_SURREALDB_URL is not set")),
    };

    // skipcq: RS-W1015
    let username = match env::var("ARROW_SURREALDB_USER") {
        Ok(url) => url,
        Err(_why) => {
            return Err(eyre!(
                "Environment variable ARROW_SURREALDB_USER is not set"
            ))
        }
    };

    // skipcq: RS-W1015
    let password = match env::var("ARROW_SURREALDB_PWD") {
        Ok(url) => url,
        Err(_why) => return Err(eyre!("Environment variable ARROW_SURREALDB_PWD is not set")),
    };

    Ok((db_url, username, password))
}

fn base() -> RequestBuilder {
    let auth = get_info().unwrap();

    return HTTP
        .post(auth.0)
        .basic_auth(auth.1, Some(auth.2))
        .header("NS", "default")
        .header("DB", "default")
        .header("Content-Type", "application/json");
}

pub async fn get_kv(key: &str) -> Result<String> {
    let res = base()
        .body(format!("SELECT * FROM kv WHERE key = \"{key}\""))
        .send()
        .await?
        .json::<types::KeyValue>()
        .await?;

    if res[0].result.is_empty() {
      return Err(eyre!("Key is non-existent"))
    }

    Ok(res[0].result[0].value.clone())
}

pub async fn set_kv(key: &str, value: &str) -> Result<String> {
    let res = base()
        .body(format!("CREATE kv SET key = \"{key}\", value = {value}"))
        .send()
        .await?
        .json::<types::KeyValue>()
        .await?;

    Ok(res[0].result[0].value.clone())
}
pub async fn delete_kv(key: &str) -> Result<bool> {
    let res: _ = base()
        .body(format!("DELETE kv WHERE key = \"{key}\" RETURN BEFORE"))
        .send()
        .await?
        .json::<types::KeyValue>()
        .await?;

    if res[0].result.is_empty() {
        return Ok(false)
    }

    Ok(true)
}

pub async fn update_kv(key: &str, value: &str) -> Result<String> {
    let res = base()
        .body(format!(
            "UPDATE kv SET value = {value} WHERE key = \"{key}\""
        ))
        .send()
        .await?
        .json::<types::KeyValue>()
        .await?;

    Ok(res[0].result[0].value.clone())
}
