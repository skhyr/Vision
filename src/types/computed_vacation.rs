use chrono::NaiveDate;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputedVacation {
    pub vacation_id: Uuid,
    pub hours: f64,
    pub days: f64,
    pub title: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub user_id: Uuid,
}
