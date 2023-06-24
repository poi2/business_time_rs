use std::collections::HashMap;

use chrono::NaiveDate;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Holidays(HashMap<NaiveDate, ()>);

impl Holidays {
    pub fn new(holidays: Vec<NaiveDate>) -> Self {
        Self(holidays.into_iter().map(|v| (v, ())).collect())
    }

    pub fn contains(&self, date: &NaiveDate) -> bool {
        self.0.contains_key(date)
    }
}
