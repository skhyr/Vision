use crate::api::DbConn;
use crate::resolvers::vacation;
use crate::types::vacation::{NewVacation, Vacation};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};

#[post(
    "/vacation/add/<user_id>",
    format = "application/json",
    data = "<body>",
    rank = 1
)]
async fn add_vacation(
    user_id: String,
    conn: DbConn,
    body: Json<NewVacation>,
) -> Result<Json<Vacation>, status::BadRequest<()>> {
    conn.run(move |c| {
        let new_vacation = body.into_inner();

        vacation::add_vacation(new_vacation, user_id, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes() -> Vec<Route> {
    routes![add_vacation]
}
