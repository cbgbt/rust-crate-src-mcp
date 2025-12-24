.PHONY: build release test lint format format-check check integ deny

build:
	cargo build --quiet

release:
	cargo build --quiet --release

test:
	cargo test --quiet

integ:
	cargo test --quiet -- --ignored

lint:
	cargo clippy --quiet -- -D warnings
	@python3 scripts/lint_loc.py

format:
	cargo fmt --quiet

format-check:
	cargo fmt --quiet -- --check

deny:
	cargo deny --locked check

check: format .WAIT test lint format-check deny
