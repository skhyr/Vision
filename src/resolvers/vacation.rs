use crate::services::vacation as VacationService;
use crate::types::vacation::{NewVacation, Vacation, VacationStats};
use crate::types::ComputedVacation;
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

    VacationService::gen_vacation_stats(&vacation, conn)
}

pub fn get_computed_vacation(
    vacation_id: String,
    conn: &PgConnection,
) -> Result<ComputedVacation, Errors> {
    let vacation_uuid = utils::parse_uuid(vacation_id)?;
    VacationService::get_computed_vacation(vacation_uuid, conn)
}

pub fn add_vacation(
    new_vacation: NewVacation,
    user_id: String,
    conn: &PgConnection,
) -> Result<Vacation, Errors> {
    let user_uuid = utils::parse_uuid(user_id)?;

    VacationService::add(new_vacation, user_uuid, conn)
}
