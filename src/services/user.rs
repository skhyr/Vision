use crate::repos::UserRepo;
use crate::services::calculator as Calculator;
use crate::services::date as DateService;
use crate::services::{transition, vacation};
use crate::types::{Config, Initials};
use crate::types::{Info, Stats, Transition, Vacation};
use crate::utils::countries::Countries;
use crate::utils::errors::Errors;
use diesel::PgConnection;
use uuid::Uuid;

pub fn get_initials(user_id: Uuid, conn: &PgConnection) -> Result<Initials, Errors> {
    let transitions = transition::get_sorted_transitions(user_id, conn)?;
    let vacations = vacation::get_user_vacations(user_id, conn)?;
    let config = get_config(user_id, conn)?;
    Ok(Initials(vacations, transitions, config))
}

pub fn get_config(user_id: Uuid, conn: &PgConnection) -> Result<Config, Errors> {
    let user = UserRepo::get_by_id(user_id, conn)?;

    Ok(Config {
        accounting_day: user.accounting_day,
        country: Countries::PL,
        date: DateService::get_now(),
    })
}

pub fn get_info(
    vacations: Vec<Vacation>,
    transitions: Vec<Transition>,
    conn: &PgConnection,
) -> Result<Info, Errors> {
    Ok(Info {
        stats: get_stats(&vacations, &transitions)?,
        vacations: vacations,
        transitions: transitions,
    })
}

pub fn get_stats(
    vacations: &Vec<Vacation>,
    transitions: &Vec<Transition>,
) -> Result<Stats, Errors> {
    Ok(Stats {
        generated_hours: Calculator::count_generated_hours(transitions)?,
        used_hours: Calculator::count_used_hours(vacations, transitions)?,
        used_days: Calculator::count_used_days(vacations)?,
        hours_left: Calculator::count_hours_left(vacations, transitions)?,
        days_left: Calculator::count_days_left(vacations, transitions)?,
    })
}
