use std::process::Command;
use std::str;

#[test]
fn test_help_message() {
    // Run the CLI with '--help' to get the help message
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--help")
        .output()
        .expect("Failed to execute 'cargo run -- --help'");

    // Check that the command executed successfully
    assert!(output.status.success(), "Command did not run successfully");

    // Convert the output to a UTF-8 encoded string
    let stdout = String::from_utf8(output.stdout)
        .expect("Output was not valid UTF-8");

    // Assertions to check if the help message contains expected text
    assert!(stdout.contains("Usage: task_manager [SUBCOMMAND]"), "Help message does not contain expected 'Usage' text");
    assert!(stdout.contains("Subcommands:"), "Help message does not list subcommands");
    assert!(stdout.contains("add"), "Help message does not mention 'add' subcommand");
    assert!(stdout.contains("list"), "Help message does not mention 'list' subcommand");
    assert!(stdout.contains("complete"), "Help message does not mention 'complete' subcommand");
    assert!(stdout.contains("remove"), "Help message does not mention 'remove' subcommand");
}