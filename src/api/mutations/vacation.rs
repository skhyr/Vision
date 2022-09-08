use crate::api::DbConn;
use crate::services::vacation as VacationService;
use crate::types::{vacation::NewVacation, Stats, Vacation};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};
use uuid::Uuid;

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
        let parsed_user_id = Uuid::parse_str(user_id.as_str())
            .map_err(|_| status::BadRequest(Some("Invalid Id")))
            .unwrap();
        let new_vacation = body.into_inner();

        VacationService::add(new_vacation, parsed_user_id, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes() -> Vec<Route> {
    routes![add_vacation]
}
