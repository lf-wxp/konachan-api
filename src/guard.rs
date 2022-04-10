use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use crate::conf::API_KEY;

#[derive(Debug)]
pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
  Missing,
  Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
  type Error = ApiKeyError;

  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    fn is_valid(key: &str) -> bool {
      key == API_KEY
    }

    match req.headers().get_one("x-api-key") {
      None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
      Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
      Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
    }
  }
}
