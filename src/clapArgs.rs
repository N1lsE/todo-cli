pub mod clap_args {

    use clap::{command, Arg, ArgMatches, Command};

    pub fn setup_cli_args() -> ArgMatches {
        let arg_matches = command!()
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
                        .required(true),
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

        #[warn(clippy::needless_return)]
        return arg_matches;
    }
}
