use std::process::Command;

const PEBBLE: &'static str = "../target/release/pebble";

#[test]
fn state_missing_container_id() {
    let output = Command::new(PEBBLE).args(&["state"]).output().unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: The following required arguments were not provided:"));
    assert!(output.contains("<container-id>"));
}

#[test]
fn state_no_such_container() {
    let output = Command::new(PEBBLE)
        .args(&["state", "foo"])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: no such container"));
}
