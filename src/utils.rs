// src/utils.rs
use chrono::Offset;
use chrono::TimeZone;
use chrono::{Datelike, FixedOffset, NaiveDate, Utc};
use chrono_tz::Tz;
use std::str::FromStr;
/// Checks if a given year is a leap year.
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Returns the week number of the year for a given date.
pub fn get_week_of_year(date: NaiveDate) -> u32 {
    date.iso_week().week()
}

/// Returns the ordinal suffix for a given day.
pub fn get_ordinal_suffix(day: u32) -> String {
    match day {
        1 | 21 | 31 => "st".to_string(),
        2 | 22 => "nd".to_string(),
        3 | 23 => "rd".to_string(),
        _ => "th".to_string(),
    }
}

/// Formats the timezone offset.
pub fn format_timezone(offset: FixedOffset) -> String {
    let seconds = offset.local_minus_utc();
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    format!("{:+03}:{:02}", hours, minutes)
}

/// Returns the number of days in a given month of a specific year.
pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 => 31,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 30,
    }
}

/// Retrieves the timezone offset for a given timezone string.
pub fn get_timezone_offset(timezone: &str) -> Option<FixedOffset> {
    match Tz::from_str(timezone) {
        Ok(tz) => {
            let offset = tz.offset_from_utc_datetime(&Utc::now().naive_utc());
            Some(offset.fix())
        }
        Err(_) => None,
    }
}
