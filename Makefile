.PHONY = test build

export PATH := target/release:$(PATH)

build:
	cargo build --release

test: test-unit build
	bats acceptance/state.bats
	bats acceptance/kill.bats
	bats acceptance/start.bats

test-unit:
	cargo test
