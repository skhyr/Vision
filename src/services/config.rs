use crate::repos::UserRepo;
use crate::services::date as DateService;
use crate::services::{transition, vacation};
use crate::types::{Config, Initials};
use crate::utils::countries::Countries;
use crate::utils::Errors;
use diesel::PgConnection;
use uuid::Uuid;

pub fn generate_config(user_id: Uuid, conn: &PgConnection) -> Result<Config, Errors> {
    let user = UserRepo::get_by_id(user_id, conn)?;

    Ok(Config {
        accounting_day: user.accounting_day,
        country: Countries::PL,
        date: DateService::get_now(),
    })
}

pub fn get_user_initial(user_id: Uuid, conn: &PgConnection) -> Result<Initials, Errors> {
    let transitions = transition::get_sorted_transitions(user_id, conn)?;
    let vacations = vacation::get_user_vacations(user_id, conn)?;
    let config = generate_config(user_id, conn)?;
    Ok(Initials(vacations, transitions, config))
}
