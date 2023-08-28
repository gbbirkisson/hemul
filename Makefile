CLIPPY_ARGS=-W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -A clippy::missing-const-for-fn -A clippy::missing-errors-doc

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
fix: fix-clippy fix-fmt

.PHONY: fix-clippy
fix-clippy:
	cargo clippy --fix --allow-staged -- $(CLIPPY_ARGS)

.PHONY: fix-fmt
fix-fmt:
	cargo fmt

.PHONY: doc
doc:
	cargo doc
	xdg-open ./target/doc/hemul/index.html
