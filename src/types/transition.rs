use crate::schema::transitions;
use chrono::NaiveDate;
use diesel::{self, Queryable};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "transitions"]
pub struct Transition {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub fraction: f64,
}

#[derive(Debug)]
pub struct NewTransition {
    pub date: NaiveDate,
    pub fraction: f64,
}
