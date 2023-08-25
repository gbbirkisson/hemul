.PHONY: dev
dev:
	cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used
