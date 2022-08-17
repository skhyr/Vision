use crate::repos::vacation as VacationRepo;
use crate::services::date as DateService;
use crate::services::transition as TransitionService;
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
    let all_vacations = VacationRepo::get_by_user_id(user_id, conn)?;
    let transitions = TransitionService::get_sorted_transitions(user_id, conn)?;

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

pub fn count_generated_hours(user_id: Uuid, conn: &PgConnection) -> Result<f64, Errors> {
    let transitions = TransitionService::get_sorted_transitions(user_id, &conn)?;
    let now = DateService::get_now_as_transition_date(user_id, conn)?;

    let (res, _) = transitions
        .iter()
        .fold((0.0, now), |(acc, prev_date), transition| {
            let duration = DateService::num_months_between(transition.date, prev_date) as f64;
            let gen_hours = 1.75 * duration * transition.fraction;
            (acc + gen_hours, transition.date)
        });
    Ok(res)
}
