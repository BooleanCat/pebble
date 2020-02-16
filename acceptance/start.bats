@test "start missing container ID" {
  run pebble start

  [ "$status" -eq 1 ]

  local message="error: The following required arguments were not provided:"
  [[ "$output" == "$message"* ]]
  [[ "$output" == *"<container-id>"* ]]
}

@test "start for a non-existent container" {
  run pebble start foo
  [ "$status" -eq 1 ]
  [ "$output" = 'error: no such container' ]
}
