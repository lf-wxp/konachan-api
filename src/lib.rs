use bytes::Bytes;
use reqwest;
use rocket::http::RawStr;
use rocket::response::{self, Responder, Response};
use rocket::{
  http::ContentType,
  serde::{Deserialize, Serialize},
  Request,
};
use roxmltree;
use std::io::Cursor;

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
  id: i32,
  url: String,
  width: i32,
  height: i32,
  preview: String,
  preview_width: i32,
  preview_height: i32,
  sample: String,
  sample_width: i32,
  sample_height: i32,
  tags: String,
  security: bool,
  name: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
  pub data: Option<Post>,
  pub code: u8,
  pub msg: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
  count: i32,
  images: Vec<Image>,
}

pub struct ImageResponse {
  pub data: Bytes,
}

impl<'r> Responder<'r, 'static> for ImageResponse {
  fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
    Response::build()
      .sized_body(self.data.len(), Cursor::new(self.data))
      .header(ContentType::new("content-type", "image/jpeg"))
      .ok()
  }
}

pub async fn get_image(url: String) -> Result<Bytes, reqwest::Error> {
  Ok(reqwest::get(url).await?.bytes().await?)
}

pub fn attr_to_int(e: roxmltree::Node, attr: &str) -> i32 {
  e.attribute(attr)
    .unwrap_or("")
    .to_string()
    .parse::<i32>()
    .unwrap_or(0)
}
pub fn attr_to_string(e: roxmltree::Node, attr: &str) -> String {
  e.attribute(attr).unwrap_or("").to_string()
}

pub async fn get_post(
  url: &str,
  page: String,
  tags: String,
) -> Result<Post, Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let resp = client
    .get(url)
    .query(&[("page", page), ("tags", tags)])
    .send()
    .await?
    .text()
    .await?;
  Ok(parse(resp.to_string()))
}

pub fn parse(xml: String) -> Post {
  let doc = roxmltree::Document::parse(&xml).unwrap();
  let elem = doc.descendants();
  let mut count = 0;
  let mut images: Vec<Image> = vec![];
  for e in elem {
    match e.tag_name().name() {
      "posts" => {
        count = attr_to_int(e, "count");
      }
      "post" => {
        let url = e.attribute("file_url").unwrap_or("");
        let encoded_name = url.split("/").last().unwrap_or("");
        let name = RawStr::new(encoded_name).url_decode().unwrap().to_string();
        images.push(Image {
          id: attr_to_int(e, "id"),
          url: url.to_string(),
          width: attr_to_int(e, "width"),
          height: attr_to_int(e, "height"),
          preview: attr_to_string(e, "preview_url"),
          preview_width: attr_to_int(e, "preview_width"),
          preview_height: attr_to_int(e, "preview_height"),
          sample: attr_to_string(e, "sample_url"),
          sample_width: attr_to_int(e, "sample_width"),
          sample_height: attr_to_int(e, "sample_height"),
          tags: attr_to_string(e, "tags"),
          security: attr_to_string(e, "rating") == "s",
          name,
        });
      }
      _ => {}
    }
  }
  Post { count, images }
}
