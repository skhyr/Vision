use std::hash::Hash;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Countries {
    PL,
}

pub struct YearDay {
    pub day: i32,
    pub month: i32,
}

impl YearDay {
    pub fn from_dm(d: i32, m: i32) -> YearDay {
        YearDay { day: d, month: m }
    }
}
