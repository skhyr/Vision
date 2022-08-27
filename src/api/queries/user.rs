use crate::api::DbConn;
use crate::repos::UserRepo;
use crate::services::user as UserService;
use crate::types::{Info, User};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{get, routes, Route};
use uuid::Uuid;

#[get("/user/info/<id>", rank = 1)]
async fn get_info(id: String, conn: DbConn) -> Result<Json<Info>, status::BadRequest<()>> {
    conn.run(move |c| {
        let user_id = Uuid::parse_str(id.as_str())
            .map_err(|_| status::BadRequest(Some("Invalid Id")))
            .unwrap();

        UserService::get_info(user_id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[get("/user/<id>", rank = 2)]
async fn get_user(id: String, conn: DbConn) -> Result<Json<User>, status::BadRequest<()>> {
    conn.run(move |c| {
        let user_id = Uuid::parse_str(id.as_str())
            .map_err(|_| status::BadRequest(Some("Invalid Id")))
            .unwrap();

        UserRepo::get_by_id(user_id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes() -> Vec<Route> {
    routes![get_info, get_user]
}
