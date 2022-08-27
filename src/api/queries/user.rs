use crate::api::DbConn;
use crate::services::user as UserService;
use crate::types::Info;
use crate::utils::errors::Errors;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{get, routes, Route};
use uuid::Uuid;

#[get("/info/<id>")]
async fn get_info(id: String, conn: DbConn) -> Result<Json<Info>, status::BadRequest<()>> {
    conn.run(move |c| {
        println!("{:?}", id);
        let user_id = Uuid::parse_str(id.as_str())
            .map_err(|_| status::BadRequest(Some("Invalid Id")))
            .unwrap();

        UserService::get_info(user_id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes() -> Vec<Route> {
    routes![get_info]
}
