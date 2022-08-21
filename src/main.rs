#[macro_use]
extern crate rocket;
use lib::api::initial::init_routes;

#[launch]
fn rocket() -> _ {
    init_routes()
}
