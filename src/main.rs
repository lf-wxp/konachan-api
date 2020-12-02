#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod lib;
mod conf;

use rocket::http::Status;
use lib:: {get_post, ApiResponse };
use conf::API;

#[get("/post/<page>")]
fn post(page: i8) -> ApiResponse {
    match get_post(API, page) {
        Ok(data) => ApiResponse {
            json: json!(data),
            status: Status::Ok,
        },
        Err(_) => ApiResponse {
            json: json!({ "error": "internal error"}),
            status: Status::UnprocessableEntity,
        },
    }
}

fn main() {
    rocket::ignite().mount("/", routes![post]).launch();
}
