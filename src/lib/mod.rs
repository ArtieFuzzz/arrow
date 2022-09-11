pub mod surrealdb;
pub mod tenor;
mod types;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct KVMessage {
    pub key: String,
    pub value: String,
}
#[derive(Serialize, Deserialize)]

pub struct Message {
    pub message: String,
}
