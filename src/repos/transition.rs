use crate::schema::transitions;
use crate::types::transition::Transition;
use crate::utils::errors::Errors;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Result<Transition, Errors> {
    transitions::table
        .find(id)
        .first::<Transition>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn get_by_user_id(user_id: Uuid, conn: &PgConnection) -> Result<Vec<Transition>, Errors> {
    transitions::table
        .filter(transitions::user_id.eq(user_id))
        .load::<Transition>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn get_all(conn: &PgConnection) -> Result<Vec<Transition>, Errors> {
    transitions::table
        .load::<Transition>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn insert(transition: Transition, conn: &PgConnection) -> Result<Transition, Errors> {
    diesel::insert_into(transitions::table)
        .values(transition)
        .get_result::<Transition>(conn)
        .map_err(|_| Errors::InsertFailed)
}
