#[macro_use]
extern crate diesel;

pub mod api;
pub mod repos;
pub mod schema;
pub mod services;
pub mod types;
pub mod utils;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
pub fn launch_server() -> _ {
    rocket::build().mount("/", routes![index])
}
