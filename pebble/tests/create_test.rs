use std::path::Path;
use std::process::Command;
use uuid::Uuid;

const PEBBLE: &'static str = "../target/release/pebble";

#[test]
fn create() {
    let id = format!("{}", Uuid::new_v4().to_hyphenated());

    assert!(Command::new(PEBBLE)
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
        .unwrap()
        .success());

    assert!(
        String::from_utf8(Command::new(PEBBLE).arg("list").output().unwrap().stdout)
            .unwrap()
            .contains(&id)
    );

    assert!(Command::new(PEBBLE)
        .args(&["delete", &id])
        .status()
        .unwrap()
        .success());
}

#[test]
fn create_missing_container_id() {
    let output = Command::new(PEBBLE).arg("create").output().unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: The following required arguments were not provided:"));
    assert!(output.contains("<container-id>"));
}

#[test]
fn create_missing_bundle_path() {
    let id = format!("{}", Uuid::new_v4().to_hyphenated());

    let output = Command::new(PEBBLE)
        .args(&["create", &id])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: The following required arguments were not provided:"));
    assert!(output.contains("<path-to-bundle>"));
}

#[test]
fn create_bundle_not_found() {
    let id = format!("{}", Uuid::new_v4().to_hyphenated());

    let output = Command::new(PEBBLE)
        .args(&["create", &id, "bar"])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains(r#"error: open "bar""#));
}

#[test]
fn create_bad_bundle() {
    let id = format!("{}", Uuid::new_v4().to_hyphenated());

    assert!(String::from_utf8(
        Command::new(PEBBLE)
            .args(&["create", &id, "tests/fixtures/bundle-junk.json"])
            .output()
            .unwrap()
            .stderr
    )
    .unwrap()
    .contains(r#"error: parse "tests/fixtures/bundle-junk.json""#));
}
