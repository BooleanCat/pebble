use std::process::Command;

const PEBBLE: &'static str = "../target/release/pebble";

#[test]
fn create_missing_container_id() {
    let output = Command::new(PEBBLE).args(&["create"]).output().unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: The following required arguments were not provided:"));
    assert!(output.contains("<container-id>"));
}

#[test]
fn create_missing_bundle_path() {
    let output = Command::new(PEBBLE)
        .args(&["create", "foo"])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains("error: The following required arguments were not provided:"));
    assert!(output.contains("<path-to-bundle>"));
}

#[test]
fn create_bundle_not_found() {
    let output = Command::new(PEBBLE)
        .args(&["create", "foo", "bar"])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains(r#"error: open "bar""#));
}

#[test]
fn create_bad_bundle() {
    let output = Command::new(PEBBLE)
        .args(&["create", "foo", "tests/fixtures/bundle-junk.json"])
        .output()
        .unwrap();

    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains(r#"error: parse "tests/fixtures/bundle-junk.json""#));
}
