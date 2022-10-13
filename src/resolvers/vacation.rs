use crate::services::authorization::{authorize, AccessLevels::*, AuthObj};
use crate::services::{user, vacation};
use crate::types::vacation::{NewVacation, Vacation, VacationStats};
use crate::types::{ComputedVacation, Initials, Token};
use crate::utils;
use crate::utils::errors::Errors;
use diesel::PgConnection;
use uuid::Uuid;

/*
  auth: Admin of the user
  desc: Estimate vacation before adding
*/
pub fn calc_vacation(
    new_vacation: NewVacation,
    user_id: String,
    date: Option<String>,
    token: Token,
    conn: &PgConnection,
) -> Result<VacationStats, Errors> {
    let user_uuid = utils::parse_uuid(user_id)?;
    let date = utils::parse_date(date)?;
    authorize(token, Admin(AuthObj::User(user_uuid)), conn)?;
    let vacation = Vacation {
        id: Uuid::new_v4(),
        user_id: user_uuid,
        title: new_vacation.title,
        start_date: new_vacation.start_date,
        end_date: new_vacation.end_date,
    };
    let Initials(_, transitions, config) = user::get_initials(user_uuid, date, conn)?;
    vacation::get_vacation_stats(&vacation, &transitions, &config)
}

/*
  auth: Co-worker of the vacation's owner
*/
pub fn get_computed_vacation(
    vacation_id: String,
    token: Token,
    conn: &PgConnection,
) -> Result<ComputedVacation, Errors> {
    let vacation_uuid = utils::parse_uuid(vacation_id)?;
    authorize(token, Organization(AuthObj::Vacation(vacation_uuid)), conn)?;
    vacation::get_computed_vacation(vacation_uuid, conn)
}

/*
  auth: Admin of the user
*/
pub fn add_vacation(
    new_vacation: NewVacation,
    user_id: String,
    token: Token,
    conn: &PgConnection,
) -> Result<Vacation, Errors> {
    let user_uuid = utils::parse_uuid(user_id)?;
    authorize(token, Admin(AuthObj::User(user_uuid)), conn)?;
    vacation::add(new_vacation, user_uuid, conn)
}
