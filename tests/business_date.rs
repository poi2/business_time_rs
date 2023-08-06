#[test]
fn readme() {
    use business_time_rs::{BusinessDate, BusinessHour, Calendar, Holidays, Workdays};
    use chrono::{NaiveDate, NaiveTime, TimeZone, Utc, Weekday};

    // You can define custom holidays, workdays and business hours.
    let calendar = Calendar::builder()
        .holidays(Holidays::new(vec![
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(),
        ]))
        .workdays(Workdays::new(vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
        ]))
        .business_hour(BusinessHour::new(
            NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
        ))
        .build();

    // New Year's Day 2023 is Sunday.
    let new_year_day = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

    // One business day later is January 4, 2023.
    // Because next business day is January 3 (January 2 is holiday).
    // The next day is January 4 and if it is before business hours, return the business start time.
    assert_eq!(
        new_year_day.add_business_days(1, &calendar),
        Utc.with_ymd_and_hms(2023, 1, 4, 9, 0, 0).unwrap()
    );

    // The next business day after tomorrow is January 4, 2023.
    assert_eq!(
        new_year_day.add_business_days(2, &calendar),
        Utc.with_ymd_and_hms(2023, 1, 5, 9, 0, 0).unwrap()
    );

    // If 0 days is entered, it returns, if current time is during business hours, return current time.
    assert_eq!(
        Utc.with_ymd_and_hms(2023, 1, 4, 12, 0, 0)
            .unwrap()
            .add_business_days(0, &calendar),
        Utc.with_ymd_and_hms(2023, 1, 4, 12, 0, 0).unwrap()
    );

    // But if current time is outside of business hours, return next business start hours.
    assert_eq!(
        Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0)
            .unwrap()
            .add_business_days(0, &calendar),
        Utc.with_ymd_and_hms(2023, 1, 3, 9, 0, 0).unwrap()
    );

    // One business day before is December 29, 2022.
    // Because before business day is December 30 (December 31 is Saturday).
    // The before day is December 29 and if it is after business hours, return the business end time.
    assert_eq!(
        new_year_day.sub_business_days(1, &calendar),
        Utc.with_ymd_and_hms(2022, 12, 29, 18, 0, 0).unwrap()
    );

    // If 0 days entered, it returns, if current time is during business hours, return current time.
    assert_eq!(
        Utc.with_ymd_and_hms(2023, 1, 4, 12, 0, 0)
            .unwrap()
            .sub_business_days(0, &calendar),
        Utc.with_ymd_and_hms(2023, 1, 4, 12, 0, 0).unwrap()
    );

    // But if current time is outside of business hours, return next business end hours.
    assert_eq!(
        new_year_day.sub_business_days(0, &calendar),
        Utc.with_ymd_and_hms(2022, 12, 30, 18, 0, 0).unwrap()
    );

    // And you can write like this.
    // assert_eq!(
    //     1.business_days_after(&new_year_day, &calendar),
    //     new_year_day.add_business_days(1, &calendar),
    // );

    // assert_eq!(
    //     1.business_days_before(&new_year_day, &calendar),
    //     new_year_day.sub_business_days(1, &calendar),
    // );
}
