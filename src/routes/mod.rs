pub mod image;
pub mod kv;

use crate::lib::Message;
use rocket::{http::Status, response::status, serde::json::Json};

#[get("/")]
pub fn root() -> status::Custom<Json<Message>> {
    status::Custom(
        Status::Ok,
        Json(Message {
            message: "Hello there traveler.".to_string(),
        }),
    )
}
