use crate::schema::transitions;
use chrono::NaiveDate;
use diesel::{self, Queryable};
use uuid::Uuid;

#[derive(Clone, Queryable, Insertable, Debug)]
#[table_name = "transitions"]
pub struct Transition {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
}
