use rocket::Route;

pub mod transition;
pub mod user;
pub mod vacation;

pub fn get_routes() -> Vec<Route> {
    vec![
        vacation::get_routes(),
        user::get_routes(),
        transition::get_routes(),
    ]
    .into_iter()
    .flatten()
    .collect()
}
