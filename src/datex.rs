// src/datex.rs
use crate::durationx::DurationX;
use crate::formatter::DateFormatter;
use crate::locale::LocaleManager;
use crate::manipulator::{DurationUnit, TimeManipulator};
use crate::parser::DateParser;
use crate::timezone::TimeZoneHandler;
use crate::utils::*;
use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};
use chrono_tz::Tz;
use std::fmt;
use std::ops::{Add, Sub};

/// A versatile date and time struct that provides extensive functionality for parsing, formatting, and manipulating dates and times.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateX {
    pub datetime: DateTime<Utc>,
}

impl DateX {
    /// Creates a new `DateX` instance representing the current date and time.
    pub fn now() -> Self {
        DateX {
            datetime: Utc::now(),
        }
    }

    pub fn to_timezone(&self, timezone: &str) -> Result<DateX, String> {
        match timezone.parse::<Tz>() {
            Ok(tz) => {
                let datetime_in_tz = self.datetime.with_timezone(&tz);
                // Convert back to UTC for internal consistency
                let datetime_utc = datetime_in_tz.with_timezone(&Utc);
                Ok(DateX {
                    datetime: datetime_utc,
                })
            }
            Err(_) => Err("Invalid timezone".to_string()),
        }
    }

    /// Returns the localized datetime in the specified timezone as a string.
    pub fn format_in_timezone(&self, format: &str, timezone: &str) -> Result<String, String> {
        match timezone.parse::<Tz>() {
            Ok(tz) => {
                let datetime_in_tz = self.datetime.with_timezone(&tz);
                Ok(datetime_in_tz.format(format).to_string())
            }
            Err(_) => Err("Invalid timezone".to_string()),
        }
    }

    /// Creates a `DateX` instance from a string and a format.
    pub fn parse(date_str: &str, format: &str) -> Result<Self, String> {
        match DateParser::parse(date_str, format) {
            Ok(datetime) => Ok(DateX { datetime }),
            Err(e) => Err(e),
        }
    }

