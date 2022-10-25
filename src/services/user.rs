use crate::logic::calculator as Calculator;
use crate::logic::date::get_now;
use crate::repos::UserRepo;
use crate::services::{transition, vacation};
use crate::types::{Config, Initials};
use crate::types::{Info, Stats, Transition, Vacation};
use crate::utils::countries::Countries;
use crate::utils::errors::Errors;
use chrono::NaiveDate;
use diesel::PgConnection;
use uuid::Uuid;

pub fn get_initials(
    user_id: Uuid,
    date: Option<NaiveDate>,
    conn: &PgConnection,
) -> Result<Initials, Errors> {
    let transitions = transition::get_filtered_transitions(user_id, date, conn)?;
    let vacations = vacation::get_filtered_vacations(user_id, date, conn)?;
    let config = get_config(user_id, date, conn)?;
    Ok(Initials(vacations, transitions, config))
}

pub fn get_config(
    user_id: Uuid,
    date: Option<NaiveDate>,
    conn: &PgConnection,
) -> Result<Config, Errors> {
    let user = UserRepo::get_by_id(user_id, conn)?;
    Ok(Config {
        date,
        accounting_day: user.accounting_day,
        country: Countries::PL,
        full_time_h: 8.0,
        monthly_gen_days: 1.75,
    })
}

pub fn get_info(
    vacations: Vec<Vacation>,
    transitions: Vec<Transition>,
    config: &Config,
) -> Result<Info, Errors> {
    Ok(Info {
        stats: get_stats(&vacations, &transitions, config)?,
        vacations,
        transitions,
    })
}

fn get_stats(
    vacations: &Vec<Vacation>,
    transitions: &Vec<Transition>,
    config: &Config,
) -> Result<Stats, Errors> {
    let (so_far_h, so_far_d) = Calculator::count_used(
        vacations,
        transitions,
        &Config {
            date: Some(get_now()),
            ..*config
        },
    )?;
    let (all_time_h, all_time_d) = Calculator::count_used(
        vacations,
        transitions,
        &Config {
            date: None,
            ..*config
        },
    )?;
    let generated_h = Calculator::count_generated_h(
        transitions,
        &Config {
            date: Some(get_now()),
            ..*config
        },
    )?;

    Ok(Stats {
        so_far_h,
        so_far_d,
        reserved_h: all_time_h - so_far_h,
        reserved_d: all_time_d - so_far_d,
        free_h: generated_h - all_time_h,
        free_d: Calculator::hours_to_days(generated_h - all_time_h, transitions)?,
    })
}
