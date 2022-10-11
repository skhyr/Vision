use crate::utils::errors::Errors;
use chrono::NaiveDate;
use uuid::Uuid;

pub fn parse_uuid(s: String) -> Result<Uuid, Errors> {
    Uuid::parse_str(s.as_str()).map_err(|_| Errors::InvalidId)
}

pub fn parse_date(date: Option<String>) -> Result<Option<NaiveDate>, Errors> {
    match date {
        None => Ok(None),
        Some(d) => NaiveDate::parse_from_str(&d, "%Y-%m-%d")
            .map(|r| Some(r))
            .map_err(|_| Errors::InvalidDate),
    }
}
