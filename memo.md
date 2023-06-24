    // pub trait BusinessDaysFromNowWithTimeZone<Tz: TimeZone, DATE> {
    //     fn business_days_from_now_with_offset(
    //         self,
    //         business_days: &BusinessDays,
    //         offset: Tz::Offset,
    //     ) -> DATE;
    // }

    // impl<Tz: TimeZone> BusinessDaysFromNowWithTimeZone<Tz, DateTime<Tz>> for i32 {
    //     fn business_days_from_now_with_offset(
    //         self,
    //         business_days: &BusinessDays,
    //         offset: Tz::Offset,
    //     ) -> DateTime<Tz> {
    //         let now = DateTime::from_utc(Utc::now().naive_utc(), offset);

    //         if self >= 0 {
    //             now.add_business_days(self as usize, business_days)
    //         } else {
    //             now.sub_business_days(self.abs() as usize, business_days)
    //         }
    //     }
    // }

    // pub trait BusinessDaysFromNow<DATE> {
    //     fn business_days_from_now(self, business_days: &BusinessDays) -> DATE;
    // }

    // impl BusinessDaysFromNow<NaiveDate> for i32 {
    //     fn business_days_from_now(self, business_days: &BusinessDays) -> NaiveDate {
    //         let now = Utc::now().naive_utc().date();

    //         if self >= 0 {
    //             now.add_business_days(self as usize, business_days)
    //         } else {
    //             now.sub_business_days(self.abs() as usize, business_days)
    //         }
    //     }
    // }

    // impl BusinessDaysFromNow<NaiveDateTime> for i32 {
    //     fn business_days_from_now(self, business_days: &BusinessDays) -> NaiveDateTime {
    //         let now = Utc::now().naive_utc();

    //         if self >= 0 {
    //             now.add_business_days(self as usize, business_days)
    //         } else {
    //             now.sub_business_days(self.abs() as usize, business_days)
    //         }
    //     }
    // }


    // #[test]
    // fn business_days_from_now_work_properly_for_native_date() {
    //     let _date: NaiveDate = 1.business_days_from_now(&business_days());
    //     let _date: NaiveDate = (-1).business_days_from_now(&business_days());
    // }

    // #[test]
    // fn business_days_from_now_work_properly_for_native_date_time() {
    //     let _date_time: NaiveDateTime = 1.business_days_from_now(&business_days());
    //     let _date_time: NaiveDateTime = (-1).business_days_from_now(&business_days());
    // }

    // #[test]
    // fn business_days_from_now_with_offset_work_properly() {
    //     let _date_time: DateTime<FixedOffset> = 1.business_days_from_now_with_offset(
    //         &business_days(),
    //         FixedOffset::east_opt(9 * 60 * 60).unwrap(),
    //     );
    //     let _date_time: DateTime<FixedOffset> = (-1).business_days_from_now_with_offset(
    //         &business_days(),
    //         FixedOffset::east_opt(9 * 60 * 60).unwrap(),
    //     );
    // }
