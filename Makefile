.PHONY = test build test-unit check

export PATH := target/release:$(PATH)

build:
	cargo build --release

test: check test-unit build
	bats acceptance/state.bats
	bats acceptance/create.bats
	bats acceptance/start.bats
	bats acceptance/kill.bats
	bats acceptance/delete.bats

test-unit:
	cargo test

check:
	cargo check
	cargo fmt -- --check
	cargo clippy
