use rocket::response::status;
use rocket::serde::json::Json;

pub mod initial;
pub mod mutations;
pub mod queries;
pub mod return_objects;

pub use initial::DbConn;
pub type Response<T> = Result<Json<T>, status::BadRequest<&'static str>>;
