use rocket::{http::Status, response::status};
use crate::lib::tenor;

#[get("/tenor?<tags>")]
pub async fn tenor_gif(tags: &str) -> status::Custom<String> {
  if tags.is_empty() {
    return status::Custom(Status::BadRequest, "You must specify the `tags` query".to_string())
  }

  let url = tenor::grab(tags).await.unwrap().clone();

  status::Custom(Status::Ok, url)
}