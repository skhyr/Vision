use crate::schema::transitions;
use chrono::NaiveDate;
use diesel::{self, Queryable};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, Deserialize, Serialize)]
#[table_name = "transitions"]
pub struct Transition {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub fraction: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTransition {
    pub date: NaiveDate,
    pub fraction: f64,
}
