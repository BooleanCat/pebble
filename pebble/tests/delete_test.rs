use std::path::Path;
use std::process::Command;
use uuid::Uuid;

const PEBBLE: &'static str = "../target/release/pebble";

#[test]
fn delete() {
    let id = format!("{}", Uuid::new_v4().to_hyphenated());

    let status = Command::new(PEBBLE)
        .args(&[
            "create",
            &id,
            Path::new("tests")
                .join("fixtures")
                .join("bundle.json")
                .to_str()
                .unwrap(),
        ])
        .status()
        .unwrap();

    assert!(status.success());

    let status = Command::new(PEBBLE)
        .args(&["delete", &id])
        .status()
        .unwrap();

    assert!(status.success());
}

#[test]
fn delete_missing_container_id() {
    let output = Command::new(PEBBLE).arg("delete").output().unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: The following required arguments were not provided:"));
    assert!(output.contains("<container-id>"));
}

#[test]
fn delete_no_such_container() {
    let id = format!("{}", Uuid::new_v4().to_hyphenated());

    let output = Command::new(PEBBLE)
        .args(&["delete", &id])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains(&format!(r#"error: delete "{}": no such container"#, &id)));
}
