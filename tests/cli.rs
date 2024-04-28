use std::process::Command;



#[test]
fn test_help_message() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("USAGE:\n    task_manager <SUBCOMMAND>"));
    assert!(stdout.contains("add"));
    assert!(stdout.contains("list"));
    assert!(stdout.contains("complete"));
    assert!(stdout.contains("remove"));
}