use crate::logic::date as DateService;
use crate::types::{Config, Transition, Vacation};
use crate::utils::errors::Errors;
use chrono::{Datelike, NaiveDate};

use super::date::get_now;

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

fn pair_transitions_with_vacations<'a>(
    vacations: Vec<&'a Vacation>,
    transitions: &'a Vec<Transition>,
) -> Result<Vec<(&'a Vacation, &'a Transition)>, Errors> {
    vacations
        .into_iter()
        .map(|vacation| {
            match_transition_to_vacation(vacation, &transitions)
                .map(|transition| (vacation, transition))
        })
        .collect()
}

pub fn find_transition_for_date(
    transitions: &Vec<Transition>,
    date: NaiveDate,
) -> Result<&Transition, Errors> {
    transitions
        .into_iter()
        .filter(|t| t.date.num_days_from_ce() < date.num_days_from_ce())
        .collect::<Vec<&Transition>>()
        .into_iter()
        .max_by(|t1, t2| t1.date.cmp(&t2.date))
        .ok_or(Errors::Unknown)
}

pub fn get_vacation_length(vacation: &Vacation) -> i32 {
    DateService::count_days_between(vacation.start_date, vacation.end_date)
}

pub fn filter_vacations(vacations: &Vec<Vacation>, date: Option<NaiveDate>) -> Vec<&Vacation> {
    match date {
        None => vacations.iter().collect(),
        Some(d) => vacations.iter().filter(|v| v.end_date < d).collect(),
    }
}

pub fn filter_transitions(
    transitions: &Vec<Transition>,
    date: Option<NaiveDate>,
) -> Vec<&Transition> {
    match date {
        None => transitions.iter().collect(),
        Some(d) => transitions.iter().filter(|t| t.date < d).collect(),
    }
}

pub fn count_used(
    vacations: &Vec<Vacation>,
    transitions: &Vec<Transition>,
    config: &Config,
) -> Result<(f64, f64), Errors> {
    let filtered_vacations = filter_vacations(vacations, config.date);
    pair_transitions_with_vacations(filtered_vacations, transitions).map(|pairs| {
        pairs
            .into_iter()
            .map(|(vacation, transition)| {
                let used_d =
                    DateService::count_days_between(vacation.start_date, vacation.end_date) as f64;
                let used_h = used_d * transition.fraction * config.full_time_h;
                (used_h, used_d)
            })
            .fold((0.0, 0.0), |(sum_h, sum_d), (h, d)| (sum_h + h, sum_d + d))
    })
}

pub fn count_generated_h(transitions: &Vec<Transition>, config: &Config) -> Result<f64, Errors> {
    let starting_date = config.date.ok_or(Errors::InvalidDate)?;
    let filtered = filter_transitions(transitions, Some(starting_date));
    let (sum, _) = filtered
        .iter()
        .fold((0.0, starting_date), |(acc, prev_date), transition| {
            let duration = DateService::num_months_between(transition.date, prev_date) as f64;
            println!("space {:?}", duration);
            let gen_hours =
                config.monthly_gen_days * config.full_time_h * duration * transition.fraction;
            (acc + gen_hours, transition.date)
        });
    Ok(sum)
}

pub fn hours_to_days(hours: f64, transitions: &Vec<Transition>) -> Result<f64, Errors> {
    let current_fraction = find_transition_for_date(transitions, get_now())?.fraction;
    Ok((hours / (current_fraction * 8.0)).floor())
}