    /// Creates a `DateX` instance from a timestamp in seconds.
    pub fn from_timestamp(timestamp: i64) -> Self {
        let datetime = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(timestamp, 0)
                .unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0)),
            Utc,
        );
        DateX { datetime }
    }

    /// Formats the date according to the specified format string.
    pub fn format(&self, format_str: &str) -> String {
        DateFormatter::format(self, format_str)
    }

    /// Returns the year.
    pub fn year(&self) -> i32 {
        self.datetime.year()
    }

    /// Returns the month (1-12).
    pub fn month(&self) -> u32 {
        self.datetime.month()
    }

    /// Returns the day of the month (1-31).
    pub fn day(&self) -> u32 {
        self.datetime.day()
    }

    /// Returns the hour (0-23).
    pub fn hour(&self) -> u32 {
        self.datetime.hour()
    }

    /// Returns the minute (0-59).
    pub fn minute(&self) -> u32 {
        self.datetime.minute()
    }

    /// Returns the second (0-59).
    pub fn second(&self) -> u32 {
        self.datetime.second()
    }

    /// Returns the millisecond (0-999).
    pub fn millisecond(&self) -> u32 {
        self.datetime.timestamp_subsec_millis()
    }

    /// Adds a specified duration to the date.
    pub fn add_duration(&mut self, amount: i64, unit: DurationUnit) {
        self.datetime = TimeManipulator::add_duration(self.datetime, amount, unit);
    }

    /// Subtracts a specified duration from the date.
    pub fn subtract_duration(&mut self, amount: i64, unit: DurationUnit) {
        self.datetime = TimeManipulator::subtract_duration(self.datetime, amount, unit);
    }

    /// Checks if the date is a leap year.
    pub fn is_leap_year(&self) -> bool {
        is_leap_year(self.datetime.year())
    }

    /// Returns the week number of the year.
    pub fn week_of_year(&self) -> u32 {
        get_week_of_year(self.datetime.naive_utc().date())
    }

    /// Returns the ordinal suffix for the day.
    pub fn ordinal_suffix(&self) -> String {
        get_ordinal_suffix(self.day())
    }

    /// Returns the day of the week (Monday = 1, Sunday = 7).
    pub fn day_of_week(&self) -> u32 {
        self.datetime.weekday().number_from_monday()
    }

    /// Returns the ISO week number of the year.
    pub fn iso_week(&self) -> u32 {
        self.datetime.iso_week().week()
    }

    /// Returns the number of days in the month.
    pub fn days_in_month(&self) -> u32 {
        days_in_month(self.year(), self.month())
    }

    /// Returns the number of days in the year.
    pub fn days_in_year(&self) -> u32 {
        if self.is_leap_year() {
            366
        } else {
            365
        }
    }

    /// Checks if the date is in the past.
    pub fn is_past(&self) -> bool {
        self.datetime < Utc::now()
    }

    /// Checks if the date is in the future.
    pub fn is_future(&self) -> bool {
        self.datetime > Utc::now()
    }

    /// Checks if the date is today.
    pub fn is_today(&self) -> bool {
        self.datetime.date() == Utc::now().date()
    }

    /// Checks if the date is yesterday.
    pub fn is_yesterday(&self) -> bool {
        self.datetime.date() == (Utc::now() - chrono::Duration::days(1)).date()
    }

    /// Checks if the date is tomorrow.
    pub fn is_tomorrow(&self) -> bool {
        self.datetime.date() == (Utc::now() + chrono::Duration::days(1)).date()
    }

    /// Checks if the date is a weekend.
    pub fn is_weekend(&self) -> bool {
        let weekday = self.datetime.weekday();
        weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun
    }

    /// Returns the timestamp in seconds.
    pub fn timestamp(&self) -> i64 {
        self.datetime.timestamp()
    }

    /// Returns the timestamp in milliseconds.
    pub fn timestamp_millis(&self) -> i64 {
        self.datetime.timestamp_millis()
    }

    /// Converts the date to ISO 8601 format.
    pub fn to_iso8601(&self) -> String {
        self.datetime.to_rfc3339()
    }

    /// Converts the date to UTC.
    pub fn to_utc(&self) -> Self {
        DateX {
            datetime: self.datetime.with_timezone(&Utc),
        }
    }

    /// Converts the date to a different time zone.
    pub fn to_timezone_v(&self, timezone: &str) -> Result<Self, String> {
        match get_timezone_offset(timezone) {
            Some(offset) => {
                let datetime = self.datetime.with_timezone(&offset);
                Ok(DateX {
                    datetime: datetime.into(),
                })
            }
            None => Err("Invalid timezone".to_string()),
        }
    }

    /// Returns the difference between two dates in the specified unit.
    pub fn diff(&self, other: &DateX, unit: DurationUnit) -> i64 {
        TimeManipulator::diff(self.datetime, other.datetime, unit)
    }

    /// Checks if the date is the same as another date in terms of a specific unit.
    pub fn is_same(&self, other: &DateX, unit: DurationUnit) -> bool {
        TimeManipulator::is_same(self.datetime, other.datetime, unit)
    }

    /// Checks if the date is before another date.
    pub fn is_before(&self, other: &DateX) -> bool {
        self.datetime < other.datetime
    }

    /// Checks if the date is after another date.
    pub fn is_after(&self, other: &DateX) -> bool {
        self.datetime > other.datetime
    }

    /// Checks if the date is between two dates.
    pub fn is_between(&self, start: &DateX, end: &DateX) -> bool {
        self.datetime >= start.datetime && self.datetime <= end.datetime
    }

    /// Rounds the date to the nearest minute.
    pub fn round_to_nearest_minute(&mut self) {
        if self.second() >= 30 {
            self.add_duration(1, DurationUnit::Minutes);
        }
        self.datetime = self.datetime.with_second(0).unwrap();
        self.datetime = self.datetime.with_nanosecond(0).unwrap();
    }

    pub fn format_with_locale(&self, format_str: &str, locale_code: &str) -> String {
        let locale_manager = LocaleManager::new();
        locale_manager.format_with_locale(&self.datetime, format_str, locale_code)
    }

    /// Gets the offset from UTC for the current time zone.
    pub fn timezone_offset(&self, timezone: &str) -> Result<chrono::FixedOffset, String> {
        TimeZoneHandler::get_timezone_offset(self.datetime, timezone)
    }
    /// Adds a DurationX to the date.
    pub fn add_durationx(&mut self, durationx: DurationX) {
        self.datetime = self.datetime + durationx.duration();
    }

    /// Subtracts a DurationX from the date.
    pub fn subtract_durationx(&mut self, durationx: DurationX) {
        self.datetime = self.datetime - durationx.duration();
    }

    /// Lists all available time zones.
    pub fn available_timezones() -> Vec<&'static str> {
        TimeZoneHandler::list_timezones()
    }

    /// Rounds the date to the nearest hour.
    pub fn round_to_nearest_hour(&mut self) {
        if self.minute() >= 30 {
            self.add_duration(1, DurationUnit::Hours);
        }
        self.datetime = self.datetime.with_minute(0).unwrap();
        self.datetime = self.datetime.with_second(0).unwrap();
        self.datetime = self.datetime.with_nanosecond(0).unwrap();
    }

    /// Sets the date to the start of the day.
    pub fn start_of_day(&mut self) {
        self.datetime = self.datetime.with_hour(0).unwrap();
        self.datetime = self.datetime.with_minute(0).unwrap();
        self.datetime = self.datetime.with_second(0).unwrap();
        self.datetime = self.datetime.with_nanosecond(0).unwrap();
    }

    /// Sets the date to the end of the day.
    pub fn end_of_day(&mut self) {
        self.datetime = self.datetime.with_hour(23).unwrap();
        self.datetime = self.datetime.with_minute(59).unwrap();
        self.datetime = self.datetime.with_second(59).unwrap();
        self.datetime = self.datetime.with_nanosecond(999_999_999).unwrap();
    }

    /// Sets the date to the start of the month.
    pub fn start_of_month(&mut self) {
        self.datetime = self.datetime.with_day(1).unwrap();
        self.start_of_day();
    }

    /// Sets the date to the end of the month.
    pub fn end_of_month(&mut self) {
        let days_in_month = days_in_month(self.year(), self.month());
        self.datetime = self.datetime.with_day(days_in_month).unwrap();
        self.end_of_day();
    }

    /// Sets the date to the start of the year.
    pub fn start_of_year(&mut self) {
        self.datetime = self.datetime.with_month(1).unwrap();
        self.start_of_month();
    }

    /// Sets the date to the end of the year.
    pub fn end_of_year(&mut self) {
        self.datetime = self.datetime.with_month(12).unwrap();
        self.end_of_month();
    }

    /// Adds business days to the date, skipping weekends.
    pub fn add_business_days(&mut self, days: i64) {
        let mut days_added = 0;
        while days_added < days {
            self.add_duration(1, DurationUnit::Days);
            if !self.is_weekend() {
                days_added += 1;
            }
        }
    }

    /// Subtracts business days from the date, skipping weekends.
    pub fn subtract_business_days(&mut self, days: i64) {
        let mut days_subtracted = 0;
        while days_subtracted < days {
            self.subtract_duration(1, DurationUnit::Days);
            if !self.is_weekend() {
                days_subtracted += 1;
            }
        }
    }

    /// Returns a human-readable relative time string.
    pub fn time_ago(&self) -> String {
        let now = Utc::now();
        let diff = now - self.datetime;
        let seconds = diff.num_seconds();
        let minutes = diff.num_minutes();
        let hours = diff.num_hours();
        let days = diff.num_days();

        if seconds < 60 {
            format!("{} seconds ago", seconds)
        } else if minutes < 60 {
            format!("{} minutes ago", minutes)
        } else if hours < 24 {
            format!("{} hours ago", hours)
        } else {
            format!("{} days ago", days)
        }
    }

    /// Returns a human-readable relative time string from now.
    pub fn from_now(&self) -> String {
        let now = Utc::now();
        let diff = self.datetime - now;
        let seconds = diff.num_seconds();
        let minutes = diff.num_minutes();
        let hours = diff.num_hours();
        let days = diff.num_days();

        if seconds < 60 {
            format!("in {} seconds", seconds)
        } else if minutes < 60 {
            format!("in {} minutes", minutes)
        } else if hours < 24 {
            format!("in {} hours", hours)
        } else {
            format!("in {} days", days)
        }
    }

    /// Returns the date as a NaiveDateTime.
    pub fn to_naive_datetime(&self) -> NaiveDateTime {
        self.datetime.naive_utc()
    }

    /// Returns the date as a DateTime<Utc>.
    pub fn to_datetime(&self) -> DateTime<Utc> {
        self.datetime
    }

    /// Returns the date as a formatted string.
    pub fn to_string(&self) -> String {
        self.format("%Y-%m-%d %H:%M:%S")
    }
}

impl fmt::Display for DateX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted = self.format("%Y-%m-%d %H:%M:%S");
        write!(f, "{}", formatted)
    }
}

impl Add<chrono::Duration> for DateX {
    type Output = DateX;

    fn add(self, rhs: chrono::Duration) -> Self::Output {
        DateX {
            datetime: self.datetime + rhs,
        }
    }
}

impl Sub<chrono::Duration> for DateX {
    type Output = DateX;

    fn sub(self, rhs: chrono::Duration) -> Self::Output {
        DateX {
            datetime: self.datetime - rhs,
        }
    }
}
