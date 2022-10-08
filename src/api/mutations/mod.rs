use rocket::Route;

pub mod user;
pub mod vacation;

pub fn get_routes() -> Vec<Route> {
    vec![vacation::get_routes(), user::get_routes()]
        .into_iter()
        .flatten()
        .collect()
}
