use crate::logic::date;
use chrono::NaiveDate;

#[test]
fn num_months_between_1() {
    let d1 = NaiveDate::from_ymd(2022, 07, 01);
    let d2 = NaiveDate::from_ymd(2022, 09, 01);
    let result = date::num_months_between(d1, d2);
    assert_eq!(result, 2);
}

#[test]
fn num_months_between_2() {
    let d1 = NaiveDate::from_ymd(2021, 07, 01);
    let d2 = NaiveDate::from_ymd(2022, 09, 01);
    let result = date::num_months_between(d1, d2);
    assert_eq!(result, 14);
}

#[test]
fn num_months_between_3() {
    let d1 = NaiveDate::from_ymd(2022, 07, 10);
    let d2 = NaiveDate::from_ymd(2022, 07, 20);
    let result = date::num_months_between(d1, d2);
    assert_eq!(result, 0);
}

#[test]
fn num_months_between_4() {
    let d1 = NaiveDate::from_ymd(2022, 07, 10);
    let d2 = NaiveDate::from_ymd(2022, 09, 05);
    let result = date::num_months_between(d1, d2);
    assert_eq!(result, 1);
}

#[test]
fn num_months_between_5() {
    let d1 = NaiveDate::from_ymd(2022, 07, 10);
    let d2 = NaiveDate::from_ymd(2022, 09, 20);
    let result = date::num_months_between(d1, d2);
    assert_eq!(result, 2);
}
