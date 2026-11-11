.PHONY: build run test lint

build:
	cargo build --release

run:
	cargo run --quiet

test:
	cargo test

lint:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings
