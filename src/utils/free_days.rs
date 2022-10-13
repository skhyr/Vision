use chrono::NaiveDate;

use super::countries::Countries;

fn get_free_days_pl() -> Vec<NaiveDate> {
    vec![
        NaiveDate::from_ymd(2022, 1, 1),
        NaiveDate::from_ymd(2022, 1, 6),
        NaiveDate::from_ymd(2022, 4, 17),
        NaiveDate::from_ymd(2022, 4, 18),
        NaiveDate::from_ymd(2022, 5, 1),
        NaiveDate::from_ymd(2022, 5, 3),
        NaiveDate::from_ymd(2022, 6, 5),
        NaiveDate::from_ymd(2022, 6, 16),
        NaiveDate::from_ymd(2022, 8, 15),
        NaiveDate::from_ymd(2022, 11, 1),
        NaiveDate::from_ymd(2022, 12, 25),
        NaiveDate::from_ymd(2022, 12, 26),
    ]
}

fn get_free_days(country: Countries) -> Vec<NaiveDate> {
    match country {
        Countries::PL => get_free_days_pl(),
    }
}

pub fn is_day_free(date: &NaiveDate, country: Countries) -> bool {
    get_free_days(country).iter().any(|d| d == date)
}
