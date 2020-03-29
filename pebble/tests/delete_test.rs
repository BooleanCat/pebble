use std::process::Command;

const PEBBLE: &'static str = "../target/release/pebble";

#[test]
fn delete_missing_container_id() {
    let output = Command::new(PEBBLE).args(&["delete"]).output().unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: The following required arguments were not provided:"));
    assert!(output.contains("<container-id>"));
}

#[test]
fn delete_no_such_container() {
    let output = Command::new(PEBBLE)
        .args(&["delete", "foo"])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains(r#"error: delete: no such container "foo""#));
}
