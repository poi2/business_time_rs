use derive_getters::Getters;
use getset::Setters;

use crate::{BusinessHour, Holidays, Workdays};

#[derive(Clone, Debug, Getters, Setters)]
#[getset(set = "pub")]
/// Calendar is a collection of BusinessHour, Workdays and Holidays.
pub struct Calendar {
    pub business_hour: BusinessHour,
    pub workdays: Workdays,
    pub holidays: Holidays,
}

#[cfg(any(test, feature = "test"))]
impl Default for Calendar {
    fn default() -> Self {
        Self {
            business_hour: BusinessHour::default(),
            workdays: Workdays::default(),
            holidays: Holidays::default(),
        }
    }
}
