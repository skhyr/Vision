use crate::schema::users;
use crate::types::User;
use crate::utils::errors::Errors;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Result<User, Errors> {
    users::table
        .find(id)
        .first::<User>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn get_all(conn: &PgConnection) -> Result<Vec<User>, Errors> {
    users::table
        .load::<User>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn get_all_by_organization(
    organization_id: Uuid,
    conn: &PgConnection,
) -> Result<Vec<User>, Errors> {
    users::table
        .filter(users::organization_id.eq(organization_id))
        .load::<User>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn insert(user: User, conn: &PgConnection) -> Result<User, Errors> {
    diesel::insert_into(users::table)
        .values(user)
        .get_result::<User>(conn)
        .map_err(|_| Errors::InsertFailed)
}
