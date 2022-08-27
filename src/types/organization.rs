use crate::schema::organizations;
use diesel::{self, Queryable};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, Deserialize, Serialize)]
#[table_name = "organizations"]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
}
