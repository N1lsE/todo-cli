pub mod parseDueTime {
    use chrono::{self, Duration, Local, Timelike};
    use regex::Regex;

    pub fn parse_due_time(due_time_string: String) -> String {
        // Remove any non-numeric or non-colon characters at the end
        let re =
            Regex::new(r"^(?P<time>\d{1,4}(?::\d{2})?|in \d+ (hours?|minutes?|min)|in \d+)\D*$")
                .unwrap();
        let clean_input = re
            .captures(&due_time_string)
            .map_or("", |caps| caps.name("time").unwrap().as_str());

        let current_time = Local::now();

        if let Ok(hours) = clean_input.parse::<u32>() {
            if hours < 24 {
                // Handle case "12" -> "12:00"
                return format!("{:02}:00", hours);
            } else if hours < 2400 {
                // Handle case "1240" -> "12:40"
                return format!("{:02}:{:02}", hours / 100, hours % 100);
            }
        } else if clean_input.starts_with("in ") {
            let parts: Vec<&str> = clean_input.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(value) = parts[1].parse::<i64>() {
                    if parts.len() == 3 && parts[2].starts_with("hour") {
                        // Handle case "in 3 hours"
                        let new_time = current_time + Duration::hours(value);
                        return format!("{:02}:{:02}", new_time.hour(), new_time.minute());
                    } else if parts.len() == 3
                        && (parts[2].starts_with("min") || parts[2].starts_with("minute"))
                    {
                        // Handle case "in 30 min"
                        let new_time = current_time + Duration::minutes(value);
                        return format!("{:02}:{:02}", new_time.hour(), new_time.minute());
                    } else if parts.len() == 2 {
                        // Handle case "in 3"
                        let new_time = current_time + Duration::hours(value);
                        return format!("{:02}:{:02}", new_time.hour(), new_time.minute());
                    }
                }
            }
        } else if clean_input.contains(':') {
            // Handle case "14:39"
            return clean_input.to_string();
        }

        due_time_string
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use chrono::{Duration, Local, TimeZone};

        #[test]
        fn test_parse_due_time_numeric() {
            assert_eq!(parse_due_time("12".to_string()), "12:00");
            assert_eq!(parse_due_time("1240".to_string()), "12:40");
            assert_eq!(parse_due_time("14:39".to_string()), "14:39");
        }

        #[test]
        fn test_parse_due_time_relative_hours() {
            let current_time = Local::now();
            let three_hours_later = current_time + Duration::hours(3);
            let expected_three_hours_time = format!(
                "{:02}:{:02}",
                three_hours_later.hour(),
                three_hours_later.minute()
            );

            assert_eq!(
                parse_due_time("in 3 hours".to_string()),
                expected_three_hours_time
            );
            assert_eq!(
                parse_due_time("in 3".to_string()),
                expected_three_hours_time
            );
        }

        #[test]
        fn test_parse_due_time_relative_minutes() {
            let current_time = Local::now();
            let thirty_minutes_later = current_time + Duration::minutes(30);
            let expected_thirty_minutes_time = format!(
                "{:02}:{:02}",
                thirty_minutes_later.hour(),
                thirty_minutes_later.minute()
            );

            assert_eq!(
                parse_due_time("in 30 min".to_string()),
                expected_thirty_minutes_time
            );
            assert_eq!(
                parse_due_time("in 30 minutes".to_string()),
                expected_thirty_minutes_time
            );
        }

        #[test]
        fn test_parse_due_time_erroneous_inputs() {
            let current_time = Local::now();
            let three_hours_later = current_time + Duration::hours(3);
            let thirty_minutes_later = current_time + Duration::minutes(30);
            let expected_three_hours_time = format!(
                "{:02}:{:02}",
                three_hours_later.hour(),
                three_hours_later.minute()
            );
            let expected_thirty_minutes_time = format!(
                "{:02}:{:02}",
                thirty_minutes_later.hour(),
                thirty_minutes_later.minute()
            );

            assert_eq!(parse_due_time("12d".to_string()), "12:00");
            assert_eq!(parse_due_time("1240d".to_string()), "12:40");
            assert_eq!(
                parse_due_time("in 3 hoursz".to_string()),
                expected_three_hours_time
            );
            assert_eq!(
                parse_due_time("in 3d".to_string()),
                expected_three_hours_time
            );
            assert_eq!(
                parse_due_time("in 30 minz".to_string()),
                expected_thirty_minutes_time
            );
            assert_eq!(
                parse_due_time("in 30 minutesz".to_string()),
                expected_thirty_minutes_time
            );
        }

        #[test]
        fn test_parse_due_time_invalid_inputs() {
            assert_eq!(parse_due_time("invalid".to_string()), "invalid");
            assert_eq!(parse_due_time("".to_string()), "");
            assert_eq!(parse_due_time("in hours".to_string()), "in hours");
        }
    }
}
