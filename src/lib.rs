use reqwest;
use roxmltree;
use serde::{ Serialize, Deserialize };
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket_contrib::json::JsonValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
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
    safe: bool,
}

#[derive(Debug)]
pub struct ApiResponse {
    pub json: JsonValue,
    pub status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    count: i32,
    images: Vec<Image>,
}

pub fn attr_to_int(e: roxmltree::Node, attr: &str) -> i32 {
    e.attribute(attr).unwrap().to_string().parse::<i32>().unwrap()
}


#[tokio::main]
pub async fn get_post(url: &str, page: i8) -> Result<Post, Box<dyn std::error::Error>> {
   let client = reqwest::Client::new();
   let resp = client.get(url)
        .query(&[("page", page)])
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
            },
            "post" => {
                images.push( Image {
                    url: e.attribute("file_url").unwrap().to_string(),
                    width: attr_to_int(e, "width"),
                    height: attr_to_int(e, "height"),
                    preview: e.attribute("preview_url").unwrap().to_string(),
                    preview_width: attr_to_int(e,"preview_width"),
                    preview_height: attr_to_int(e, "preview_height"),
                    sample: e.attribute("sample_url").unwrap().to_string(),
                    sample_width: attr_to_int(e, "sample_width"),
                    sample_height: attr_to_int(e, "sample_height"),
                    tags: e.attribute("tags").unwrap().to_string(),
                    safe: e.attribute("rating").unwrap() == "s"
                });
            },
            _ => {}
        }
    }
    Post {
        count,
        images
    }
}
