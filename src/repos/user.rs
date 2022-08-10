use crate::schema::users::{self, table as all_users};
use crate::types::user::User;
use crate::utils::errors::Errors;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Result<User, Errors> {
    all_users
        .find(id)
        .first::<User>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn get_all(conn: &PgConnection) -> Result<Vec<User>, Errors> {
    users::table
        .load::<User>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn insert(user: User, conn: &PgConnection) -> Result<User, Errors> {
    diesel::insert_into(all_users)
        .values(user)
        .get_result::<User>(conn)
        .map_err(|_| Errors::InsertFailed)
}
