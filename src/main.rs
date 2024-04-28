use clap::{App, AppSettings, ArgMatches};

// Edith has heard about a cool new task manager CLI app.
// She goes to check out its help message by running the app with the --help flag.
// $ task_manager --help





fn main() {
    let matches = App::new("Task Manager")
        .version("0.1.0")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new("add").about("Add a new task"))
        .subcommand(App::new("list").about("List all tasks"))
        .subcommand(App::new("complete").about("Mark a task as completed"))
        .subcommand(App::new("remove").about("Remove a task"))
        .get_matches();

    match matches.subcommand() {
        Some(("add", _)) => {
            // Handle the "add" subcommand
        }
        Some(("list", _)) => {
            // Handle the "list" subcommand
        }
        Some(("complete", _)) => {
            // Handle the "complete" subcommand
        }
        Some(("remove", _)) => {
            // Handle the "remove" subcommand
        }
        _ => unreachable!(),
    }
}

