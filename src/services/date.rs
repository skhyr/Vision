use crate::utils::errors::Errors;
use chrono::{Datelike, NaiveDate, Utc};
use diesel::pg::PgConnection;
use uuid::Uuid;

pub fn get_now_as_transition_date(
    _user_id: Uuid,
    _conn: &PgConnection,
) -> Result<NaiveDate, Errors> {
    let now = Utc::now().date_naive();
    // TODO implement logic to handle accounting_day
    Ok(NaiveDate::from_ymd(now.year(), now.month(), 1))
}

pub fn num_months_between(date1: NaiveDate, date2: NaiveDate) -> i32 {
    let num_years = date2.year() - date1.year();
    let num_months = date2.month() as i32 - date1.month() as i32;
    num_years * 12 + num_months as i32
}
