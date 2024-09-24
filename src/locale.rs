// src/locale.rs
use chrono::Timelike;
use chrono::{DateTime, Datelike, Utc};
use std::collections::HashMap;
pub struct LocaleManager {
    month_names: HashMap<String, Vec<&'static str>>,
    weekday_names: HashMap<String, Vec<&'static str>>,
}

impl LocaleManager {
    /// Creates a new LocaleManager with default locales.
    pub fn new() -> Self {
        let mut month_names = HashMap::new();
        let mut weekday_names = HashMap::new();

        month_names.insert(
            "en".to_string(),
            vec![
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ],
        );

        weekday_names.insert(
            "en".to_string(),
            vec![
                "Monday",
                "Tuesday",
                "Wednesday",
                "Thursday",
                "Friday",
                "Saturday",
                "Sunday",
            ],
        );

        // Example for Spanish
        month_names.insert(
            "es".to_string(),
            vec![
                "Enero",
                "Febrero",
                "Marzo",
                "Abril",
                "Mayo",
                "Junio",
                "Julio",
                "Agosto",
                "Septiembre",
                "Octubre",
                "Noviembre",
                "Diciembre",
            ],
        );

        weekday_names.insert(
            "es".to_string(),
            vec![
                "Lunes",
                "Martes",
                "Miércoles",
                "Jueves",
                "Viernes",
                "Sábado",
                "Domingo",
            ],
        );

        LocaleManager {
            month_names,
            weekday_names,
        }
    }

    /// Formats a datetime with the specified locale.
    pub fn format_with_locale(
        &self,
        datetime: &DateTime<Utc>,
        format_str: &str,
        locale_code: &str,
    ) -> String {
        let months = self
            .month_names
            .get(locale_code)
            .unwrap_or(&self.month_names["en"]);
        let weekdays = self
            .weekday_names
            .get(locale_code)
            .unwrap_or(&self.weekday_names["en"]);

        let mut output = format_str.to_string();
        output = output.replace("%B", months[(datetime.month0()) as usize]);
        output = output.replace(
            "%A",
            weekdays[(datetime.weekday().num_days_from_monday()) as usize],
        );
        output = output.replace("%Y", &datetime.year().to_string());
        output = output.replace("%m", &format!("{:02}", datetime.month()));
        output = output.replace("%d", &format!("{:02}", datetime.day()));
        output = output.replace("%H", &format!("{:02}", datetime.hour()));
        output = output.replace("%M", &format!("{:02}", datetime.minute()));
        output = output.replace("%S", &format!("{:02}", datetime.second()));

        output
    }
}
