@test "delete missing container ID" {
  run pebble delete

  [ "$status" -eq 1 ]

  local message="error: The following required arguments were not provided:"
  [[ "$output" == "$message"* ]]
  [[ "$output" == *"<container-id>"* ]]
}

@test "delete for a non-existent container" {
  run pebble delete foo
  [ "$status" -eq 1 ]
  [ "$output" = 'error: no such container' ]
}
