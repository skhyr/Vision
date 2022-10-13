use crate::api::DbConn;
use crate::resolvers::organization;
use crate::types::{Organization, Token, User, Vacation};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{get, routes, Route};

#[get("/organization", rank = 2)]
async fn get_organization(
    token: Token,
    conn: DbConn,
) -> Result<Json<Organization>, status::BadRequest<()>> {
    conn.run(move |c| {
        organization::get_organization(token, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[get("/organization/users", rank = 1)]
async fn get_users(token: Token, conn: DbConn) -> Result<Json<Vec<User>>, status::BadRequest<()>> {
    conn.run(move |c| {
        organization::get_users(token, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[get("/organization/vacations?<date>", rank = 1)]
async fn get_vacations(
    token: Token,
    date: Option<String>,
    conn: DbConn,
) -> Result<Json<Vec<Vacation>>, status::BadRequest<()>> {
    conn.run(move |c| {
        organization::get_vacations(token, date, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes() -> Vec<Route> {
    routes![get_organization, get_users, get_vacations]
}
