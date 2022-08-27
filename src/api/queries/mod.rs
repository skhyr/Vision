use rocket::Route;

pub mod organization;
pub mod user;

pub fn get_routes() -> Vec<Route> {
    vec![user::get_routes(), organization::get_routes()]
        .into_iter()
        .flatten()
        .collect()
}
