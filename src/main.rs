use chrono::{self, Local};
use core::panic;
use std::{
    env::{self, current_dir},
    error::{self, Error},
    fmt::format,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::exit,
    str::FromStr,
    time,
};

use clap::{arg, command, Arg, ArgMatches, Command};
use homedir::get_my_home;

fn main() {
    let result = command!()
        .about("This CLI-Tool is to manage Todos. It can be used for global todos in your home dir or in your current working dir.")
        .arg(Arg::new("global").short('g').long("global").required(false).num_args(0).help("use todo in home folder"))
        .subcommands([
            Command::new("create").arg(
                Arg::new("config_file")
                    .short('c')
                    .long("config")
                    .alias("configfile")
                    .num_args(1)
                    .required(false)
                    .help("creates a copy of the given config_file to use in this todo."),
            ),
            Command::new("clear").alias("c"),
            Command::new("open").alias("o"),
            Command::new("config").alias("conf").subcommands([
                Command::new("name").arg(
                    Arg::new("new_name")
                        .aliases(["newname", "name"])
                        .help("this name is written beside the todo to track it.")
                        .required(false),
                ),
                Command::new("deleted")
                    .alias("delete")
                    .arg(Arg::new("delete_methode").help("deletemethode: in_file/delete\nin_file saves deleted todos in .todo.deleted\ndelete doesn't save deleted todos").required(false)),
            ]),
            Command::new("delete").arg(Arg::new("index").required(false)),
            Command::new("add").args([
                Arg::new("description").required(true),
                Arg::new("due_date")
                    .short('d')
                    .long("due-date")
                    .aliases(["duedate"])
                    .required(false),
                Arg::new("due_time")
                    .short('t')
                    .long("due-time")
                    .aliases(["duetime"])
                    .required(false),
            ]),
            Command::new("list").alias("ls"),
            Command::new("finish").arg(Arg::new("index")),
        ])
        .get_matches();

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
    if let Some(list_args) = result.subcommand_matches("list") {
        display_todo(false, todo_dir);
        exit(0);
    }
    if let Some(open_args) = result.subcommand_matches("open") {
        println!("open");
    }
    if let Some(finish_args) = result.subcommand_matches("finish") {
        println!("finish");
    }
    if let Some(delete_args) = result.subcommand_matches("delete") {
        println!("delete");
    }
    if let Some(config_args) = result.subcommand_matches("config") {
        println!("config");
    }
    if let Some(clear_args) = result.subcommand_matches("clear") {
        println!("clear");
    }
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
        file.write(config.as_bytes());
        println!(".todo.config created");
    }

    let mut todo_path = current_dir.clone();
    todo_path.push(".todo.todo");
    if !todo_path.exists() {
        println!("{}", todo_path.clone().to_string_lossy());
        fs::write(todo_path, "");
        println!(".todo.todo created");
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
    let mut due_date = "-";
    let description = match add_args.get_one::<String>("description") {
        None => todo!("no description given!"),
        Some(s) => s.clone(),
    };
    let due_date = match add_args.get_one::<String>("due_date") {
        // TODO: handle different cases:
        // in 2 days
        // next week
        // tomorow
        // today
        // 12.04.2024
        // 2.4.25
        // 02.3         -> next possible day 02.03
        // 1.07.2024
        // week 21      -> next possible week 21
        None => todo!("no due date given!"),
        Some(s) => s.clone(),
    };
    let due_time = match add_args.get_one::<String>("due_time") {
        None => todo!("no due time given!"),
        Some(s) => s.clone(),
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
    // println!("{}", todo_file_path.clone().to_string_lossy());
    let mut file = match OpenOptions::new()
        .read(false)
        .write(false)
        .append(true)
        .create(false)
        .open(&todo_file_path)
    {
        Ok(file) => file,
        Err(e) => panic!("failed to open file! {}", e),
    };

    if let Err(e) = writeln!(file, "{}", todo_to_add) {
        eprintln!("Coldn't write to file: {}", e);
    }
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
