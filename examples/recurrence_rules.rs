use date_rs::recurrence::{Frequency, RecurrenceGenerator, RecurrenceRule};
use date_rs::DateX;

fn main() {
    // Start date for the recurrence
    let start_date = DateX::parse("2023-09-26", "%Y-%m-%d")
        .unwrap()
        .to_datetime();

    // Define a recurrence rule: Daily, every 2 days, for 5 occurrences
    let rule = RecurrenceRule {
        frequency: Frequency::Daily,
        interval: 2,
        count: Some(5),
        until: None,
    };

    // Create a recurrence generator
    let mut generator = RecurrenceGenerator::new(start_date, rule);

    // Generate and display the occurrences
    while let Some(occurrence) = generator.next() {
        println!("Next occurrence: {}", occurrence);
    }
}
