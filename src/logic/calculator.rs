use crate::logic::date as DateService;
use crate::types::{Transition, Vacation};
use crate::utils::errors::Errors;
use chrono::Datelike;

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

pub fn pair_transitions_with_vacations<'a>(
    vacations: &'a Vec<Vacation>,
    transitions: &'a Vec<Transition>,
) -> Result<Vec<(&'a Vacation, &'a Transition)>, Errors> {
    vacations
        .iter()
        .map(|vacation| {
            match_transition_to_vacation(vacation, &transitions)
                .map(|transition| (vacation, transition))
        })
        .collect()
}

pub fn get_vacation_length(vacation: &Vacation) -> i32 {
    DateService::count_days_between(vacation.start_date, vacation.end_date)
}

pub fn count_used_hours(
    vacations: &Vec<Vacation>,
    transitions: &Vec<Transition>,
) -> Result<f64, Errors> {
    pair_transitions_with_vacations(&vacations, &transitions).map(|vec| {
        vec.iter()
            .map(|(vacation, transition)| {
                let vacation_length =
                    DateService::count_days_between(vacation.start_date, vacation.end_date);
                vacation_length as f64 * transition.fraction * 8.0
            })
            .fold(0.0, |acc, len| acc + len)
    })
}

pub fn count_used_days(vacations: &Vec<Vacation>) -> Result<f64, Errors> {
    Ok(vacations
        .iter()
        .map(|vacation| {
            DateService::count_days_between(vacation.start_date, vacation.end_date) as f64
        })
        .fold(0.0, |acc, len| acc + len))
}

pub fn count_generated_hours(transitions: &Vec<Transition>) -> Result<f64, Errors> {
    let now = DateService::get_now_as_transition_date()?;
    let (res, _) = transitions
        .iter()
        .fold((0.0, now), |(acc, prev_date), transition| {
            let duration = DateService::num_months_between(transition.date, prev_date) as f64;
            let gen_hours = 1.75 * 8.0 * duration * transition.fraction;
            (acc + gen_hours, transition.date)
        });
    Ok(res)
}

pub fn count_hours_left(
    vacations: &Vec<Vacation>,
    transitions: &Vec<Transition>,
) -> Result<f64, Errors> {
    let generated = count_generated_hours(transitions)?;
    let used = count_used_hours(vacations, transitions)?;
    Ok(generated - used)
}

pub fn count_days_left(
    vacations: &Vec<Vacation>,
    transitions: &Vec<Transition>,
) -> Result<f64, Errors> {
    let generated = count_generated_hours(transitions)?;
    let used = count_used_hours(vacations, transitions)?;
    let current_fraction = match transitions.get(0) {
        Some(t) => t.fraction,
        None => 1.0,
    };
    Ok((generated - used) / (current_fraction * 8.0))
}
