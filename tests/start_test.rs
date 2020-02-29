use std::process::Command;

#[test]
#[ignore]
fn start_missing_container_id() {
    let output = Command::new("target/release/pebble")
        .args(&["start"])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: The following required arguments were not provided:"));
    assert!(output.contains("<container-id>"));
}

#[test]
#[ignore]
fn start_no_such_container() {
    let output = Command::new("target/release/pebble")
        .args(&["start", "foo"])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: no such container"));
}
