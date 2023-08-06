use derive_getters::Getters;
use derive_new::new;
use getset::Setters;
use typed_builder::TypedBuilder;

use crate::{BusinessHour, Holidays, Workdays};

#[derive(Clone, Debug, Getters, Setters, new, TypedBuilder)]
#[getset(set = "pub")]
/// Calendar is a collection of BusinessHour, Workdays and Holidays.
pub struct Calendar {
    pub holidays: Holidays,
    pub workdays: Workdays,
    pub business_hour: BusinessHour,
}

#[cfg(any(test, feature = "test"))]
impl Default for Calendar {
    fn default() -> Self {
        Self {
            holidays: Holidays::default(),
            workdays: Workdays::default(),
            business_hour: BusinessHour::default(),
        }
    }
}
