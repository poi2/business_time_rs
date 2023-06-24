use std::collections::HashMap;

use chrono::Weekday;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Workdays(HashMap<Weekday, ()>);

impl Workdays {
    pub fn new(workdays: Vec<Weekday>) -> Self {
        Self(workdays.into_iter().map(|v| (v, ())).collect())
    }

    pub fn contains(&self, weekday: &Weekday) -> bool {
        self.0.contains_key(weekday)
    }
}

impl Default for Workdays {
    fn default() -> Self {
        Self::new(vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
        ])
    }
}
