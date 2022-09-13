use crate::lib::{
    surrealdb::{delete_kv, get_kv, set_kv, update_kv},
    KVMessage, Message,
};
use rocket::{http::Status, response::status, serde::json::Json};
use serde_json::{json, Value};

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

#[patch("/<key>?<value>")]
pub async fn kv_update<'a>(key: &'a str, value: &'a str) -> status::Custom<Json<KVMessage>> {
    let res = update_kv(key, value).await.unwrap();

    status::Custom(
        Status::Ok,
        Json(KVMessage {
            key: key.to_owned(),
            value: res,
        }),
    )
}

#[delete("/<key>")]
pub async fn kv_delete<'a>(key: &'a str) -> status::Custom<Json<Message>> {
    let deleted = match delete_kv(key).await {
        Ok(v) => v,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                Json(Message {
                    message: "The key cannot be found in the database...".to_string(),
                }),
            )
        }
    };

    if !deleted {
        return status::Custom(
            Status::Ok,
            Json(Message {
                message: "Key cannot be found...".to_string(),
            }),
        );
    }

    status::Custom(
        Status::Ok,
        Json(Message {
            message: "Deleted".to_string(),
        }),
    )
}

#[get("/<key>")]
pub async fn kv_get<'a>(key: &'a str) -> status::Custom<Json<Value>> {
    let res = match get_kv(key).await {
        Ok(v) => v,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                Json(json!({ "message": "Key does not exist" })),
            )
        }
    };

    status::Custom(
        Status::Ok,
        Json(json!({ "key": key.to_string(), "value": res})),
    )
}
