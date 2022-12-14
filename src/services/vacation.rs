use crate::logic::calculator as Calculator;
use crate::repos::VacationRepo;
use crate::services::transition as TransitionService;
use crate::services::user as UserService;
use crate::types::{
    vacation::{NewVacation, VacationStats},
    ComputedVacation, Config, Transition, Vacation,
};
use crate::utils::errors::Errors;
use chrono::NaiveDate;
use diesel::PgConnection;
use uuid::Uuid;

pub fn get_vacation_stats(
    vacation: &Vacation,
    transitions: &Vec<Transition>,
    config: &Config,
) -> Result<VacationStats, Errors> {
    let len = Calculator::get_vacation_length(vacation);
    let transition = Calculator::match_transition_to_vacation(vacation, transitions)?;
    Ok(VacationStats {
        days: len as f64,
        hours: len as f64 * config.full_time_h * transition.fraction,
    })
}

pub fn get_computed_vacation(
    vacation_id: Uuid,
    conn: &PgConnection,
) -> Result<ComputedVacation, Errors> {
    let vacation = VacationRepo::get_by_id(vacation_id, conn)?;
    let transitions = TransitionService::get_sorted_transitions(vacation.user_id, conn)?;
    let config = UserService::get_config(vacation.user_id, None, conn)?;
    let stats = get_vacation_stats(&vacation, &transitions, &config)?;
    Ok(ComputedVacation {
        vacation_id,
        user_id: vacation.user_id,
        title: vacation.title,
        start_date: vacation.start_date,
        end_date: vacation.end_date,
        hours: stats.hours,
        days: stats.days,
    })
}

pub fn get_user_vacations(user_id: Uuid, conn: &PgConnection) -> Result<Vec<Vacation>, Errors> {
    VacationRepo::get_by_user_id(user_id, conn)
}

pub fn get_filtered_vacations(
    user_id: Uuid,
    date: Option<NaiveDate>,
    conn: &PgConnection,
) -> Result<Vec<Vacation>, Errors> {
    let vacations = get_user_vacations(user_id, conn)?;
    Ok(match date {
        None => vacations,
        Some(d) => vacations.into_iter().filter(|v| v.start_date < d).collect(),
    })
}

pub fn add(
    new_vacation: NewVacation,
    user_id: Uuid,
    conn: &PgConnection,
) -> Result<Vacation, Errors> {
    let vacation = Vacation {
        id: Uuid::new_v4(),
        user_id,
        title: new_vacation.title,
        start_date: new_vacation.start_date,
        end_date: new_vacation.end_date,
    };
    VacationRepo::insert(vacation, conn)
}
