#[macro_use]
extern crate rocket;
use rocket::response::status::{BadRequest};
use rocket::serde::json::Json;
use log::{error};

mod conf;
mod lib;
mod fairing;

use conf::API;
use lib::{get_image, get_post, ApiResponse, ImageResponse};

#[get("/post/<page>")]
async fn post(page: i8) -> Json<ApiResponse> {
  match get_post(API, page).await {
    Ok(data) => Json(ApiResponse {
      data: Some(data),
      code: 0,
    }),
    Err(_) => Json(ApiResponse {
      data: None,
      code: 1,
    }),
  }
}

#[get("/image?<url>")]
async fn image(url: String) -> Result<ImageResponse, BadRequest<String>> {
  match get_image(url).await {
    Ok(data) => Ok(ImageResponse { data }),
    Err(err) => {
      error!("error, {:?}", err.status());
      Err(BadRequest(Some(String::from("the image url can't access"))))
    }
  }
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .attach(fairing::CORS)
    .mount("/", routes![post])
    .mount("/", routes![image])
}
