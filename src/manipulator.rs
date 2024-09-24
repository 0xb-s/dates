// src/manipulator.rs
use chrono::Datelike;
use chrono::Timelike;
use chrono::{DateTime, Duration, Utc};

/// Enumeration of duration units for time manipulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationUnit {
    Years,
    Months,
    Weeks,
    Days,
    Hours,
    Minutes,
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
}

/// A struct responsible for manipulating time durations.
pub struct TimeManipulator;

impl TimeManipulator {
    /// Adds a duration to the given DateTime<Utc>.
    pub fn add_duration(datetime: DateTime<Utc>, amount: i64, unit: DurationUnit) -> DateTime<Utc> {
        match unit {
            DurationUnit::Years => {
                let new_year = datetime.year() + amount as i32;
                datetime.with_year(new_year).unwrap_or(datetime)
            }
            DurationUnit::Months => {
                let total_months = datetime.month0() as i64 + amount;
                let new_year = datetime.year() + (total_months / 12) as i32;
                let new_month = (total_months % 12) as u32 + 1;
                datetime
                    .with_year(new_year)
                    .and_then(|dt| dt.with_month(new_month))
                    .unwrap_or(datetime)
            }
            DurationUnit::Weeks => datetime + Duration::weeks(amount),
            DurationUnit::Days => datetime + Duration::days(amount),
            DurationUnit::Hours => datetime + Duration::hours(amount),
            DurationUnit::Minutes => datetime + Duration::minutes(amount),
            DurationUnit::Seconds => datetime + Duration::seconds(amount),
            DurationUnit::Milliseconds => datetime + Duration::milliseconds(amount),
            DurationUnit::Microseconds => datetime + Duration::microseconds(amount),
            DurationUnit::Nanoseconds => datetime + Duration::nanoseconds(amount),
        }
    }

    /// Subtracts a duration from the given DateTime<Utc>.
    pub fn subtract_duration(
        datetime: DateTime<Utc>,
        amount: i64,
        unit: DurationUnit,
    ) -> DateTime<Utc> {
        Self::add_duration(datetime, -amount, unit)
    }

    /// Returns the difference between two DateTime<Utc> in the specified unit.
    pub fn diff(datetime1: DateTime<Utc>, datetime2: DateTime<Utc>, unit: DurationUnit) -> i64 {
        match unit {
            DurationUnit::Years => datetime1.year() as i64 - datetime2.year() as i64,
            DurationUnit::Months => {
                let years_diff = Self::diff(datetime1, datetime2, DurationUnit::Years);
                let months_diff = datetime1.month() as i64 - datetime2.month() as i64;
                years_diff * 12 + months_diff
            }
            DurationUnit::Weeks => {
                let days_diff = Self::diff(datetime1, datetime2, DurationUnit::Days);
                days_diff / 7
            }
            DurationUnit::Days => {
                let duration = datetime1 - datetime2;
                duration.num_days()
            }
            DurationUnit::Hours => {
                let duration = datetime1 - datetime2;
                duration.num_hours()
            }
            DurationUnit::Minutes => {
                let duration = datetime1 - datetime2;
                duration.num_minutes()
            }
            DurationUnit::Seconds => {
                let duration = datetime1 - datetime2;
                duration.num_seconds()
            }
            DurationUnit::Milliseconds => {
                let duration = datetime1 - datetime2;
                duration.num_milliseconds()
            }
            DurationUnit::Microseconds => {
                let duration = datetime1 - datetime2;
                duration.num_microseconds().unwrap_or(0)
            }
            DurationUnit::Nanoseconds => {
                let duration = datetime1 - datetime2;
                duration.num_nanoseconds().unwrap_or(0)
            }
        }
    }

    /// Checks if two DateTime<Utc> are the same in terms of the specified unit.
    pub fn is_same(datetime1: DateTime<Utc>, datetime2: DateTime<Utc>, unit: DurationUnit) -> bool {
        match unit {
            DurationUnit::Years => datetime1.year() == datetime2.year(),
            DurationUnit::Months => {
                datetime1.year() == datetime2.year() && datetime1.month() == datetime2.month()
            }
            DurationUnit::Weeks => datetime1.iso_week() == datetime2.iso_week(),
            DurationUnit::Days => datetime1.date() == datetime2.date(),
            DurationUnit::Hours => {
                datetime1.date() == datetime2.date() && datetime1.hour() == datetime2.hour()
            }
            DurationUnit::Minutes => {
                datetime1.date() == datetime2.date()
                    && datetime1.hour() == datetime2.hour()
                    && datetime1.minute() == datetime2.minute()
            }
            DurationUnit::Seconds => datetime1.timestamp() == datetime2.timestamp(),
            DurationUnit::Milliseconds => {
                datetime1.timestamp_millis() == datetime2.timestamp_millis()
            }
            DurationUnit::Microseconds => {
                datetime1.timestamp_micros() == datetime2.timestamp_micros()
            }
            DurationUnit::Nanoseconds => datetime1.timestamp_nanos() == datetime2.timestamp_nanos(),
        }
    }
}
