use std::process::Command;

#[test]
#[ignore]
fn kill_missing_container_id() {
    let output = Command::new("target/release/pebble")
        .args(&["kill"])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: The following required arguments were not provided:"));
    assert!(output.contains("<container-id>"));
}

#[test]
#[ignore]
fn kill_unknown_signal() {
    let output = Command::new("target/release/pebble")
        .args(&["kill", "foo", "SIGFOO"])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: Invalid value for '<signal>'"));
}

#[test]
#[ignore]
fn kill_no_such_container() {
    let output = Command::new("target/release/pebble")
        .args(&["kill", "foo"])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: no such container"));
}