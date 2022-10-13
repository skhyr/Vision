use crate::types::Config;
use crate::utils::errors::Errors;
use chrono::{Datelike, NaiveDate, Utc, Weekday};

pub fn get_now_as_transition_date(config: &Config) -> Result<NaiveDate, Errors> {
    let now = Utc::now().date_naive();
    let day: u32 = config
        .accounting_day
        .try_into()
        .map_err(|_| Errors::InvalidDate)?;

    let date = NaiveDate::from_ymd_opt(now.year(), now.month(), day).ok_or(Errors::InvalidDate);
    match config.date {
        Some(d) => Ok(d),
        None => date,
    }
}

pub fn get_now() -> NaiveDate {
    Utc::now().date_naive()
}

pub fn num_months_between(date1: NaiveDate, date2: NaiveDate) -> i32 {
    let num_years = date2.year() - date1.year();
    let num_months = date2.month() as i32 - date1.month() as i32;
    num_years * 12 + num_months as i32
}

pub fn is_workday(date: &NaiveDate) -> bool {
    match date.weekday() {
        Weekday::Sat => false,
        Weekday::Sun => false,
        _ => true,
    }
}

pub fn count_days_between(start_date: NaiveDate, end_date: NaiveDate) -> i32 {
    start_date
        .iter_days()
        .take_while(|d| d <= &end_date)
        .filter(is_workday)
        .count() as i32
}
