use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Stats {
    pub so_far_h: f64,
    pub so_far_d: f64,
    pub reserved_h: f64,
    pub reserved_d: f64,
    pub free_h: f64,
    pub free_d: f64,
}
