use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use std::env;

pub struct CORS; 

#[rocket::async_trait]
impl Fairing for CORS {
  fn info(&self) -> Info {
    Info {
      name: "Add CORS headers to responses",
      kind: Kind::Response,
    }
  }

  async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
    let origin = env::var("CORS_ORIGIN").unwrap_or("".to_string());
    if !origin.is_empty() {
      response.set_header(Header::new("Access-Control-Allow-Origin", origin));
      response.set_header(Header::new(
        "Access-Control-Allow-Methods",
        "POST, GET, PATCH, OPTIONS",
      ));
      response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
      response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
  }
}
