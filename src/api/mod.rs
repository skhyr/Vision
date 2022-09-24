use rocket::response::status;
use rocket::serde::json::Json;

pub mod initial;
pub mod mutations;
pub mod queries;
mod return_objects;
mod utils;

pub use initial::DbConn;
pub type Response<T> = Result<Json<T>, status::BadRequest<&'static str>>;
