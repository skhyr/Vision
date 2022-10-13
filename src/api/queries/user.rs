use crate::api::DbConn;
use crate::resolvers::user;
use crate::types::{Info, Token, User};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{get, routes, Route};

#[get("/user/info/<id>?<date>", rank = 4)]
async fn get_info(
    id: String,
    token: Token,
    conn: DbConn,
    date: Option<String>,
) -> Result<Json<Info>, status::BadRequest<()>> {
    conn.run(move |c| {
        user::get_info(token, id, date, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[get("/user/info/me", rank = 3)]
async fn get_my_info(token: Token, conn: DbConn) -> Result<Json<Info>, status::BadRequest<()>> {
    conn.run(move |c| {
        user::get_my_info(token, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[get("/user/<id>", rank = 2)]
async fn get_user(
    id: String,
    token: Token,
    conn: DbConn,
) -> Result<Json<User>, status::BadRequest<()>> {
    conn.run(move |c| {
        user::get_user(token, id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[get("/user/me", rank = 1)]
async fn get_me(token: Token, conn: DbConn) -> Result<Json<User>, status::BadRequest<()>> {
    conn.run(move |c| {
        user::get_me(token, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes() -> Vec<Route> {
    routes![get_info, get_my_info, get_user, get_me]
}
