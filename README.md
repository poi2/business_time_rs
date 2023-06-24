# BusinessTime

BusinessTime is a library for handling business days calculations, allowing you to add or subtract business days from a given date.
And you can define custom workdays and holidays.

## Uses

BusinessTime support the following date types.

- `chrono::DateTime<Tz>`
- `chrono::NaiveDateTime`
- `chrono::NaiveDate`

```rust
use business_time::{BusinessDate, Holidays, Workdays};
use chrono::{NaiveDate, TimeZone, Utc, Weekday};

// You can define custom workdays.
let workdays = Workdays::new(vec![
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
]);
// You can define custom holidays.
let holidays = Holidays::new(vec![
    NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
    NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(),
]);

// New Year's Day 2023 is Sunday.
let new_year_day = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

// The next business day is January 3, 2023.
// Because January 2, 2023 is holiday.
assert_eq!(
    new_year_day.add_business_days(1, &workdays, &holidays),
    Utc.with_ymd_and_hms(2023, 1, 3, 0, 0, 0).unwrap()
);

// The next business day after tomorrow is January 4, 2023.
assert_eq!(
    new_year_day.add_business_days(1, &workdays, &holidays),
    Utc.with_ymd_and_hms(2023, 1, 4, 0, 0, 0).unwrap()
);

// The previous business day is December 30, 2022.
// Because December 31, 2022 is Saturday.
assert_eq!(
    new_year_day.sub_business_days(1, &workdays, &holidays),
    Utc.with_ymd_and_hms(2022, 12, 30, 0, 0, 0).unwrap()
);

// And you can write like this.
assert_eq!(
    new_year_day.add_business_days(1, &workdays, &holidays),
    1.business_days_after(&new_year_day, &workdays, &holidays)
);

assert_eq!(
    new_year_day.sub_business_days(1, &workdays, &holidays),
    1.business_days_before(&new_year_day, &workdays, &holidays)
);
```

---

```rust
let now = Utc.now();

// in friday in business hour + 1 business_day => monday 09:00:00 or 00:00:00
// in friday after business hour + 1 business_day => monday 18:00:00 or tuesday 09:00:00 or 00:00:00

// Similar as English sentences.
let one_business_days_later      = 1_u32.business_days().after(&now, &calendar);
let two_business_hours_later     = 2_u32.business_hours().after(&now, &calendar);
let three_business_minutes_later = 3_u32.business_minutes().after(&now, &calendar);

let one_business_days_ago      = 1_u32.business_days().before(&now, &calendar);
let two_business_hours_ago     = 2_u32.business_hours().before(&now, &calendar);
let three_business_minutes_ago = 3_u32.business_minutes().before(&now, &calendar);

// Object oriented style.
let one_business_days_two_hours_and_three_minutes_later = now
    .add_business_days(1, &calendar)
    .add_business_hours(2, &calendar)
    .add_business_minutes(3, &calendar)
    .

let one_business_days_two_hours_and_three_minutes_ago = now
    .sub_business_days(1, &calendar)
    .sub_business_hours(2, &calendar)
    .sub_business_minutes(3, &calendar);

// Mathematical style.
let one_business_days_two_hours_and_three_minutes_later =
    now + (1_u32.business_days(), &calendar) + (2_u32.business_hours(), &calendar) + (3_u32.business_minutes(), &calendar);
```

---

```
09:00-18:00 is business hour.

Friday 08:59 + 1.business_days() => Monday 09:00
Friday 18:00 + 1.business_days() => Monday 18:00
Friday 18:01 + 1.business_days() => Tuesday 09:00
Monday 09:00 + 1.business_days() => Tuesday 09:00

if current_time is in business hour
  current_time + 1.business_days()
else current_time is NOT in business hour
  next_business_opening_time() + 1.business_days()
end

Tuesday 09:00 - 1.business_days() => Monday 09:00
Monday 18:01 - 1.business_days() => Friday 18:00
Monday 18:00 - 1.business_days() => Friday 18:00
Monday 09:00 - 1.business_days() => Friday 09:00
Monday 08:59 - 1.business_days() => Thursday 18:00

if current_time is in business hour
  current_time - 1.business_days()
else current_time is NOT in business hour
  previous_business_closing_time() - 1.business_days()
end
```
