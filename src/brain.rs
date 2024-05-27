pub mod handle_todo {

    use chrono::{self, Local};
    use core::panic;
    use std::{
        fs::{self, write, OpenOptions},
        io::{Read, Write},
        path::{Path, PathBuf},
        process::exit,
    };

    use crate::brain::parse_due_date::parse_due_date;
    use crate::brain::parse_due_time::parse_due_time;

    use edit_distance::edit_distance;

    use clap::ArgMatches;

    use crate::{
        filehandler::file_handler::{self, append_line},
        get_current_working_dir,
    };

    use crate::databasehandler::database_handler;

    pub fn handle_delete(delete_args: &ArgMatches, todo_dir: PathBuf) {
        if let Some(index_s) = delete_args.get_one::<String>("index") {
            let index = index_s
                .parse::<usize>()
                .expect("couldnt parse string to usize");

            //read file
            let mut todo_file_path = todo_dir.clone();
            todo_file_path.push(".todo.todo");
            let mut todo_file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(todo_file_path.clone())
                .expect("Couldn't open file");

            if is_delete_in_file(todo_dir.clone()) {
                let mut delete_file_path = todo_dir.clone();
                delete_file_path.push(".todo.deleted");
                // let mut finished_file = OpenOptions::new()
                //     .append(true)
                //     .open(finished_file_path)
                //     .expect("Couldn't open file");
                //get line of index
                let mut file_content = String::new();
                todo_file
                    .read_to_string(&mut file_content)
                    .expect("failed to read file content");
                let l: Vec<&str> = file_content.lines().collect();
                let line: &str = l.get(index).expect("index doesnt exists");

                append_line(&delete_file_path, line.to_string())
                    .expect("failed to append line to file");
            }
            //delete line
            file_handler::remove_line(&todo_file_path, index).unwrap();

            //paste line in delete_file

            return;
        }

        display_todo(true, todo_dir);
    }

    // Again just GPT4 because why not
    pub fn is_delete_in_file(todo_dir: PathBuf) -> bool {
        let mut config_file_path = todo_dir;
        config_file_path.push(".todo.config"); // The specific configuration file

        // Open the file for reading and expect it to succeed
        let mut file = OpenOptions::new()
            .read(true)
            .open(&config_file_path)
            .expect("Failed to open the configuration file");

        // Read the whole file into a string
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read the file contents");

        // Check if the string "in_file" exists in the contents
        // contents.lines().any(|line| line.trim() == "in_file")
        contents
            .lines()
            .nth(1) // nth(1) for the second line, as nth is zero-indexed
            .map_or(false, |line| line.trim() == "in_file")
    }

    pub fn handle_finish(finish_args: &ArgMatches, todo_dir: PathBuf) {
        if let Some(index_s) = finish_args.get_one::<String>("index") {
            let index = index_s
                .parse::<usize>()
                .expect("couldnt parse string to usize");

            //read file
            let mut todo_file_path = todo_dir.clone();
            todo_file_path.push(".todo.todo");
            let mut todo_file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(todo_file_path.clone())
                .expect("Couldn't open file");

            let mut finished_file_path = todo_dir.clone();
            finished_file_path.push(".todo.finished");
            // let mut finished_file = OpenOptions::new()
            //     .append(true)
            //     .open(finished_file_path)
            //     .expect("Couldn't open file");

            //get line of index
            let mut file_content = String::new();
            todo_file
                .read_to_string(&mut file_content)
                .expect("failed to read file content");
            let l: Vec<&str> = file_content.lines().collect();
            let line: &str = l.get(index).expect("index doesnt exists");
            //delete line
            file_handler::remove_line(&todo_file_path, index).unwrap();

            //paste line in delete_file
            append_line(&finished_file_path, line.to_string())
                .expect("failed to append line to file");

            return;
        }

        display_todo(true, todo_dir);
    }

    pub fn handle_config(config_args: &ArgMatches) {
        let current_dir = get_current_working_dir().unwrap();
        let mut config_path = current_dir.clone();
        config_path.push(".todo.config");

        if !config_path.exists() {
            println!("config doesn't exist in: {}", config_path.to_string_lossy());
            return;
        }

        if let Some(name_args) = config_args.subcommand_matches("name") {
            let name_value = name_args.get_one::<String>("new_name").unwrap();

            let mut content = String::new();
            if let Ok(mut file) = OpenOptions::new().read(true).open(config_path.clone()) {
                file.read_to_string(&mut content).unwrap();
            }

            let mut content_lines: Vec<&str> = content.lines().collect();
            if content_lines.len() != 2 {
                println!("config file is broken!");
                exit(0);
            }
            content_lines.remove(0);
            content_lines.insert(0, name_value);
            // println!("{:?}", content_lines);

            write(
                config_path,
                format!(
                    "{}\n{}",
                    content_lines.first().unwrap(),
                    content_lines.get(1).unwrap()
                )
                .as_bytes(),
            )
            .unwrap();
            return;
        }

        if let Some(deleted_args) = config_args.subcommand_matches("deleted") {
            let deleted_value = deleted_args.get_one::<String>("delete_method").unwrap();

            let mut content = String::new();
            if let Ok(mut file) = OpenOptions::new().read(true).open(config_path.clone()) {
                file.read_to_string(&mut content).unwrap();
            }

            let mut content_lines: Vec<&str> = content.lines().collect();
            if content_lines.len() != 2 {
                println!("config file is broken!");
                exit(0);
            }

            content_lines.remove(1);
            let deleted_parsed_value = parse_deleted_value(deleted_value);
            content_lines.insert(1, &deleted_parsed_value);

            write(
                config_path,
                format!(
                    "{}\n{}",
                    content_lines.first().unwrap(),
                    content_lines.get(1).unwrap()
                )
                .as_bytes(),
            )
            .unwrap();
            return;
        }

        let mut file = OpenOptions::new().read(true).open(config_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        println!("current config:");
        for line in content.lines() {
            println!("\t{}", line);
        }

        #[warn(clippy::needless_return)]
        return;
    }

    pub fn parse_deleted_value(deleted_value: &str) -> String {
        let in_file_distance = edit_distance(deleted_value, "in_file");
        let delete_distance = edit_distance(deleted_value, "delete");

        if in_file_distance < delete_distance {
            return "in_file".to_string();
        }
        "delete".to_string()
    }

    pub fn handle_create(create_args: &ArgMatches) {
        let current_dir = get_current_working_dir().unwrap();
        let config = match create_args.get_one::<String>("config_file") {
            Some(file) => {
                let path = Path::new(file);
                let mut f = OpenOptions::new().read(true).open(path).unwrap();
                let mut config_str = String::new();
                f.read_to_string(&mut config_str).unwrap();
                config_str
            }
            None => {
                let mut config_str = String::new();
                config_str.push_str("default-name\n");
                config_str.push_str("in_file\n");
                config_str
            }
        };
        // check if file exists
        let mut config_path = current_dir.clone();
        config_path.push(".todo.config");
        if config_path.exists() {
            println!("config already exists!");
        } else {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(config_path)
                .unwrap();
            file.write_all(config.as_bytes()).unwrap();
            println!(".todo.config created");
        }

        let mut todo_path = current_dir.clone();
        todo_path.push(".todo.todo");
        if !todo_path.exists() {
            println!("{}", todo_path.clone().to_string_lossy());
            fs::write(todo_path, "").unwrap();
            println!(".todo.todo created");
        }

        let mut finished_path = current_dir.clone();
        finished_path.push(".todo.finished");
        if !finished_path.exists() {
            println!("{}", finished_path.clone().to_string_lossy());
            fs::write(finished_path, "").unwrap();
            println!(".todo.finished created");
        }

        let mut deleted_path = current_dir.clone();
        deleted_path.push(".todo.deleted");
        if !deleted_path.exists() {
            println!("{}", deleted_path.clone().to_string_lossy());
            fs::write(deleted_path, "").unwrap();
            println!(".todo.deleted created");
        }
    }

    pub fn display_todo(with_index: bool, todo_dir: PathBuf) {
        let mut todo_file_path = todo_dir.clone();
        todo_file_path.push(".todo.todo");
        let mut file = match OpenOptions::new()
            .read(true)
            .write(false)
            .append(false)
            .create(false)
            .open(&todo_file_path)
        {
            Ok(file) => file,
            Err(e) => panic!("failed to open file! {}", e),
        };

        let mut string_file = String::new();
        file.read_to_string(&mut string_file).unwrap();
        let mut index: usize = 0;
        for line in string_file.lines() {
            let mut display_line = String::new();
            if with_index {
                display_line.push_str(&format!("[{}]\t", index));
                index += 1;
            }
            let values: Vec<&str> = line.split('\t').collect();

            // get_name(),
            // description,
            // due_date,
            // due_time,
            // date,
            // time
            let name = values.get(0).unwrap_or(&"");
            let description = values.get(1).unwrap_or(&"");
            let due_date = values.get(2).unwrap_or(&"");
            let due_time = values.get(3).unwrap_or(&"");
            let date = values.get(4).unwrap_or(&"");
            let time = values.get(5).unwrap_or(&"");
            display_line.push_str(&format!("{}\t", description));
            display_line.push_str(&format!("{}\t", due_date));
            println!("{}", display_line);
        }
        // println!("{}", string_file);
    }

    pub fn handle_add(add_args: &ArgMatches, todo_dir: PathBuf) {
        let description = match add_args.get_one::<String>("description") {
            None => todo!("no description given!"),
            Some(s) => s.clone(),
        };
        let due_date = match add_args.get_one::<String>("due_date") {
            None => todo!("no due date given!"),
            Some(s) => parse_due_date(s.clone()),
        };
        let due_time = match add_args.get_one::<String>("due_time") {
            None => todo!("no due time given!"),
            Some(s) => parse_due_time(s.clone()),
        };
        let now = Local::now();
        let date = now.format("%H:%M").to_string();
        let time = now.format("%y-%m-%d").to_string();
        let todo_to_add: String = format!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            get_name(todo_dir.clone()),
            description,
            due_date,
            due_time,
            date,
            time
        );

        let mut todo_file_path = todo_dir.clone();
        todo_file_path.push(".todo.todo");
        append_line(&todo_file_path, todo_to_add).expect("failed to append line to file");
    }

    pub fn get_name(todo_dir: PathBuf) -> String {
        let mut config_path = todo_dir.clone();
        config_path.push(".todo.config");
        if !config_path.exists() {
            println!("config doesn't exist. couldn't get name. please use the `create` command");
            return "default-name".to_string();
        }
        let mut file = OpenOptions::new().read(true).open(config_path).unwrap();
        let mut name = String::new();
        file.read_to_string(&mut name).unwrap();
        let line = name.lines().next().unwrap_or("no-name-given");

        line.to_string()
    }
}

mod parse_due_date {
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

mod parse_due_time {
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
        use chrono::{Duration, Local};

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
