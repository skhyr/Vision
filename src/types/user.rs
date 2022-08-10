use crate::schema::users;
use diesel::{self, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, Queryable, Insertable, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub surname: String,
    pub access_code: i32,
    pub accounting_day: i32,
}
