use chrono::{Duration, NaiveDate};

use crate::{BusinessDate, BusinessDateDecorator, Calendar};

impl BusinessDate for NaiveDate {}

impl BusinessDateDecorator for NaiveDate {
    fn add_assign_one_day(&mut self) {
        *self += Duration::days(1);
    }

    fn sub_assign_one_day(&mut self) {
        *self -= Duration::days(1);
    }

    fn move_to_business_start(&mut self, _calendar: &Calendar) {
        // Noop
    }

    fn move_to_business_end(&mut self, _calendar: &Calendar) {
        // Noop
    }

    /// Return false since NaiveDate has no time.
    fn is_before_business_start(&self, _calendar: &Calendar) -> bool {
        false
    }

    /// Return false since NaiveDate has no time.
    fn is_after_business_end(&self, _calendar: &Calendar) -> bool {
        false
    }

    fn date_naive(&self) -> NaiveDate {
        self.clone()
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn can_add_assign_one_day() {
        let mut date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();

        date.add_assign_one_day();

        assert_eq!(date, NaiveDate::from_ymd_opt(2023, 1, 2).unwrap());
    }

    #[test]
    fn can_sub_assign_one_day() {
        let mut date = NaiveDate::from_ymd_opt(2023, 1, 2).unwrap();

        date.sub_assign_one_day();

        assert_eq!(date, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
    }

    #[test]
    fn always_not_before_business_start() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();

        assert!(!date.is_before_business_start(&Calendar::default()));
    }

    #[test]
    fn always_not_after_business_end() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();

        assert!(!date.is_after_business_end(&Calendar::default()));
    }

    #[test]
    fn can_date_naive() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();

        assert_eq!(
            date.date_naive(),
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
        );
    }
}
