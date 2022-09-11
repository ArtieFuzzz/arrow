use super::types;
use crate::HTTP;
use color_eyre::{eyre::eyre, Result};
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

pub async fn get_kv(key: &str) -> Result<String> {
    let db = get_info()?;

    let res: _ = HTTP
        .post(db.0)
        .basic_auth(db.1, Some(db.2))
        .header("NS", "default")
        .header("DB", "default")
        .header("Content-Type", "application/json")
        .body(format!("SELECT * FROM kv WHERE key = \"{key}\""))
        .send()
        .await?
        .json::<types::KeyValue>()
        .await?;

    Ok(res[0].result[0].value.clone())
}

pub async fn set_kv(key: &str, value: &str) -> Result<String> {
    let db = get_info()?;

    let res: _ = HTTP
        .post(db.0)
        .basic_auth(db.1, Some(db.2))
        .header("NS", "default")
        .header("DB", "default")
        .header("Content-Type", "application/json")
        .body(format!("CREATE kv SET key = {key}, value = {value}"))
        .send()
        .await?
        .json::<types::KeyValue>()
        .await?;

    Ok(res[0].result[0].value.clone())
}
