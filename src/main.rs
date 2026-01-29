#[macro_use]
extern crate rocket;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use log::error;

mod conf;
mod utils;
mod fairing;
mod guard;

use conf::API;
use utils::{get_image, get_post, ApiResponse, ImageResponse};

#[get("/post?<page>&<limit>&<tags>")]
async fn post(page: String, limit: String, tags: String, _key: guard::ApiKey<'_>) -> Json<ApiResponse> {
  match get_post(API, &page, &limit, &tags).await {
    Ok(data) => Json(ApiResponse {
      data: Some(data),
      msg: None,
      code: 0,
    }),
    Err(err) => Json(ApiResponse {
      data: None,
      code: 1,
      msg: Some(err.to_string()),
    }),
  }
}

#[get("/image?<url>")]
async fn image(url: String) -> Result<ImageResponse, BadRequest<String>> {
  match get_image(&url).await {
    Ok(data) => Ok(ImageResponse { data }),
    Err(err) => {
      error!("error, {:?}", err.status());
      Err(BadRequest(String::from("the image url can't access")))
    }
  }
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .attach(fairing::Cors)
    .mount("/", routes![post])
    .mount("/", routes![image])
}
