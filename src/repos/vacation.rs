use crate::schema::vacations::{self, table as all_vacations};
use crate::types::vacation::Vacation;
use crate::utils::errors::Errors;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Result<Vacation, Errors> {
    all_vacations
        .find(id)
        .first::<Vacation>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn get_by_user_id(user_id: Uuid, conn: &PgConnection) -> Result<Vec<Vacation>, Errors> {
    all_vacations
        .filter(vacations::user_id.eq(user_id))
        .load::<Vacation>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn get_all(conn: &PgConnection) -> Result<Vec<Vacation>, Errors> {
    all_vacations
        .load::<Vacation>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn insert(vacation: Vacation, conn: &PgConnection) -> Result<Vacation, Errors> {
    diesel::insert_into(all_vacations)
        .values(vacation)
        .get_result::<Vacation>(conn)
        .map_err(|_| Errors::InsertFailed)
}
