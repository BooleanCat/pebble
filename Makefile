.PHONY = test build test-unit check

export PATH := target/release:$(PATH)

build:
	cargo build --release

test: check test-libpebble build test-pebble

test-libpebble:
	cargo test -p libpebble

test-pebble:
	cargo test -p pebble

check:
	cargo check
	cargo fmt -- --check
	cargo clippy
