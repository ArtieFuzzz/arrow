use serde::{Serialize, Deserialize};

pub type KeyValue = Vec<KeyValueElement>;

#[derive(Serialize, Deserialize)]
pub struct KeyValueElement {
    #[serde(rename = "time")]
    pub time: String,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "result")]
    pub result: Vec<Result>,
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "value")]
    pub value: String,
}