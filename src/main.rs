#[macro_use]
extern crate rocket;
use rocket::response::status::NotFound;
use rocket::{serde::json::Json};

mod conf;
mod lib;

use conf::API;
use lib::{get_post, ApiResponse, get_image, ImageResponse};

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
async fn image(url: String) -> Result<ImageResponse, NotFound<String>> {
    match get_image(url).await {
        Ok(data) => Ok(ImageResponse { data }),
        Err(err) => { 
            print!("error, {:?}", err.status());
            Err(NotFound(String::from("error")))
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![post])
     .mount("/", routes![image])
}
