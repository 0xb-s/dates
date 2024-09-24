// src/timezone.rs
use chrono::Offset;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;
use std::str::FromStr;
/// Provides functionalities for time zone conversions.
pub struct TimeZoneHandler;

impl TimeZoneHandler {
    /// Converts a DateTime<Utc> to the specified time zone.
    pub fn convert_to_timezone(
        datetime: DateTime<Utc>,
        timezone: &str,
    ) -> Result<DateTime<Tz>, String> {
        match Tz::from_str(timezone) {
            Ok(tz) => Ok(datetime.with_timezone(&tz)),
            Err(_) => Err("Invalid timezone".to_string()),
        }
    }

    /// Gets the offset from UTC for the specified time zone at the given datetime.
    pub fn get_timezone_offset(
        datetime: DateTime<Utc>,
        timezone: &str,
    ) -> Result<chrono::FixedOffset, String> {
        match Tz::from_str(timezone) {
            Ok(tz) => {
                let offset = tz.offset_from_utc_datetime(&datetime.naive_utc());
                Ok(offset.fix())
            }
            Err(_) => Err("Invalid timezone".to_string()),
        }
    }

    /// Lists all available time zones.
    pub fn list_timezones() -> Vec<&'static str> {
        chrono_tz::TZ_VARIANTS.iter().map(|tz| tz.name()).collect()
    }
}
