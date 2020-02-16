@test "state for a non-existent container" {
  run pebble state foo
  [ "$status" -eq 1 ]
  [ "$output" = 'error: no such container' ]
}

@test "state missing container ID" {
  run pebble state

  [ "$status" -eq 1 ]

  local message="error: The following required arguments were not provided:"
  [[ "$output" == "$message"* ]]
  [[ "$output" == *"<container-id>"* ]]
}
