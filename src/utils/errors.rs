#[derive(Debug)]
pub enum Errors {
    InsertFailed,
    QueryFailed,
    Unauthorized,
    InvalidVacationFound,
    InvalidId,
    InvalidDate,
}
