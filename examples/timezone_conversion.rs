use date_rs::DateX;

fn main() {

    let date = DateX::now();
    println!("Current UTC Time: {}", date);


    let ny_time = date
        .format_in_timezone("%Y-%m-%d %H:%M:%S", "America/New_York")
        .unwrap();
    println!("Time in New York: {}", ny_time);

    // Format date in Tokyo time
    let tokyo_time = date
        .format_in_timezone("%Y-%m-%d %H:%M:%S", "Asia/Tokyo")
        .unwrap();
    println!("Time in Tokyo: {}", tokyo_time);

    // Get the timezone offset for New York
    let ny_offset = date.timezone_offset("America/New_York").unwrap();
    println!("New York Timezone Offset: {}", ny_offset);


    let timezones = DateX::available_timezones();
    println!("Available Time Zones (first 5): {:?}", timezones);
}
