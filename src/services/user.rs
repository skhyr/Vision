use crate::repos::vacation as VacationRepo;
use crate::services::calculator as Calculator;
use crate::services::transition as TransitionService;
use crate::types::{Info, Stats, Transition, Vacation};
use crate::utils::errors::Errors;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub fn gen_stats(
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

pub fn get_info(user_id: Uuid, conn: &PgConnection) -> Result<Info, Errors> {
    let vacations = VacationRepo::get_by_user_id(user_id, conn)?;
    let transitions = TransitionService::get_sorted_transitions(user_id, conn)?;
    Ok(Info {
        stats: gen_stats(&vacations, &transitions)?,
        vacations,
        transitions,
    })
}
