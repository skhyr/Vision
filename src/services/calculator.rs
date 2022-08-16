use crate::repos::transition as TransitionRepo;
use crate::repos::vacation as VacationRepo;
use crate::types::transition::Transition;
use crate::types::vacation::Vacation;
use crate::utils::errors::Errors;
use chrono::Datelike;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub fn count_days_in_vacation(vacation: &Vacation) -> i32 {
    vacation.end_date.num_days_from_ce() - vacation.start_date.num_days_from_ce() + 1
}

pub fn match_transition_to_vacation<'a>(
    vacation: &Vacation,
    transitions: &'a Vec<Transition>,
) -> Result<&'a Transition, Errors> {
    transitions
        .iter()
        .find(|transition| {
            transition.date.num_days_from_ce() < vacation.start_date.num_days_from_ce()
        })
        .ok_or(Errors::InvalidVacationFound)
}

pub fn count_used_hours(user_id: Uuid, conn: &PgConnection) -> Result<f64, Errors> {
    let all_vacations = VacationRepo::get_by_user_id(user_id, &conn)?;
    let mut transitions = TransitionRepo::get_by_user_id(user_id, &conn)?;
    transitions.sort_by(|a, b| {
        let an = a.date.num_days_from_ce();
        let bn = b.date.num_days_from_ce();
        bn.cmp(&an)
    });

    all_vacations
        .iter()
        .map(|vacation| {
            match_transition_to_vacation(vacation, &transitions)
                .map(|transition| (vacation, transition))
        })
        .collect::<Result<Vec<(&Vacation, &Transition)>, Errors>>()
        .map(|vec| {
            vec.iter()
                .map(|(vacation, transition)| {
                    let vacation_length = count_days_in_vacation(&vacation);
                    vacation_length as f64 * transition.fraction * 8.0
                })
                .fold(0.0, |acc, len| acc + len)
        })
}
