mod datetime;
mod naive_date;
mod naive_date_time;

use chrono::{Datelike, NaiveDate, Weekday};
pub use datetime::*;
pub use naive_date::*;
pub use naive_date_time::*;

use crate::Calendar;

pub trait BusinessDate: BusinessDateDecorator {
    /// Add business days to self.
    fn add_business_days(&self, mut days: usize, calendar: &Calendar) -> Self {
        let mut date = self.clone();

        if !date.is_workday(calendar) || date.is_after_business_end(calendar) {
            days += 1;
            date.move_to_business_start(calendar)
        } else if date.is_before_business_start(calendar) {
            date.move_to_business_start(calendar)
        };

        while days > 0 {
            date.add_assign_one_day();
            if date.is_workday(calendar) {
                days -= 1;
            }
        }

        date
    }

    /// Subtract business days from self.
    fn sub_business_days(&self, mut days: usize, calendar: &Calendar) -> Self {
        let mut date = self.clone();

        if !date.is_workday(calendar) || date.is_before_business_start(calendar) {
            days += 1;
            date.move_to_business_end(calendar)
        } else if date.is_after_business_end(calendar) {
            date.move_to_business_end(calendar)
        };

        while days > 0 {
            date.sub_assign_one_day();
            if date.is_workday(calendar) {
                days -= 1;
            }
        }

        date
    }
}

pub trait BusinessDateDecorator: Datelike + Clone + Sized {
    /// Add one day to self.
    fn add_assign_one_day(&mut self);

    /// Subtract one day from self.
    fn sub_assign_one_day(&mut self);

    /// Move self to business start.
    fn move_to_business_start(&mut self, calendar: &Calendar);

    /// Move self to business end.
    fn move_to_business_end(&mut self, calendar: &Calendar);

    /// Return true if self is before business start.
    fn is_before_business_start(&self, calendar: &Calendar) -> bool;

    /// Return true if self is after business end.
    fn is_after_business_end(&self, calendar: &Calendar) -> bool;

    /// Return NaiveDate of self.
    fn date_naive(&self) -> NaiveDate;

    /// Return true if self is workday.
    fn is_workday(&self, calendar: &Calendar) -> bool {
        calendar.workdays().contains(&self.weekday())
            && !calendar
                .holidays()
                .contains(&BusinessDateDecorator::date_naive(self))
    }

    /// Return true if self is weekend.
    /// Saturday and Sunday are weekend.
    fn is_weekend(&self) -> bool {
        let weekday = self.weekday();
        Weekday::Sat == weekday || Weekday::Sun == weekday
    }

    /// Return true if self is weekday.
    /// Monday, Tuesday, Wednesday, Thursday and Friday are weekday.
    fn is_weekday(&self) -> bool {
        !self.is_weekend()
    }
}

#[cfg(test)]
mod test_for_add_business_days {
    use chrono::{NaiveDate, NaiveDateTime};
    use rstest::rstest;

    use crate::{business_date::BusinessDate, holidays::Holidays, Calendar};

    #[rstest]
    #[case::monday_at_1300_next_business_day_is_tuesday_at_1300(
            NaiveDate::from_ymd_opt(2023, 7, 10)
                .unwrap()
                .and_hms_opt(13, 0, 0)
                .unwrap(),
            NaiveDate::from_ymd_opt(2023, 7, 11)
                .unwrap()
                .and_hms_opt(13, 0, 0)
                .unwrap()
        )]
    #[case::monday_at_0700_next_business_day_is_tuesday_at_0900(
            NaiveDate::from_ymd_opt(2023, 7, 10)
                .unwrap()
                .and_hms_opt(7, 0, 0)
                .unwrap(),
            NaiveDate::from_ymd_opt(2023, 7, 11)
                .unwrap()
                .and_hms_opt(9, 0, 0)
                .unwrap()
        )]
    #[case::monday_at_2000_next_business_day_is_wednesday_at_0900(
            NaiveDate::from_ymd_opt(2023, 7, 10)
                .unwrap()
                .and_hms_opt(20, 0, 0)
                .unwrap(),
            NaiveDate::from_ymd_opt(2023, 7, 12)
                .unwrap()
                .and_hms_opt(9, 0, 0)
                .unwrap()
        )]
    fn add_business_days(#[case] input: NaiveDateTime, #[case] expected: NaiveDateTime) {
        assert_eq!(
            crate::BusinessDate::add_business_days(&input, 1, &Calendar::default()),
            expected
        );
    }

    #[rstest]
    #[case::friday_at_1300_next_business_day_is_four_days_later_at_1300(
            NaiveDate::from_ymd_opt(2023, 7, 14)
                .unwrap()
                .and_hms_opt(13, 0, 0)
                .unwrap(),
            NaiveDate::from_ymd_opt(2023, 7, 18)
                .unwrap()
                .and_hms_opt(13, 0, 0)
                .unwrap()
        )]
    #[case::friday_at_0700_next_business_day_is_four_days_later_at_0900(
            NaiveDate::from_ymd_opt(2023, 7, 14)
                .unwrap()
                .and_hms_opt(7, 0, 0)
                .unwrap(),
            NaiveDate::from_ymd_opt(2023, 7, 18)
                .unwrap()
                .and_hms_opt(9, 0, 0)
                .unwrap()
        )]
    #[case::friday_at_2000_next_business_day_is_five_days_later_at_0900(
            NaiveDate::from_ymd_opt(2023, 7, 14)
                .unwrap()
                .and_hms_opt(20, 0, 0)
                .unwrap(),
            NaiveDate::from_ymd_opt(2023, 7, 19)
                .unwrap()
                .and_hms_opt(9, 0, 0)
                .unwrap()
        )]
    fn skip_holidays_and_weekends_and_add_business_days(
        #[case] input: NaiveDateTime,
        #[case] expected: NaiveDateTime,
    ) {
        let calendar = Calendar::default()
            .set_holidays(Holidays::new(vec![
                NaiveDate::from_ymd_opt(2023, 7, 17).unwrap()
            ]))
            .clone();

        assert_eq!(input.add_business_days(1, &calendar), expected);
    }
}

