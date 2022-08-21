use crate::schema::vacations;
use chrono::NaiveDate;
use diesel::{self, Queryable};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "vacations"]
pub struct Vacation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub title: String,
}

pub struct NewVacation {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub title: String,
}
