use crate::repos::VacationRepo;
use crate::services::calculator as Calculator;
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

pub fn gen_vacation_stats(
    vacation: &Vacation,
    conn: &PgConnection,
) -> Result<VacationStats, Errors> {
    let transitions = TransitionService::get_sorted_transitions(vacation.user_id, conn)?;
    get_vacation_stats(vacation, &transitions)
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

pub fn get_computed_vacation(id: Uuid, conn: &PgConnection) -> Result<ComputedVacation, Errors> {
    let vacation = VacationRepo::get_by_id(id, conn)?;
    let stats = gen_vacation_stats(&vacation, conn)?;
    Ok(ComputedVacation {
        vacation_id: id,
        title: vacation.title,
        start_date: vacation.start_date,
        end_date: vacation.end_date,
        hours: stats.hours,
        days: stats.days,
    })
}
