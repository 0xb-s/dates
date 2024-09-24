use date_rs::DateX;

fn main() {
    let date = DateX::parse("2023-09-26 15:30:00", "%Y-%m-%d %H:%M:%S").unwrap();

    let formatted_en = date.format_with_locale("%A, %d %B %Y", "en");
    println!("English: {}", formatted_en);

    let formatted_es = date.format_with_locale("%A, %d %B %Y", "es");
    println!("Spanish: {}", formatted_es);

    let formatted_fr = date.format_with_locale("%A, %d %B %Y", "fr");
    println!("French: {}", formatted_fr);
}
