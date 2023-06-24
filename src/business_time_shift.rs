use crate::{BusinessDate, Calendar};

pub enum BusinessTimeDuration {
    Day(usize),
    Hour(usize),
    Minute(usize),
}

pub trait IntoBusinessTimeDuration {
    fn business_days(self) -> BusinessTimeDuration;
    fn business_hours(self) -> BusinessTimeDuration;
    fn business_minutes(self) -> BusinessTimeDuration;
}

macro_rules! impl_into_business_time_duration {
    ($($t:ty),*) => {
        $(
            impl IntoBusinessTimeDuration for $t {
                fn business_days(self) -> BusinessTimeDuration {
                    BusinessTimeDuration::Day(self as usize)
                }
                fn business_hours(self) -> BusinessTimeDuration {
                    BusinessTimeDuration::Hour(self as usize)
                }
                fn business_minutes(self) -> BusinessTimeDuration {
                    BusinessTimeDuration::Minute(self as usize)
                }
            }
        )*
    };
}

impl_into_business_time_duration!(u8, u16, u32, u64, u128, usize);

pub trait BusinessTimeShift<DATE> {
    fn after(self, date: DATE, calendar: &Calendar) -> DATE;
    fn before(self, date: DATE, calendar: &Calendar) -> DATE;
}

impl<DATE> BusinessTimeShift<DATE> for BusinessTimeDuration
where
    DATE: BusinessDate,
{
    fn after(self, date: DATE, calendar: &Calendar) -> DATE {
        match self {
            BusinessTimeDuration::Day(days) => date.add_business_days(days, calendar),
            _ => unimplemented!(),
        }
    }

    fn before(self, date: DATE, calendar: &Calendar) -> DATE {
        match self {
            BusinessTimeDuration::Day(days) => date.sub_business_days(days, calendar),
            _ => unimplemented!(),
        }
    }
}

// #[cfg(test)]
// mod test {
//     use chrono::{TimeZone, Utc};

//     use crate::{holidays::test_util::holidays, workdays::test_util::workdays};

//     use super::*;

//     #[test]
//     fn after() {
//         assert_eq!(
//             1_u32.business_days().after(
//                 Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
//                 &workdays(),
//                 &holidays()
//             ),
//             Utc.with_ymd_and_hms(2023, 1, 3, 0, 0, 0).unwrap()
//         )
//     }
//     #[test]
//     fn before() {
//         assert_eq!(
//             1_u32.business_days().before(
//                 Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
//                 &workdays(),
//                 &holidays()
//             ),
//             Utc.with_ymd_and_hms(2022, 12, 30, 0, 0, 0).unwrap()
//         )
//     }
// }
