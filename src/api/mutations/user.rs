use crate::api::DbConn;
use crate::resolvers::user;
use crate::types::user::NewUser;
use crate::types::{Token, User};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};

#[post("/user/add", format = "application/json", data = "<body>", rank = 1)]
async fn add_user(
    token: Token,
    conn: DbConn,
    body: Json<NewUser>,
) -> Result<Json<User>, status::BadRequest<()>> {
    conn.run(move |c| {
        let new_user = body.into_inner();

        user::add_user(new_user, token, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes() -> Vec<Route> {
    routes![add_user]
}
