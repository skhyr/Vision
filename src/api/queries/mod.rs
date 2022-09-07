use rocket::Route;

pub mod organization;
pub mod user;
pub mod vacation;

pub fn get_routes() -> Vec<Route> {
    vec![
        user::get_routes(),
        organization::get_routes(),
        vacation::get_routes(),
    ]
    .into_iter()
    .flatten()
    .collect()
}
