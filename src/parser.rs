// src/parser.rs
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use regex::Regex;
use std::collections::HashMap;

/// A struct responsible for parsing date strings into `DateX` instances.
pub struct DateParser;

impl DateParser {
    /// Parses a date string according to the provided format string.
    pub fn parse(date_str: &str, format: &str) -> Result<DateTime<Utc>, String> {
        let regex_str = Self::format_to_regex(format);
        let re = Regex::new(&regex_str).map_err(|e| e.to_string())?;
        let caps = re
            .captures(date_str)
            .ok_or("Date string does not match format")?;

        let mut date_parts: HashMap<&str, u32> = HashMap::new();
        let mut year: i32 = 1970;
        let mut hour_12 = false;
        let mut is_pm = false;

        for name in re.capture_names().flatten() {
            if let Some(value) = caps.name(name) {
                match name {
                    "Y" => {
                        year = value.as_str().parse::<i32>().map_err(|_| "Invalid year")?;
                    }
                    "y" => {
                        let y = value.as_str().parse::<i32>().map_err(|_| "Invalid year")?;
                        year = if y < 70 { 2000 + y } else { 1900 + y };
                    }
                    "m" => {
                        date_parts.insert(
                            "month",
                            value.as_str().parse::<u32>().map_err(|_| "Invalid month")?,
                        );
                    }
                    "d" => {
                        date_parts.insert(
                            "day",
                            value.as_str().parse::<u32>().map_err(|_| "Invalid day")?,
                        );
                    }
                    "H" | "I" => {
                        date_parts.insert(
                            "hour",
                            value.as_str().parse::<u32>().map_err(|_| "Invalid hour")?,
                        );
                        if name == "I" {
                            hour_12 = true;
                        }
                    }
                    "M" => {
                        date_parts.insert(
                            "minute",
                            value
                                .as_str()
                                .parse::<u32>()
                                .map_err(|_| "Invalid minute")?,
                        );
                    }
                    "S" => {
                        date_parts.insert(
                            "second",
                            value
                                .as_str()
                                .parse::<u32>()
                                .map_err(|_| "Invalid second")?,
                        );
                    }
                    "p" => {
                        is_pm = value.as_str().to_lowercase() == "pm";
                    }
                    // ... Additional tokens can be added here
                    _ => {}
                }
            }
        }

        let month = *date_parts.get("month").unwrap_or(&1);
        let day = *date_parts.get("day").unwrap_or(&1);
        let mut hour = *date_parts.get("hour").unwrap_or(&0);
        let minute = *date_parts.get("minute").unwrap_or(&0);
        let second = *date_parts.get("second").unwrap_or(&0);

        if hour_12 {
            if is_pm && hour != 12 {
                hour += 12;
            } else if !is_pm && hour == 12 {
                hour = 0;
            }
        }

        let naive_date = NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid date")?;
        let naive_time = NaiveTime::from_hms_opt(hour, minute, second).ok_or("Invalid time")?;
        let naive_datetime = NaiveDateTime::new(naive_date, naive_time);

        Ok(DateTime::<Utc>::from_utc(naive_datetime, Utc))
    }

    fn format_to_regex(format: &str) -> String {
        let mut regex_str = String::new();
        let mut chars = format.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '%' {
                if let Some(next_char) = chars.next() {
                    match next_char {
                        'Y' => regex_str.push_str(r"(?P<Y>\d{4})"),
                        'y' => regex_str.push_str(r"(?P<y>\d{2})"),
                        'm' => regex_str.push_str(r"(?P<m>\d{1,2})"),
                        'd' => regex_str.push_str(r"(?P<d>\d{1,2})"),
                        'H' => regex_str.push_str(r"(?P<H>\d{1,2})"),
                        'I' => regex_str.push_str(r"(?P<I>\d{1,2})"),
                        'M' => regex_str.push_str(r"(?P<M>\d{1,2})"),
                        'S' => regex_str.push_str(r"(?P<S>\d{1,2})"),
                        'p' => regex_str.push_str(r"(?P<p>AM|PM|am|pm)"),
                        '%' => regex_str.push('%'),
                        // ... Additional tokens can be added here
                        _ => regex_str.push(next_char),
                    }
                }
            } else {
                if "()[]{}.*+?^$|\\".contains(c) {
                    regex_str.push('\\');
                }
                regex_str.push(c);
            }
        }

        format!("^{}$", regex_str)
    }
}
