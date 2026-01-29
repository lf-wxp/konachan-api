use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use std::env;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
  fn info(&self) -> Info {
    Info {
      name: "Add CORS headers to responses",
      kind: Kind::Response,
    }
  }

  async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
    let host = request.headers().get_one("Host").unwrap_or("");
    let env_val = env::var("CORS_ORIGIN").unwrap_or("".to_string());
    let mut origin_iter = env_val.split(",");
    let is_match_host = origin_iter.find(|&s| s == host).unwrap_or("").to_string();
    if !is_match_host.is_empty() {
      response.set_header(Header::new("Access-Control-Allow-Origin", is_match_host));
      response.set_header(Header::new(
        "Access-Control-Allow-Methods",
        "POST, GET, PATCH, OPTIONS",
      ));
      response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
      response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
  }
}
