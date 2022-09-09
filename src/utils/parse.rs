use crate::utils::errors::Errors;
use uuid::Uuid;

pub fn parse_uuid(s: String) -> Result<Uuid, Errors> {
    Uuid::parse_str(s.as_str()).map_err(|_| Errors::InvalidId)
}
