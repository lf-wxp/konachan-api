use bytes::Bytes;
use rocket::response::{self, Responder, Response};
use rocket::{
  Request,
  http::ContentType,
  serde::{Deserialize, Serialize},
};
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

pub async fn get_image(url: &str) -> Result<Bytes, reqwest::Error> {
  let response = reqwest::get(url).await?;
  let bytes: Bytes = response.bytes().await?;
  Ok(bytes)
}

pub fn attr_to_int(e: roxmltree::Node, attr: &str) -> i32 {
  e.attribute(attr).unwrap_or("").parse::<i32>().unwrap_or(0)
}

pub fn attr_to_string(e: roxmltree::Node, attr: &str) -> String {
  e.attribute(attr).unwrap_or("").to_string()
}

pub async fn get_post(
  url: &str,
  page: &str,
  limit: &str,
  tags: &str,
) -> Result<Post, Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();

  // Use reqwest::Url for proper URL building with encoding
  let mut parsed_url = reqwest::Url::parse(url)?;
  {
    let mut query_pairs = parsed_url.query_pairs_mut();
    query_pairs.append_pair("page", page);
    query_pairs.append_pair("limit", limit);
    query_pairs.append_pair("tags", tags);
  }

  let resp: reqwest::Response = client.get(parsed_url).send().await?;
  let text: String = resp.text().await?;
  Ok(parse(text)?)
}

