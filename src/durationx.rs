// src/durationx.rs
use chrono::Duration;
use std::fmt;

/// A struct representing a time duration with human-friendly parsing and formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurationX {
    duration: Duration,
}

impl DurationX {
    /// Creates a new DurationX from a duration string (e.g., "2 days", "3 hours").
    pub fn parse(duration_str: &str) -> Result<Self, String> {
        let parts: Vec<&str> = duration_str.trim().split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Invalid duration format".to_string());
        }

        let amount = parts[0].parse::<i64>().map_err(|_| "Invalid amount")?;
        let unit = parts[1];

        let duration = match unit {
            "years" | "year" => Duration::days(amount * 365),
            "months" | "month" => Duration::days(amount * 30),
            "weeks" | "week" => Duration::weeks(amount),
            "days" | "day" => Duration::days(amount),
            "hours" | "hour" => Duration::hours(amount),
            "minutes" | "minute" => Duration::minutes(amount),
            "seconds" | "second" => Duration::seconds(amount),
            _ => return Err("Invalid duration unit".to_string()),
        };

        Ok(DurationX { duration })
    }

    /// Returns the underlying Duration.
    pub fn duration(&self) -> Duration {
        self.duration
    }
}

impl fmt::Display for DurationX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.duration.num_seconds();
        if secs >= 86400 {
            let days = secs / 86400;
            write!(f, "{} days", days)
        } else if secs >= 3600 {
            let hours = secs / 3600;
            write!(f, "{} hours", hours)
        } else if secs >= 60 {
            let minutes = secs / 60;
            write!(f, "{} minutes", minutes)
        } else {
            write!(f, "{} seconds", secs)
        }
    }
}
