API_KEY ?= API_KEY

.PHONY: lint test

lint:
	cargo clippy --all-features --all-targets

test:
	API_KEY=API_KEY cargo test --all-features  --all-targets