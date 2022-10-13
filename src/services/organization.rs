use crate::repos::{UserRepo, VacationRepo};
use crate::types::{user::NewUser, User, Vacation};
use crate::utils::errors::Errors;
use chrono::NaiveDate;
use diesel::PgConnection;
use uuid::Uuid;

pub fn get_all_users(organization_id: Uuid, conn: &PgConnection) -> Result<Vec<User>, Errors> {
    UserRepo::get_all_by_organization(organization_id, conn)
}

pub fn get_all_vacations(
    organization_id: Uuid,
    date: Option<NaiveDate>,
    conn: &PgConnection,
) -> Result<Vec<Vacation>, Errors> {
    let all: Vec<Vacation> = UserRepo::get_all_by_organization(organization_id, conn)?
        .iter()
        .map(|user| VacationRepo::get_by_user_id(user.id, conn))
        .collect::<Result<Vec<Vec<Vacation>>, Errors>>()
        .map(|res| res.into_iter().flatten().collect())?;
    match date {
        None => Ok(all),
        Some(d) => Ok(all.into_iter().filter(|v| v.end_date > d).collect()),
    }
}

pub fn add_user(
    new_user: NewUser,
    organization_id: Uuid,
    conn: &PgConnection,
) -> Result<User, Errors> {
    let user = User {
        id: Uuid::new_v4(),
        organization_id,
        name: new_user.name,
        surname: new_user.surname,
        accounting_day: new_user.accounting_day,
        access_code: new_user.access_code,
    };
    UserRepo::insert(user, conn)
}
