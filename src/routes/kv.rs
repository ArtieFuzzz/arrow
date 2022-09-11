use crate::lib::{
    surrealdb::{get_kv, set_kv},
    KVMessage,
};
use rocket::{http::Status, response::status, serde::json::Json};

#[post("/<key>?<value>")]
pub async fn kv_post<'a>(key: &'a str, value: &'a str) -> status::Custom<Json<KVMessage>> {
    let res = set_kv(key, value).await.unwrap();

    status::Custom(
        Status::Created,
        Json(KVMessage {
            key: key.to_owned(),
            value: res,
        }),
    )
}

#[get("/<key>")]
pub async fn kv_get<'a>(key: &'a str) -> status::Custom<Json<KVMessage>> {
    let res = get_kv(key).await.unwrap();

    status::Custom(
        Status::Ok,
        Json(KVMessage {
            key: key.to_owned(),
            value: res,
        }),
    )
}
