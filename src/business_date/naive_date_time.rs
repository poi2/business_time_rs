use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

use crate::{BusinessDate, BusinessDateDecorator, Calendar};

impl BusinessDate for NaiveDateTime {}

impl BusinessDateDecorator for NaiveDateTime {
    fn add_assign_one_day(&mut self) {
        *self += Duration::days(1);
    }

    fn sub_assign_one_day(&mut self) {
        *self -= Duration::days(1);
    }

    fn move_to_business_start(&mut self, calendar: &Calendar) {
        *self = change_hms(self, calendar.business_hour().start);
    }

    fn move_to_business_end(&mut self, calendar: &Calendar) {
        *self = change_hms(self, calendar.business_hour().end)
    }

    fn is_before_business_start(&self, calendar: &Calendar) -> bool {
        self.time() < calendar.business_hour().start
    }

    fn is_after_business_end(&self, calendar: &Calendar) -> bool {
        self.time() > calendar.business_hour().end
    }

    fn date_naive(&self) -> NaiveDate {
        Self::date(&self)
    }
}

fn change_hms(current: &NaiveDateTime, dist: NaiveTime) -> NaiveDateTime {
    NaiveDate::from_ymd_opt(current.year(), current.month(), current.day())
        .unwrap()
        .and_hms_opt(dist.hour(), dist.minute(), dist.second())
        .unwrap()
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;
    use rstest::rstest;

    use super::*;

    #[test]
    fn can_add_assign_one_day() {
        let mut date = NaiveDate::from_ymd_opt(2023, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        date.add_assign_one_day();

        assert_eq!(
            date,
            NaiveDate::from_ymd_opt(2023, 1, 2)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        );
    }

    #[test]
    fn can_sub_assign_one_day() {
        let mut date = NaiveDate::from_ymd_opt(2023, 1, 2)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        date.sub_assign_one_day();

        assert_eq!(
            date,
            NaiveDate::from_ymd_opt(2023, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        );
    }

    #[test]
    fn can_move_to_business_start() {
        let mut date = NaiveDate::from_ymd_opt(2023, 7, 3)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();

        date.move_to_business_start(&Calendar::default());

        assert_eq!(
            date,
            NaiveDate::from_ymd_opt(2023, 7, 3)
                .unwrap()
                .and_hms_opt(9, 0, 0)
                .unwrap()
        );
    }

    #[test]
    fn can_move_to_business_end() {
        let mut date = NaiveDate::from_ymd_opt(2023, 7, 3)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();

        date.move_to_business_end(&Calendar::default());

        assert_eq!(
            date,
            NaiveDate::from_ymd_opt(2023, 7, 3)
                .unwrap()
                .and_hms_opt(18, 0, 0)
                .unwrap()
        );
    }

    #[rstest]
    #[case::before_business_start(
        NaiveDate::from_ymd_opt(2023, 7, 3)
            .unwrap()
            .and_hms_opt(8, 59, 59)
            .unwrap(),
        true,
    )]
    #[case::after_business_start(
        NaiveDate::from_ymd_opt(2023, 7, 3)
            .unwrap()
            .and_hms_opt(9, 0, 0)
            .unwrap(),
        false,
    )]
    fn before_business_start(#[case] input: NaiveDateTime, #[case] expected: bool) {
        assert_eq!(
            input.is_before_business_start(&Calendar::default()),
            expected
        );
    }

    #[rstest]
    #[case::after_business_end(
        NaiveDate::from_ymd_opt(2023, 7, 3)
            .unwrap()
            .and_hms_opt(18, 0, 1)
            .unwrap(),
        true,
    )]
    #[case::before_business_end(
        NaiveDate::from_ymd_opt(2023, 7, 3)
            .unwrap()
            .and_hms_opt(18, 0, 0)
            .unwrap(),
        false,
    )]
    fn after_business_end(#[case] input: NaiveDateTime, #[case] expected: bool) {
        assert_eq!(input.is_after_business_end(&Calendar::default()), expected);
    }

    #[test]
    fn can_date_naive() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        assert_eq!(
            date.date_naive(),
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
        );
    }
}
