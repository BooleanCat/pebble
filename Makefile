.PHONY = test build test-unit check

export PATH := target/release:$(PATH)

build:
	cargo build --release

test: check test-unit build
	cargo test -- --ignored

test-unit:
	cargo test

check:
	cargo check
	cargo fmt -- --check
	cargo clippy
