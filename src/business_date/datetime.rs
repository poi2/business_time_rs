use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveTime, TimeZone, Timelike};

use crate::{BusinessDate, BusinessDateDecorator, Calendar};

impl<Tz: TimeZone> BusinessDate for DateTime<Tz> {}

impl<Tz: TimeZone> BusinessDateDecorator for DateTime<Tz> {
    fn add_assign_one_day(&mut self) {
        *self += Duration::days(1);
    }

    fn sub_assign_one_day(&mut self) {
        *self -= Duration::days(1);
    }

    fn move_to_business_start(&mut self, calendar: &Calendar) {
        *self = adjust(self, calendar.business_hour().start);
    }

    fn move_to_business_end(&mut self, calendar: &Calendar) {
        *self = adjust(self, calendar.business_hour().end);
    }

    fn is_before_business_start(&self, calendar: &Calendar) -> bool {
        self.time() < calendar.business_hour().start
    }

    fn is_after_business_end(&self, calendar: &Calendar) -> bool {
        self.time() > calendar.business_hour().end
    }

    fn date_naive(&self) -> NaiveDate {
        Self::date_naive(&self)
    }
}

/// No error due to time transfer on current day.
fn adjust<Tz: TimeZone>(current: &DateTime<Tz>, dist: NaiveTime) -> DateTime<Tz> {
    DateTime::<Tz>::from_utc(
        NaiveDate::from_ymd_opt(current.year(), current.month(), current.day())
            .unwrap()
            .and_hms_opt(dist.hour(), dist.minute(), dist.second())
            .unwrap(),
        current.offset().clone(),
    )
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDate, Utc};
    use rstest::rstest;

    use super::*;

    #[test]
    fn can_add_assign_one_day() {
        let mut date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

        date.add_assign_one_day();

        assert_eq!(date, Utc.with_ymd_and_hms(2023, 1, 2, 0, 0, 0).unwrap());
    }

    #[test]
    fn can_sub_assign_one_day() {
        let mut date = Utc.with_ymd_and_hms(2023, 1, 2, 0, 0, 0).unwrap();

        date.sub_assign_one_day();

        assert_eq!(date, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap());
    }

    #[test]
    fn can_move_to_business_start() {
        let mut date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

        date.move_to_business_start(&Calendar::default());

        assert_eq!(date, Utc.with_ymd_and_hms(2023, 1, 1, 9, 0, 0).unwrap());
    }

    #[test]
    fn can_move_to_business_end() {
        let mut date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

        date.move_to_business_end(&Calendar::default());

        assert_eq!(date, Utc.with_ymd_and_hms(2023, 1, 1, 18, 0, 0).unwrap());
    }

    #[rstest]
    #[case::before(8, 59, 59, true)]
    #[case::just(9, 0, 0, false)]
    #[case::after(9, 0, 1, false)]
    fn can_is_before_business_start(
        #[case] hour: u32,
        #[case] minute: u32,
        #[case] second: u32,
        #[case] expected: bool,
    ) {
        let date = Utc
            .with_ymd_and_hms(2023, 1, 1, hour, minute, second)
            .unwrap();

        assert_eq!(
            date.is_before_business_start(&Calendar::default()),
            expected
        );
    }

    #[rstest]
    #[case::before(17, 59, 59, false)]
    #[case::just(18, 0, 0, false)]
    #[case::after(18, 0, 1, true)]
    fn can_is_after_business_end(
        #[case] hour: u32,
        #[case] minute: u32,
        #[case] second: u32,
        #[case] expected: bool,
    ) {
        let date = Utc
            .with_ymd_and_hms(2023, 1, 1, hour, minute, second)
            .unwrap();

        assert_eq!(date.is_after_business_end(&Calendar::default()), expected);
    }

    #[test]
    fn can_date_naive() {
        let date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

        assert_eq!(
            date.date_naive(),
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
        );
    }
}
