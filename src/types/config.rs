use crate::types::{Transition, Vacation};
use crate::utils::countries::Countries;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct Config {
    pub date: NaiveDate,
    pub accounting_day: i32,
    pub country: Countries,
}

pub struct Initials(pub Vec<Vacation>, pub Vec<Transition>, pub Config);
