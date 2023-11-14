default: run

build:
	cargo build

run: build
	cargo run -- $(ARGS)

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

clean:
	cargo clean

pre-commit: fmt clippy test clean