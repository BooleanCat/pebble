@test "create missing container ID" {
  run pebble create

  [ "$status" -eq 1 ]

  local message="error: The following required arguments were not provided:"
  [[ "$output" == "$message"* ]]
  [[ "$output" == *"<container-id>"* ]]
}

@test "create missing bundle path" {
  run pebble create foo

  [ "$status" -eq 1 ]

  local message="error: The following required arguments were not provided:"
  [[ "$output" == "$message"* ]]
  [[ "$output" == *"<path-to-bundle>"* ]]
}

@test "create with bundle not existing" {
  run pebble create foo oops.json

  [ "$status" -eq 1 ]

  local message='error: open "oops.json"'
  echo "$output"
  [[ "$output" == "$message"* ]]
}

@test "create with non-parsable bundle" {
  run pebble create foo acceptance/fixtures/bundle-junk.json

  [ "$status" -eq 1 ]

  local message='error: parse "acceptance/fixtures/bundle-junk.json"'
  echo "$output"
  [[ "$output" == "$message"* ]]
}
