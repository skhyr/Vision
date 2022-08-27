use rocket::Route;

pub mod user;

pub fn get_routes() -> Vec<Route> {
    vec![user::get_routes()].into_iter().flatten().collect()
}
