CLIPPY_ARGS=-W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -A clippy::missing-const-for-fn -A clippy::missing-errors-doc -A clippy::must-use-candidate -A clippy::new-without-default -A clippy::ignored-unit-patterns

dev: test lint

.PHONY: test
test:
	@ which hexdump > /dev/null || (echo "hexdump is not installed" && false)
	@ which vasm6502_oldstyle  > /dev/null || (echo "vasm6502_oldstyle is not installed" && false)
	cargo test

.PHONY: lint
lint: lint-fmt lint-clippy

.PHONY: lint-fmt
lint-fmt:
	cargo fmt -- --check

.PHONY: lint-clippy
lint-clippy:
	cargo clippy -- --no-deps $(CLIPPY_ARGS)

.PHONY: fix
fix: fix-clippy fix-fmt

.PHONY: fix-clippy
fix-clippy:
	cargo clippy --fix --allow-staged -- --no-deps $(CLIPPY_ARGS)

.PHONY: fix-fmt
fix-fmt:
	cargo fmt

.PHONY: doc
doc:
	cargo doc
	xdg-open ./target/doc/hemul/index.html
