// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] extern crate rocket;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

// fn main() {
//     rocket::ignite().mount("/", routes![index]).launch();
// }
mod lib;
mod conf;

use lib::get_post;
use conf::API;

fn main() {
    let resp = get_post(API, 22);
    println!("{:#?}", resp);
}
