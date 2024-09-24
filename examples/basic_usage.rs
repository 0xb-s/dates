use date_rs::datex;
use date_rs::parse_datex;
use date_rs::{DateX, DurationUnit};
fn main() {
    let mut date = DateX::now();
    println!("Current Date and Time: {}", date);

    date.add_duration(1, DurationUnit::Days);
    println!("After adding one day: {}", date);

    let formatted = date.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted Date: {}", formatted);

    let parsed_date = DateX::parse("2024-02-29", "%Y-%m-%d").unwrap();
    println!("Parsed Date: {}", parsed_date);

    let is_leap = parsed_date.is_leap_year();
    println!("Is leap year: {}", is_leap);

    let week_of_year = date.week_of_year();
    println!("Week of the year: {}", week_of_year);

    let ordinal_suffix = date.ordinal_suffix();
    println!("Ordinal suffix: {}", ordinal_suffix);

    let days_in_month = date.days_in_month();
    println!("Days in month: {}", days_in_month);

    let time_ago = date.time_ago();
    println!("Time ago: {}", time_ago);

    let is_weekend = date.is_weekend();
    println!("Is weekend: {}", is_weekend);

    let timestamp = date.timestamp();
    println!("Timestamp: {}", timestamp);

    let macro_date = datex!(2023, 9, 24, 14, 30, 0);
    println!("Date created using macro: {}", macro_date);

    let parsed_macro_date = parse_datex!("2023-09-24 14:30:00", "%Y-%m-%d %H:%M:%S");
    println!("Date parsed using macro: {}", parsed_macro_date);
}
