@test "kill missing container ID" {
  run pebble kill

  [ "$status" -eq 1 ]

  local message="error: The following required arguments were not provided:"
  [[ "$output" == "$message"* ]]
  [[ "$output" == *"<container-id>"* ]]
}

@test "kill for a non-existent container" {
  run pebble kill foo SIGTERM
  [ "$status" -eq 1 ]
  [ "$output" = 'error: no such container' ]
}

@test "kill with unknown signal" {
  run pebble kill foo SIGFOO
  [ "$status" -eq 1 ]
  [ "$output" = "error: Invalid value for '<signal>': unknown signal" ]
}
