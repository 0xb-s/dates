/// Macro to create a new `DateX` instance from date components.
#[macro_export]
macro_rules! datex {
    ($year:expr, $month:expr, $day:expr) => {{
        use chrono::{NaiveDate, Utc};
        match NaiveDate::from_ymd_opt($year, $month, $day) {
            Some(naive_date) => {
                let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
                $crate::DateX {
                    datetime: chrono::DateTime::<Utc>::from_utc(naive_datetime, Utc),
                }
            },
            None => panic!(
                "Invalid date components in datex! macro: year={}, month={}, day={}",
                $year, $month, $day
            ),
        }
    }};
    ($year:expr, $month:expr, $day:expr, $hour:expr, $minute:expr, $second:expr) => {{
        use chrono::{NaiveDate, Utc};
        match NaiveDate::from_ymd_opt($year, $month, $day) {
            Some(naive_date) => {
                match naive_date.and_hms_opt($hour, $minute, $second) {
                    Some(naive_datetime) => $crate::DateX {
                        datetime: chrono::DateTime::<Utc>::from_utc(naive_datetime, Utc),
                    },
                    None => panic!(
                        "Invalid time components in datex! macro: hour={}, minute={}, second={}",
                        $hour, $minute, $second
                    ),
                }
            },
            None => panic!(
                "Invalid date components in datex! macro: year={}, month={}, day={}",
                $year, $month, $day
            ),
        }
    }};
}

/// Macro to parse a date string using a format.

#[macro_export]
macro_rules! parse_datex {
    ($date_str:expr, $format:expr) => {{
        match $crate::DateX::parse($date_str, $format) {
            Ok(date) => date,
            Err(e) => panic!(
                "Failed to parse date string in parse_datex! macro: '{}', error: {}",
                $date_str, e
            ),
        }
    }};
}
