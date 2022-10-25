use rocket::Route;

pub mod organization;
pub mod user;
pub mod vacation;

#[get("/", rank = 8)]
async fn index(
) -> &'static str {
    "Vision"
}

pub fn get_routes() -> Vec<Route> {
    vec![
        user::get_routes(),
        organization::get_routes(),
        vacation::get_routes(),
        routes![index]
    ]
    .into_iter()
    .flatten()
    .collect()
}
