use std::process::Command;

#[test]
#[ignore]
fn delete_missing_container_id() {
    let output = Command::new("target/release/pebble")
        .args(&["delete"])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: The following required arguments were not provided:"));
    assert!(output.contains("<container-id>"));
}

#[test]
#[ignore]
fn delete_no_such_container() {
    let output = Command::new("target/release/pebble")
        .args(&["delete", "foo"])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: no such container"));
}
