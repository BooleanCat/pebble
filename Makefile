.PHONY = test build

export PATH := target/release:$(PATH)

build:
	cargo build --release

test: build
	bats acceptance/state.bats
	bats acceptance/kill.bats
