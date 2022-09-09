use crate::api::DbConn;
use crate::resolvers::vacation;
use crate::types::vacation::{NewVacation, VacationStats};
use crate::types::ComputedVacation;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};

#[post(
    "/vacation/calc/<user_id>",
    format = "application/json",
    data = "<body>"
)]
async fn get_calc(
    user_id: String,
    conn: DbConn,
    body: Json<NewVacation>,
) -> Result<Json<VacationStats>, status::BadRequest<()>> {
    conn.run(move |c| {
        vacation::calc_vacation(body.into_inner(), user_id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[get("/vacation/<id>")]
async fn get_vacation_info(
    id: String,
    conn: DbConn,
) -> Result<Json<ComputedVacation>, status::BadRequest<()>> {
    conn.run(move |c| {
        vacation::get_computed_vacation(id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes() -> Vec<Route> {
    routes![get_calc, get_vacation_info]
}