#[cfg(test)]
mod test_for_sub_business_days {
    use chrono::{NaiveDate, NaiveDateTime};
    use rstest::rstest;

    use crate::{business_date::BusinessDate, holidays::Holidays, Calendar};

    #[rstest]
    #[case::wednesday_at_1300_previous_business_day_is_tuesday_at_1300(
            NaiveDate::from_ymd_opt(2023, 7, 12)
                .unwrap()
                .and_hms_opt(13, 0, 0)
                .unwrap(),
            NaiveDate::from_ymd_opt(2023, 7, 11)
                .unwrap()
                .and_hms_opt(13, 0, 0)
                .unwrap()
        )]
    #[case::wednesday_at_0700_previous_business_day_is_monday_at_1800(
            NaiveDate::from_ymd_opt(2023, 7, 12)
                .unwrap()
                .and_hms_opt(7, 0, 0)
                .unwrap(),
            NaiveDate::from_ymd_opt(2023, 7, 10)
                .unwrap()
                .and_hms_opt(18, 0, 0)
                .unwrap()
        )]
    #[case::wednesday_at_2000_previous_business_day_is_tuesday_at_1800(
            NaiveDate::from_ymd_opt(2023, 7, 12)
                .unwrap()
                .and_hms_opt(20, 0, 0)
                .unwrap(),
            NaiveDate::from_ymd_opt(2023, 7, 11)
                .unwrap()
                .and_hms_opt(18, 0, 0)
                .unwrap()
        )]
    fn sub_business_days(#[case] input: NaiveDateTime, #[case] expected: NaiveDateTime) {
        assert_eq!(
            BusinessDate::sub_business_days(&input, 1, &Calendar::default()),
            expected
        );
    }

    #[rstest]
    #[case::tuesday_at_1300_previous_business_day_is_four_days_ago_at_1300(
            NaiveDate::from_ymd_opt(2023, 7, 18)
                .unwrap()
                .and_hms_opt(13, 0, 0)
                .unwrap(),
            NaiveDate::from_ymd_opt(2023, 7, 14)
                .unwrap()
                .and_hms_opt(13, 0, 0)
                .unwrap()
        )]
    #[case::tuesday_at_0700_previous_business_day_is_five_days_ago_at_1800(
            NaiveDate::from_ymd_opt(2023, 7, 18)
                .unwrap()
                .and_hms_opt(7, 0, 0)
                .unwrap(),
            NaiveDate::from_ymd_opt(2023, 7, 13)
                .unwrap()
                .and_hms_opt(18, 0, 0)
                .unwrap()
        )]
    #[case::tuesday_at_2000_previous_business_day_is_four_days_later_at_1800(
            NaiveDate::from_ymd_opt(2023, 7, 18)
                .unwrap()
                .and_hms_opt(20, 0, 0)
                .unwrap(),
            NaiveDate::from_ymd_opt(2023, 7, 14)
                .unwrap()
                .and_hms_opt(18, 0, 0)
                .unwrap()
        )]
    fn skip_holidays_and_weekends_and_sub_business_days(
        #[case] input: NaiveDateTime,
        #[case] expected: NaiveDateTime,
    ) {
        let calendar = Calendar::default()
            .set_holidays(Holidays::new(vec![
                NaiveDate::from_ymd_opt(2023, 7, 17).unwrap()
            ]))
            .clone();

        assert_eq!(input.sub_business_days(1, &calendar), expected);
    }
}
