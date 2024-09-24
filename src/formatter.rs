// src/formatter.rs
use crate::utils::get_ordinal_suffix;
use crate::DateX;
use chrono::{DateTime, Datelike, Timelike, Utc};

/// A struct responsible for formatting dates according to format strings.
pub struct DateFormatter;

impl DateFormatter {
    /// Formats the `DateX` instance according to the provided format string.
    pub fn format(datex: &DateX, format_str: &str) -> String {
        let datetime = datex.datetime;
        let mut result = String::new();
        let mut chars = format_str.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '%' {
                if let Some(next_char) = chars.next() {
                    result.push_str(&Self::format_token(&datetime, next_char, datex));
                }
            } else {
                result.push(c);
            }
        }

        result
    }

    fn format_token(datetime: &DateTime<Utc>, token: char, datex: &DateX) -> String {
        match token {
            'Y' => datetime.year().to_string(),
            'y' => format!("{:02}", datetime.year() % 100),
            'm' => format!("{:02}", datetime.month()),
            'b' => datetime.format("%b").to_string(),
            'B' => datetime.format("%B").to_string(),
            'd' => format!("{:02}", datetime.day()),
            'e' => format!("{}", datetime.day()),
            'H' => format!("{:02}", datetime.hour()),
            'I' => format!("{:02}", datetime.hour12().1),
            'p' => datetime.format("%p").to_string(),
            'M' => format!("{:02}", datetime.minute()),
            'S' => format!("{:02}", datetime.second()),
            'f' => format!("{:06}", datetime.timestamp_subsec_micros()),
            '3' => format!("{:03}", datetime.timestamp_subsec_millis()),
            'z' => datetime.format("%z").to_string(),
            'Z' => datetime.format("%Z").to_string(),
            'j' => format!("{:03}", datetime.ordinal()),
            'w' => datetime.format("%w").to_string(),
            'a' => datetime.format("%a").to_string(),
            'A' => datetime.format("%A").to_string(),
            'u' => datetime.weekday().number_from_monday().to_string(),
            'V' => format!("{:02}", datetime.iso_week().week()),
            'G' => datetime.iso_week().year().to_string(),
            'D' => format!("{}{}", datetime.day(), get_ordinal_suffix(datetime.day())),
            's' => datetime.timestamp().to_string(),
            'Q' => datex.quarter().to_string(),
            't' => "\t".to_string(),
            'n' => "\n".to_string(),
            '%' => "%".to_string(),
    
            _ => token.to_string(),
        }
    }
}

impl DateX {
    /// Returns the quarter of the year (1-4).
    pub fn quarter(&self) -> u32 {
        (self.month() - 1) / 3 + 1
    }
}
