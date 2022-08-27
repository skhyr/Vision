use crate::schema::vacations;
use chrono::NaiveDate;
use diesel::{self, Queryable};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, Deserialize, Serialize)]
#[table_name = "vacations"]
pub struct Vacation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewVacation {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub title: String,
}
