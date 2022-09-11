use crate::logic::calculator as Calculator;
use crate::repos::VacationRepo;
use crate::services::transition as TransitionService;
use crate::types::{
    vacation::{NewVacation, VacationStats},
    ComputedVacation, Transition, Vacation,
};
use crate::utils::errors::Errors;
use diesel::PgConnection;
use uuid::Uuid;

pub fn get_vacation_stats(
    vacation: &Vacation,
    transitions: &Vec<Transition>,
) -> Result<VacationStats, Errors> {
    let len = Calculator::get_vacation_length(vacation);
    let transition = Calculator::match_transition_to_vacation(vacation, transitions)?;
    Ok(VacationStats {
        days: len as f64,
        hours: len as f64 * 8.0 * transition.fraction,
    })
}

pub fn get_computed_vacation(
    vacation_id: Uuid,
    conn: &PgConnection,
) -> Result<ComputedVacation, Errors> {
    let vacation = VacationRepo::get_by_id(vacation_id, conn)?;
    let transitions = TransitionService::get_sorted_transitions(vacation.user_id, conn)?;
    let stats = get_vacation_stats(&vacation, &transitions)?;
    Ok(ComputedVacation {
        vacation_id,
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
