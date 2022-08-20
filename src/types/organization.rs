use crate::schema::organizations;
use diesel::{self, Queryable};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "organizations"]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
}
