use crate::repos::transition as TransitionRepo;
use crate::types::transition::Transition;
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
