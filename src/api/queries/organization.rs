use crate::api::DbConn;
use crate::repos::OrganizationRepo;
use crate::services::organization as OrganizationSerivce;
use crate::types::{Organization, User, Vacation};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{get, routes, Route};
use uuid::Uuid;

#[get("/organization/<id>", rank = 2)]
async fn get_organization(
    id: String,
    conn: DbConn,
) -> Result<Json<Organization>, status::BadRequest<()>> {
    conn.run(move |c| {
        let organization_id = Uuid::parse_str(id.as_str())
            .map_err(|_| status::BadRequest(Some("Invalid Id")))
            .unwrap();

        OrganizationRepo::get_by_id(organization_id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[get("/organization/<id>/users", rank = 1)]
async fn get_users(id: String, conn: DbConn) -> Result<Json<Vec<User>>, status::BadRequest<()>> {
    conn.run(move |c| {
        let organization_id = Uuid::parse_str(id.as_str())
            .map_err(|_| status::BadRequest(Some("Invalid Id")))
            .unwrap();

        OrganizationSerivce::get_all_users(organization_id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[get("/organization/<id>/vacations", rank = 1)]
async fn get_vacations(
    id: String,
    conn: DbConn,
) -> Result<Json<Vec<Vacation>>, status::BadRequest<()>> {
    conn.run(move |c| {
        let organization_id = Uuid::parse_str(id.as_str())
            .map_err(|_| status::BadRequest(Some("Invalid Id")))
            .unwrap();

        OrganizationSerivce::get_all_vacations(organization_id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes() -> Vec<Route> {
    routes![get_organization, get_users, get_vacations]
}
