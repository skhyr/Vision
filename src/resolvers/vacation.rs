use crate::services::{user, vacation};
use crate::types::vacation::{NewVacation, Vacation, VacationStats};
use crate::types::{ComputedVacation, Initials};
use crate::utils;
use crate::utils::errors::Errors;
use diesel::PgConnection;
use uuid::Uuid;

pub fn calc_vacation(
    new_vacation: NewVacation,
    user_id: String,
    conn: &PgConnection,
) -> Result<VacationStats, Errors> {
    let user_uuid = utils::parse_uuid(user_id)?;
    let vacation = Vacation {
        id: Uuid::new_v4(),
        user_id: user_uuid,
        title: new_vacation.title,
        start_date: new_vacation.start_date,
        end_date: new_vacation.end_date,
    };
    let Initials(_, transitions, config) = user::get_initials(user_uuid, conn)?;
    vacation::get_vacation_stats(&vacation, &transitions, &config)
}

pub fn get_computed_vacation(
    vacation_id: String,
    conn: &PgConnection,
) -> Result<ComputedVacation, Errors> {
    let vacation_uuid = utils::parse_uuid(vacation_id)?;
    vacation::get_computed_vacation(vacation_uuid, conn)
}

pub fn add_vacation(
    new_vacation: NewVacation,
    user_id: String,
    conn: &PgConnection,
) -> Result<Vacation, Errors> {
    let user_uuid = utils::parse_uuid(user_id)?;

    vacation::add(new_vacation, user_uuid, conn)
}
