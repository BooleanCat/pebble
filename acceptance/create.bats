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
