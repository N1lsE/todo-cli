use core::panic;
use std::{
    env,
    path::{Path, PathBuf},
    process::exit,
};

mod clapargs;
use clapargs::clap_args::setup_cli_args;

mod databasehandler;
mod filehandler;

use databasehandler::database_handler::todo_database::DatabaseField;
use homedir::get_my_home;

mod brain;
use brain::handle_todo::{
    display_todo, handle_add, handle_config, handle_create, handle_delete, handle_finish,
};

fn main() {
    let result = setup_cli_args();

    // Determine the correct todo directory based on global flag
    let todo_dir = if *result.get_one::<bool>("global").unwrap() {
        get_my_home()
            .expect("Could not find home directory!")
            .unwrap()
    } else {
        get_todo_dir(get_current_working_dir().expect("Could not get current working directory!"))
            .expect("Could not find todo directory!")
    };

    // Match on subcommands and handle appropriately
    match result.subcommand() {
        Some(("create", create_args)) => {
            handle_create(create_args);
            exit(0);
        }
        Some(("add", add_args)) => {
            handle_add(add_args, todo_dir);
            exit(0);
        }
        Some(("list", _list_args)) => {
            display_todo(false, todo_dir);
            exit(0);
        }
        Some(("open", _open_args)) => {
            println!("open");
            // Additional handling for open can be added here
        }
        Some(("finish", finish_args)) => {
            handle_finish(finish_args, todo_dir);
            exit(0);
        }
        Some(("delete", delete_args)) => {
            handle_delete(delete_args, todo_dir);
            // Possibly exit or additional handling after delete
        }
        Some(("config", config_args)) => {
            handle_config(config_args);
            exit(0);
        }
        Some(("clear", clear_args)) => {
            println!("clear");
            // Additional handling for clear can be added here
        }
        _ => {
            println!("Please try `todo help` to see how to use this.");
        }
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
        println!("No todo directory was found. Please use the `create` command!");
        exit(0);
    } else if todo_path.exists() {
        Ok(current_dir)
    } else {
        match current_dir.parent() {
            None => panic!("Parent directory not found"),
            Some(parent_dir) => return get_todo_dir(parent_dir.to_path_buf()),
        }
    }
}
