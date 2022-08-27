use crate::types::stats::Stats;
use crate::types::transition::Transition;
use crate::types::vacation::Vacation;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub vacations: Vec<Vacation>,
    pub transitions: Vec<Transition>,
    pub stats: Stats,
}
