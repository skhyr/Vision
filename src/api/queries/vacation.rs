use crate::api::DbConn;
use crate::services::vacation as VacationService;
use crate::types::{vacation::NewVacation, Stats, Vacation};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};
use uuid::Uuid;

#[post(
    "/vacation/calc/<user_id>",
    format = "application/json",
    data = "<body>",
    rank = 1
)]
async fn get_calc(
    user_id: String,
    conn: DbConn,
    body: Json<NewVacation>,
) -> Result<Json<Stats>, status::BadRequest<()>> {
    conn.run(move |c| {
        let parsed_user_id = Uuid::parse_str(user_id.as_str())
            .map_err(|_| status::BadRequest(Some("Invalid Id")))
            .unwrap();
        let new_vacation = body.into_inner();
        let vacation = Vacation {
            id: Uuid::new_v4(),
            user_id: parsed_user_id,
            title: new_vacation.title,
            start_date: new_vacation.start_date,
            end_date: new_vacation.end_date,
        };

        VacationService::get_calc_vacation(vacation, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes() -> Vec<Route> {
    routes![get_calc]
}
