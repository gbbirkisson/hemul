CLIPPY_ARGS=-W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -A clippy::missing-const-for-fn

dev: test lint

.PHONY: test
test:
	cargo test

.PHONY: lint
lint: lint-fmt lint-clippy

.PHONY: lint-fmt
lint-fmt:
	cargo fmt -- --check

.PHONY: lint-clippy
lint-clippy:
	cargo clippy -- $(CLIPPY_ARGS)

.PHONY: fix
fix: fix-fmt fix-clippy

.PHONY: fix-fmt
fix-fmt:
	cargo fmt

.PHONY: fix-clippy
fix-clippy:
	cargo clippy --fix --allow-staged -- $(CLIPPY_ARGS)