pub fn parse(xml: String) -> Result<Post, roxmltree::Error> {
  let doc = roxmltree::Document::parse(&xml)?;
  let elem = doc.descendants();
  let mut count = 0;
  let mut images: Vec<Image> = Vec::with_capacity(10); // Pre-allocate capacity

  for e in elem {
    match e.tag_name().name() {
      "posts" => {
        count = attr_to_int(e, "count");
      }
      "post" => {
        let url = e.attribute("file_url").unwrap_or("");
        let encoded_name = url.split('/').next_back().unwrap_or("");
        let name = percent_encoding::percent_decode_str(encoded_name)
          .decode_utf8()
          .unwrap_or_default()
          .to_string();

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

  Ok(Post { count, images })
}

#[cfg(test)]
mod tests {
  use super::*;
  use rocket::serde::json::serde_json;

  fn create_sample_xml() -> String {
    String::from(
      r#"<?xml version="1.0" encoding="UTF-8"?>
<posts count="2">
  <post
    id="12345"
    file_url="https://konachan.net/image/test%20image.jpg"
    width="1920"
    height="1080"
    preview_url="https://konachan.net/preview/test.jpg"
    preview_width="150"
    preview_height="84"
    sample_url="https://konachan.net/sample/test.jpg"
    sample_width="1500"
    sample_height="844"
    tags="tag1 tag2 tag3"
    rating="s"
  />
  <post
    id="67890"
    file_url="https://konachan.net/image/another.png"
    width="1280"
    height="720"
    preview_url="https://konachan.net/preview/another.png"
    preview_width="150"
    preview_height="84"
    sample_url="https://konachan.net/sample/another.png"
    sample_width="850"
    sample_height="478"
    tags="landscape nature"
    rating="q"
  />
</posts>"#,
    )
  }

  fn create_empty_xml() -> String {
    String::from(
      r#"<?xml version="1.0" encoding="UTF-8"?>
<posts count="0">
</posts>"#,
    )
  }

  #[test]
  fn test_parse_returns_correct_count() {
    let xml = create_sample_xml();
    let result = parse(xml).unwrap();
    assert_eq!(result.count, 2);
  }

  #[test]
  fn test_parse_returns_correct_number_of_images() {
    let xml = create_sample_xml();
    let result = parse(xml).unwrap();
    assert_eq!(result.images.len(), 2);
  }

  #[test]
  fn test_parse_parses_first_image_correctly() {
    let xml = create_sample_xml();
    let result = parse(xml).unwrap();
    let first_image = &result.images[0];

    assert_eq!(first_image.id, 12345);
    assert_eq!(first_image.url, "https://konachan.net/image/test%20image.jpg");
    assert_eq!(first_image.width, 1920);
    assert_eq!(first_image.height, 1080);
    assert_eq!(first_image.preview, "https://konachan.net/preview/test.jpg");
    assert_eq!(first_image.preview_width, 150);
    assert_eq!(first_image.preview_height, 84);
    assert_eq!(first_image.sample, "https://konachan.net/sample/test.jpg");
    assert_eq!(first_image.sample_width, 1500);
    assert_eq!(first_image.sample_height, 844);
    assert_eq!(first_image.tags, "tag1 tag2 tag3");
    assert!(first_image.security);
    assert_eq!(first_image.name, "test image.jpg");
  }

  #[test]
  fn test_parse_parses_second_image_correctly() {
    let xml = create_sample_xml();
    let result = parse(xml).unwrap();
    let second_image = &result.images[1];

    assert_eq!(second_image.id, 67890);
    assert_eq!(second_image.width, 1280);
    assert_eq!(second_image.height, 720);
    assert_eq!(second_image.tags, "landscape nature");
    assert!(!second_image.security);
    assert_eq!(second_image.name, "another.png");
  }

  #[test]
  fn test_parse_handles_empty_posts() {
    let xml = create_empty_xml();
    let result = parse(xml).unwrap();

    assert_eq!(result.count, 0);
    assert!(result.images.is_empty());
  }

  #[test]
  fn test_parse_handles_invalid_xml() {
    let invalid_xml = String::from("not valid xml");
    let result = parse(invalid_xml);
    assert!(result.is_err());
  }

  #[test]
  fn test_attr_to_int_returns_correct_value() {
    let xml = r#"<root attr="42"></root>"#;
    let doc = roxmltree::Document::parse(xml).unwrap();
    let node = doc.root().first_child().unwrap();

    let result = attr_to_int(node, "attr");
    assert_eq!(result, 42);
  }

  #[test]
  fn test_attr_to_int_returns_zero_for_missing_attr() {
    let xml = r#"<root></root>"#;
    let doc = roxmltree::Document::parse(xml).unwrap();
    let node = doc.root().first_child().unwrap();

    let result = attr_to_int(node, "nonexistent");
    assert_eq!(result, 0);
  }

  #[test]
  fn test_attr_to_int_returns_zero_for_invalid_int() {
    let xml = r#"<root attr="not_a_number"></root>"#;
    let doc = roxmltree::Document::parse(xml).unwrap();
    let node = doc.root().first_child().unwrap();

    let result = attr_to_int(node, "attr");
    assert_eq!(result, 0);
  }

  #[test]
  fn test_attr_to_int_handles_negative_numbers() {
    let xml = r#"<root attr="-100"></root>"#;
    let doc = roxmltree::Document::parse(xml).unwrap();
    let node = doc.root().first_child().unwrap();

    let result = attr_to_int(node, "attr");
    assert_eq!(result, -100);
  }

  #[test]
  fn test_attr_to_string_returns_correct_value() {
    let xml = r#"<root attr="hello world"></root>"#;
    let doc = roxmltree::Document::parse(xml).unwrap();
    let node = doc.root().first_child().unwrap();

    let result = attr_to_string(node, "attr");
    assert_eq!(result, "hello world");
  }

  #[test]
  fn test_attr_to_string_returns_empty_for_missing_attr() {
    let xml = r#"<root></root>"#;
    let doc = roxmltree::Document::parse(xml).unwrap();
    let node = doc.root().first_child().unwrap();

    let result = attr_to_string(node, "nonexistent");
    assert_eq!(result, "");
  }

  #[test]
  fn test_parse_decodes_percent_encoded_filename() {
    let xml = String::from(
      r#"<?xml version="1.0" encoding="UTF-8"?>
<posts count="1">
  <post
    id="1"
    file_url="https://example.com/%E6%97%A5%E6%9C%AC%E8%AA%9E.jpg"
    width="100"
    height="100"
    preview_url="https://example.com/preview.jpg"
    preview_width="50"
    preview_height="50"
    sample_url="https://example.com/sample.jpg"
    sample_width="75"
    sample_height="75"
    tags="test"
    rating="s"
  />
</posts>"#,
    );

    let result = parse(xml).unwrap();
    // URL encoded Japanese characters should be decoded
    assert_eq!(result.images[0].name, "日本語.jpg");
  }

  #[test]
  fn test_image_serialization() {
    let image = Image {
      id: 1,
      url: "https://example.com/test.jpg".to_string(),
      width: 1920,
      height: 1080,
      preview: "https://example.com/preview.jpg".to_string(),
      preview_width: 150,
      preview_height: 84,
      sample: "https://example.com/sample.jpg".to_string(),
      sample_width: 1500,
      sample_height: 844,
      tags: "nature landscape".to_string(),
      security: true,
      name: "test.jpg".to_string(),
    };

    let json = serde_json::to_string(&image).unwrap();
    assert!(json.contains("\"id\":1"));
    assert!(json.contains("\"url\":\"https://example.com/test.jpg\""));
    assert!(json.contains("\"security\":true"));
  }

  #[test]
  fn test_post_serialization() {
    let post = Post {
      count: 10,
      images: vec![Image {
        id: 1,
        url: "https://example.com/test.jpg".to_string(),
        width: 100,
        height: 100,
        preview: "https://example.com/preview.jpg".to_string(),
        preview_width: 50,
        preview_height: 50,
        sample: "https://example.com/sample.jpg".to_string(),
        sample_width: 75,
        sample_height: 75,
        tags: "test".to_string(),
        security: true,
        name: "test.jpg".to_string(),
      }],
    };

    let json = serde_json::to_string(&post).unwrap();
    assert!(json.contains("\"count\":10"));
    assert!(json.contains("\"images\""));
  }

  #[test]
  fn test_api_response_serialization() {
    let response = ApiResponse {
      data: None,
      code: 0,
      msg: None,
    };

    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("\"code\":0"));
    assert!(json.contains("\"data\":null"));
  }
}
