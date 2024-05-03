use clap::{App, AppSettings, Arg, ArgMatches};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::OpenOptions;
use std::fs::read_to_string;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct Task {
    id: i32,
    task: String,
    completed: bool,
}

fn main() {
    let matches = App::new("Task Manager")
        .version("0.1.0")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("add")
                .about("Add a new task")
                .arg(Arg::new("TASK").required(true)),
        )
        .subcommand(App::new("list").about("List all tasks"))
        .subcommand(App::new("complete").about("Mark a task as completed"))
        .subcommand(App::new("remove").about("Remove a task"))
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let task_title = sub_matches.value_of("TASK").unwrap();
            let _ = print_a_task(task_title); 
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



pub fn print_a_task(task_title: &str) -> Result<String> {

    let task = Task {
        id: 1,
        task: task_title.to_owned(),
        completed: false,
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string_pretty(&task)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    Ok(j)
}
// pub fn write_to_file(j: String) {
//     // check is the file exists if not create
//     let file = OpenOptions::new()
//         .write(true)
//         .create_new(true)
//         .open("task.json");

//     if let Ok(mut file) = file {
//         if let Err(e) = file.write_all(j.as_bytes()) {
//             eprintln!("Couldn't write to file: {}", e);
//         } else {
//             println!("Successfully wrote to the file.")
//         }
//     } else {
//         eprintln!("File already exists");
//     }
// }

// fn deserialize_and_print() -> Result<()> {
//     // Some JSON input data as a &str. Maybe this comes from the user.
//     let data = read_to_string("task.json").unwrap();

//     // Parse the string of data into a Person object. This is exactly the
//     // same function as the one that produced serde_json::Value above, but
//     // now we are asking it for a Person as output.
//     let t: Task = serde_json::from_str(&data).unwrap();

//     // Do things just like with any other Rust data structure.
//     println!("Task ID: {} Task: {}", t.id, t.task);

//     Ok(())
// }

// fn main()-> Result<()>{
//     let j = print_a_task()?;

//     write_to_file(j);
//     deserialize_and_print()?;
//  Ok(())
// }