use crate::api::DbConn;
use crate::resolvers::transition;
use crate::types::transition::NewTransition;
use crate::types::{Token, Transition};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};

#[post(
    "/transition/add/<user_id>",
    format = "application/json",
    data = "<body>",
    rank = 1
)]
async fn add_transition(
    token: Token,
    conn: DbConn,
    user_id: String,
    body: Json<NewTransition>,
) -> Result<Json<Transition>, status::BadRequest<()>> {
    conn.run(move |c| {
        let new_transition = body.into_inner();

        transition::add_transition(new_transition, user_id, token, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes() -> Vec<Route> {
    routes![add_transition]
}
