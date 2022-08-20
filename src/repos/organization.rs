use crate::schema::organizations;
use crate::types::organization::Organization;
use crate::utils::errors::Errors;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Result<Organization, Errors> {
    organizations::table
        .find(id)
        .first::<Organization>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn get_all(conn: &PgConnection) -> Result<Vec<Organization>, Errors> {
    organizations::table
        .load::<Organization>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn insert(organization: Organization, conn: &PgConnection) -> Result<Organization, Errors> {
    diesel::insert_into(organizations::table)
        .values(organization)
        .get_result::<Organization>(conn)
        .map_err(|_| Errors::InsertFailed)
}
