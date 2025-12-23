.PHONY: build test integ

build:
	cargo build

test:
	cargo test

integ:
	cargo test -- --ignored
