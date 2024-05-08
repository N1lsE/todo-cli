use chrono::{self, Local};
use core::panic;
use std::{
    env,
    error::{self, Error},
    fmt::format,
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
    time,
};

use clap::{arg, command, Arg, Command};
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
                Command::new("home").arg(
                    Arg::new("home_path")
                        .aliases(["homepath", "home", "path"])
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

    let todo_dir = match result.get_one::<bool>("global").unwrap() {
        false => get_todo_dir(get_current_working_dir().expect("couldnt find PathBuff in Option"))
            .unwrap(),
        true => get_my_home()
            .expect("Couldnt find current working dir!")
            .expect("couldnt find PathBuff in Option"),
    };
    println!("Todo dir: {}", todo_dir.to_string_lossy());

    if let Some(add_args) = result.subcommand_matches("add") {
        let mut due_date = "-";
        let description = match add_args.get_one::<String>("description") {
            None => todo!("no description given!"),
            Some(s) => s.clone(),
        };
        let due_date = match add_args.get_one::<String>("due_date") {
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
            get_name(),
            description,
            due_date,
            due_time,
            date,
            time
        );

        let mut todo_file_path = todo_dir.clone();
        todo_file_path.push(".todo.todo");
        println!("{}", todo_file_path.clone().to_string_lossy());
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
    if let Some(list_args) = result.subcommand_matches("list") {
        println!("list");
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
    if let Some(create_args) = result.subcommand_matches("create") {
        println!("create");
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
        panic!("No todo dir. please create one.");
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

fn get_name() -> String {
    "NAME".to_string()
}
