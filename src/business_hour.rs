use chrono::NaiveTime;

#[derive(Clone, Debug)]
pub struct BusinessHour {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

#[cfg(any(test, feature = "test"))]
impl Default for BusinessHour {
    fn default() -> Self {
        Self {
            start: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            end: NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
        }
    }
}

impl BusinessHour {
    pub fn new(start: NaiveTime, end: NaiveTime) -> Self {
        if start < end {
            Self { start, end }
        } else {
            Self { end, start }
        }
    }

    pub fn hours_per_day(&self) -> usize {
        (self.end - self.start).num_hours() as usize
    }

    pub fn contains(&self, time: NaiveTime) -> bool {
        !self.is_before_business(time) && !self.is_after_business(time)
    }

    pub fn is_before_business(&self, time: NaiveTime) -> bool {
        time < self.start
    }

    pub fn is_after_business(&self, time: NaiveTime) -> bool {
        self.end < time
    }

    pub fn remaining_hours(&self, time: NaiveTime) -> usize {
        if self.is_before_business(time) {
            self.hours_per_day()
        } else if self.is_after_business(time) {
            0
        } else {
            (self.end - time).num_hours() as usize
        }
    }

    pub fn elaspse_hours(&self, time: NaiveTime) -> usize {
        if self.is_before_business(time) {
            0
        } else if self.is_after_business(time) {
            self.hours_per_day()
        } else {
            (time - self.start).num_hours() as usize
        }
    }
}
