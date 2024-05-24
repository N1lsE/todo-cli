use chrono::{self, Local};
use core::panic;
use std::{
    env,
    fs::{self, write, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::exit,
};

mod clapargs;
use clapargs::clap_args::setup_cli_args;

mod filehandler;

mod parseduedate;
use parseduedate::parseDueDate;

mod parseduetime;
use parseduetime::parseDueTime;

use edit_distance::edit_distance;
use filehandler::file_handler::append_line;
use homedir::get_my_home;

use clap::ArgMatches;

use crate::filehandler::file_handler;

fn main() {
    let result = setup_cli_args();

    // println!("{}", result.get_one::<bool>("global").unwrap());

    if let Some(create_args) = result.subcommand_matches("create") {
        handle_create(create_args);
        exit(0);
    }

    let todo_dir = match result.get_one::<bool>("global").unwrap() {
        false => get_todo_dir(get_current_working_dir().expect("couldnt find PathBuff in Option"))
            .unwrap(),
        true => get_my_home()
            .expect("Couldnt find current working dir!")
            .expect("couldnt find PathBuff in Option"),
    };
    // println!("Todo dir: {}", todo_dir.to_string_lossy());

    if let Some(add_args) = result.subcommand_matches("add") {
        handle_add(add_args, todo_dir);
        exit(0);
    }
    if let Some(_list_args) = result.subcommand_matches("list") {
        display_todo(false, todo_dir);
        exit(0);
    }
    if let Some(open_args) = result.subcommand_matches("open") {
        println!("open");
    }
    if let Some(finish_args) = result.subcommand_matches("finish") {
        handle_finish(finish_args, todo_dir);
        exit(0);
    }
    if let Some(delete_args) = result.subcommand_matches("delete") {
        handle_delete(delete_args, todo_dir);
    }
    if let Some(config_args) = result.subcommand_matches("config") {
        handle_config(config_args);
        exit(0);
    }
    if let Some(clear_args) = result.subcommand_matches("clear") {
        println!("clear");
    }
    println!("Please try `todo help` to see how to use this.");
}

fn handle_delete(delete_args: &ArgMatches, todo_dir: PathBuf) {
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
fn is_delete_in_file(todo_dir: PathBuf) -> bool {
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

fn handle_finish(finish_args: &ArgMatches, todo_dir: PathBuf) {
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
        append_line(&finished_file_path, line.to_string()).expect("failed to append line to file");

        return;
    }

    display_todo(true, todo_dir);
}

fn handle_config(config_args: &ArgMatches) {
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

fn parse_deleted_value(deleted_value: &str) -> String {
    let in_file_distance = edit_distance(deleted_value, "in_file");
    let delete_distance = edit_distance(deleted_value, "delete");

    if in_file_distance < delete_distance {
        return "in_file".to_string();
    }
    return "delete".to_string();
}

fn handle_create(create_args: &ArgMatches) {
    let current_dir = get_current_working_dir().unwrap();
    let config = match create_args.get_one::<String>("config_file") {
        Some(file) => {
            let path = Path::new(file);
            let mut f = OpenOptions::new().read(true).open(path).unwrap();
            let mut config_str = String::new();
            f.read_to_string(&mut config_str);
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

fn display_todo(with_index: bool, todo_dir: PathBuf) {
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
    file.read_to_string(&mut string_file);
    let mut index: usize = 0;
    for line in string_file.lines().into_iter() {
        let mut display_line = String::new();
        if with_index {
            display_line.push_str(&format!("[{}]\t", index.to_string()));
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

fn handle_add(add_args: &ArgMatches, todo_dir: PathBuf) {
    let description = match add_args.get_one::<String>("description") {
        None => todo!("no description given!"),
        Some(s) => s.clone(),
    };
    let due_date = match add_args.get_one::<String>("due_date") {
        None => todo!("no due date given!"),
        Some(s) => parseDueDate::parse_due_date(s.clone()),
    };
    let due_time = match add_args.get_one::<String>("due_time") {
        None => todo!("no due time given!"),
        Some(s) => parseDueTime::parse_due_time(s.clone()),
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

fn get_current_working_dir() -> Result<PathBuf, String> {
    let res = env::current_dir();
    match res {
        Ok(path) => Ok(path),
        Err(_) => Err("Couldnt find current working dir!".to_string()),
    }
}

fn get_todo_dir(current_dir: PathBuf) -> Result<PathBuf, String> {
    let mut todo_path = current_dir.clone();
    todo_path.push(".todo.todo");

    if current_dir.clone() == Path::new("/") {
        println!("No todo dir. please use the `create` command!\nthe todo wasn't added!");
        exit(0);
        Err("No todo dir. please create one.".to_string())
    } else if todo_path.exists() {
        Ok(current_dir)
    } else {
        match current_dir.parent() {
            None => panic!("Parent directory not found"),
            Some(parent_dir) => return get_todo_dir(parent_dir.to_path_buf()),
        }
        panic!("Parent directory not found");
        Err("Parent directory not found".to_string())
    }
}

fn get_name(todo_dir: PathBuf) -> String {
    let mut config_path = todo_dir.clone();
    config_path.push(".todo.config");
    if !config_path.exists() {
        println!("config doesn't exist. couldn't get name. please use the `create` command");
        return "default-name".to_string();
    }
    let mut file = OpenOptions::new().read(true).open(config_path).unwrap();
    let mut name = String::new();
    file.read_to_string(&mut name);
    let line = name.lines().next().unwrap_or("no-name-given");

    line.to_string()
}
