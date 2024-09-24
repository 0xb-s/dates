// src/recurrence.rs
use chrono::{DateTime, Duration, Utc};
use std::collections::VecDeque;

/// Represents a recurrence rule for generating dates.
#[derive(Debug, Clone)]
pub struct RecurrenceRule {
    pub frequency: Frequency,
    pub interval: u32,
    pub count: Option<u32>,
    pub until: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub enum Frequency {
    Secondly,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

pub struct RecurrenceGenerator {
    rule: RecurrenceRule,
    start_date: DateTime<Utc>,
    occurrences: VecDeque<DateTime<Utc>>,
}

impl RecurrenceGenerator {
    /// Creates a new RecurrenceGenerator.
    pub fn new(start_date: DateTime<Utc>, rule: RecurrenceRule) -> Self {
        let mut generator = RecurrenceGenerator {
            rule,
            start_date,
            occurrences: VecDeque::new(),
        };
        generator.generate_occurrences();
        generator
    }

    /// Generates occurrences based on the rule.
    fn generate_occurrences(&mut self) {
        let mut current_date = self.start_date;
        let mut occurrences_generated = 0;

        loop {
            if let Some(count) = self.rule.count {
                if occurrences_generated >= count {
                    break;
                }
            }

            if let Some(until) = self.rule.until {
                if current_date > until {
                    break;
                }
            }

            self.occurrences.push_back(current_date);

            current_date = match self.rule.frequency {
                Frequency::Secondly => current_date + Duration::seconds(self.rule.interval as i64),
                Frequency::Minutely => current_date + Duration::minutes(self.rule.interval as i64),
                Frequency::Hourly => current_date + Duration::hours(self.rule.interval as i64),
                Frequency::Daily => current_date + Duration::days(self.rule.interval as i64),
                Frequency::Weekly => current_date + Duration::weeks(self.rule.interval as i64),
                Frequency::Monthly => {
                    // For simplicity, add 30 days per month
                    current_date + Duration::days(30 * self.rule.interval as i64)
                }
                Frequency::Yearly => {
                    // For simplicity, add 365 days per year
                    current_date + Duration::days(365 * self.rule.interval as i64)
                }
            };

            occurrences_generated += 1;
        }
    }

    /// Returns the next occurrence.
    pub fn next(&mut self) -> Option<DateTime<Utc>> {
        self.occurrences.pop_front()
    }
}
