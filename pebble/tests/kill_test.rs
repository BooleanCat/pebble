use std::process::Command;

const PEBBLE: &'static str = "../target/release/pebble";

#[test]
fn kill_missing_container_id() {
    let output = Command::new(PEBBLE).args(&["kill"]).output().unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: The following required arguments were not provided:"));
    assert!(output.contains("<container-id>"));
}

#[test]
fn kill_unknown_signal() {
    let output = Command::new(PEBBLE)
        .args(&["kill", "foo", "SIGFOO"])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: Invalid value for '<signal>'"));
}

#[test]
fn kill_no_such_container() {
    let output = Command::new(PEBBLE)
        .args(&["kill", "foo"])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: no such container"));
}
