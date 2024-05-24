pub mod parseDueDate {
    use chrono::{self, Datelike, Duration, Local, NaiveDate};

    pub fn parse_due_date(due_date_string: String) -> String {
        let today = Local::now();
        let parts: Vec<&str> = due_date_string.split_whitespace().collect();

        match due_date_string.to_lowercase().as_str() {
            "today" => return today.format("%d.%m.%Y").to_string(),
            "tomorrow" => return (today + Duration::days(1)).format("%d.%m.%Y").to_string(),
            "next week" => return (today + Duration::days(7)).format("%d.%m.%Y").to_string(),
            _ => {}
        }

        if parts.len() == 3 && parts[0] == "in" {
            if let Ok(value) = parts[1].parse::<i64>() {
                if parts[2] == "days" {
                    return (today + Duration::days(value))
                        .format("%d.%m.%Y")
                        .to_string();
                } else if parts[2] == "weeks" {
                    return (today + Duration::weeks(value))
                        .format("%d.%m.%Y")
                        .to_string();
                }
            }
        }

        if parts.len() == 2 && parts[0] == "week" {
            if let Ok(week_number) = parts[1].parse::<u32>() {
                return parse_week_number(week_number);
            }
        }

        if let Ok(date) = NaiveDate::parse_from_str(&due_date_string, "%d.%m.%Y") {
            return date.format("%d.%m.%Y").to_string();
        }

        if let Ok(date) = NaiveDate::parse_from_str(&due_date_string, "%d.%m.%y") {
            return date.format("%d.%m.%Y").to_string();
        }

        if let Ok(date) = NaiveDate::parse_from_str(&due_date_string, "%d.%m") {
            let possible_date = find_next_possible_date(date.day(), date.month());
            return possible_date.format("%d.%m.%Y").to_string();
        }

        due_date_string
    }

    fn parse_week_number(week_number: u32) -> String {
        let today = Local::now();
        let year = today.year();
        let jan_1 = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        let jan_1_weekday = jan_1.weekday().number_from_monday();

        let first_week_start = if jan_1_weekday <= 4 {
            jan_1 - Duration::days(jan_1_weekday.into())
        } else {
            jan_1 + Duration::days(7 - jan_1_weekday as i64)
        };

        let week_start = first_week_start + Duration::weeks(week_number as i64 - 1);
        week_start.format("%d.%m.%Y").to_string()
    }

    fn find_next_possible_date(day: u32, month: u32) -> NaiveDate {
        let today = Local::today().naive_local();
        let year = today.year();
        let mut date = NaiveDate::from_ymd_opt(year, month, day)
            .unwrap_or_else(|| NaiveDate::from_ymd(year + 1, month, day));

        if date < today {
            date = NaiveDate::from_ymd(year + 1, month, day);
        }
        date
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_due_date_relative_days() {
            let current_date = Local::today().naive_local();
            let expected_date = (current_date + Duration::days(2))
                .format("%d.%m.%Y")
                .to_string();
            assert_eq!(parse_due_date("in 2 days".to_string()), expected_date);
        }

        #[test]
        fn test_parse_due_date_named_dates() {
            let current_date = Local::today().naive_local();
            let expected_tomorrow = (current_date + Duration::days(1))
                .format("%d.%m.%Y")
                .to_string();
            let expected_today = current_date.format("%d.%m.%Y").to_string();
            let expected_next_week = (current_date + Duration::days(7))
                .format("%d.%m.%Y")
                .to_string();
            assert_eq!(parse_due_date("today".to_string()), expected_today);
            assert_eq!(parse_due_date("tomorrow".to_string()), expected_tomorrow);
            assert_eq!(parse_due_date("next week".to_string()), expected_next_week);
        }

        #[test]
        fn test_parse_due_date_specific_dates() {
            assert_eq!(parse_due_date("12.04.2024".to_string()), "12.04.2024");
            assert_eq!(parse_due_date("2.4.25".to_string()), "02.04.2025");
            assert_eq!(parse_due_date("1.07.2024".to_string()), "01.07.2024");
        }

        #[test]
        fn test_parse_due_date_next_possible_day() {
            let current_date = Local::today().naive_local();
            let expected_date = find_next_possible_date(2, 3).format("%d.%m.%Y").to_string();
            assert_eq!(parse_due_date("02.3".to_string()), expected_date);
        }

        #[test]
        fn test_parse_due_date_week_number() {
            assert_eq!(parse_due_date("week 21".to_string()), parse_week_number(21));
        }

        #[test]
        fn test_parse_due_date_invalid_inputs() {
            assert_eq!(parse_due_date("invalid".to_string()), "invalid");
            assert_eq!(parse_due_date("".to_string()), "");
            assert_eq!(
                parse_due_date("some random text".to_string()),
                "some random text"
            );
        }
    }
}
