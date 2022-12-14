use crate::schema::users;
use diesel::{self, Queryable};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, Deserialize, Clone)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub surname: String,
    pub access_code: i32,
    pub accounting_day: i32,
    pub organization_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUser {
    pub name: String,
    pub surname: String,
    pub access_code: i32,
    pub accounting_day: i32,
}
