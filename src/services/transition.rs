use crate::repos::TransitionRepo;
use crate::types::transition::{NewTransition, Transition};
use crate::utils::errors::Errors;
use chrono::{Datelike, NaiveDate};
use diesel::pg::PgConnection;
use uuid::Uuid;

pub fn get_sorted_transitions(
    user_id: Uuid,
    conn: &PgConnection,
) -> Result<Vec<Transition>, Errors> {
    let mut transitions = TransitionRepo::get_by_user_id(user_id, &conn)?;
    transitions.sort_by(|a, b| {
        let an = a.date.num_days_from_ce();
        let bn = b.date.num_days_from_ce();
        bn.cmp(&an)
    });
    Ok(transitions)
}

pub fn get_filtered_transitions(
    user_id: Uuid,
    date: Option<NaiveDate>,
    conn: &PgConnection,
) -> Result<Vec<Transition>, Errors> {
    let transitions = get_sorted_transitions(user_id, conn)?;
    Ok(match date {
        None => transitions,
        Some(d) => transitions.into_iter().filter(|t| t.date < d).collect(),
    })
}

pub fn add(
    new_transition: NewTransition,
    user_id: Uuid,
    conn: &PgConnection,
) -> Result<Transition, Errors> {
    let transition = Transition {
        id: Uuid::new_v4(),
        user_id,
        fraction: new_transition.fraction,
        date: new_transition.date,
    };
    TransitionRepo::insert(transition, conn)
}
