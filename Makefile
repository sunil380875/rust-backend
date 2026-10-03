.PHONY: all run build dev fmt clippy

all: fmt clippy build

run:
	cargo run

build:
	cargo build --release

dev:
	cargo watch -x run

fmt:
	cargo fmt --all

clippy:
	cargo clippy -- -D warnings
