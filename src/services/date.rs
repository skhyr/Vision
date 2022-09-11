use crate::utils::errors::Errors;
use chrono::{Datelike, NaiveDate, Utc};

pub fn get_now_as_transition_date() -> Result<NaiveDate, Errors> {
    let now = Utc::now().date_naive();
    // TODO implement logic to handle accounting_day
    Ok(NaiveDate::from_ymd(now.year(), now.month(), 1))
}

pub fn get_now() -> NaiveDate {
    Utc::now().date_naive()
}

pub fn num_months_between(date1: NaiveDate, date2: NaiveDate) -> i32 {
    let num_years = date2.year() - date1.year();
    let num_months = date2.month() as i32 - date1.month() as i32;
    num_years * 12 + num_months as i32
}

pub fn count_days_between(start_date: NaiveDate, end_date: NaiveDate) -> i32 {
    end_date.num_days_from_ce() - start_date.num_days_from_ce() + 1
}
