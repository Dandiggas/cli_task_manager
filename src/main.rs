// Edith has heard about a cool new task manager CLI app.
// She goes to check out its help message by running the app with the --help flag.
// $ task_manager --help

// She notices the help message mentions the available subcommands for managing tasks.
// The help message should include subcommands like:
// - add: Add a new task
// - list: List all tasks
// - complete: Mark a task as completed
// - remove: Remove a task

// She is invited to add a new task using the `add` subcommand.
// She types "Buy peacock feathers" as the task title.
// (Edith's hobby is tying fly-fishing lures)
// $ task_manager add "Buy peacock feathers"

// When she hits enter, the app confirms that the task has been added.
// The app should print a success message like:
// "Task added: 1. Buy peacock feathers"

// She can view the list of tasks using the `list` subcommand.
// $ task_manager list

// The app should display the list of tasks, including the newly added task:
// 1. [ ] Buy peacock feathers

// She adds another task using the `add` subcommand.
// She enters "Use peacock feathers to make a fly" as the task title.
// (Edith is very methodical)
// $ task_manager add "Use peacock feathers to make a fly"

// The app confirms that the second task has been added.
// "Task added: 2. Use peacock feathers to make a fly"

// She lists the tasks again using the `list` subcommand.
// $ task_manager list

// The app should now display both tasks:
// 1. [ ] Buy peacock feathers
// 2. [ ] Use peacock feathers to make a fly

// Satisfied, she exits the app.

fn main(){
    println!("hello World")
}