CLIPPY_ARGS=-W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -A clippy::missing-const-for-fn -A clippy::missing-errors-doc -A clippy::must-use-candidate -A clippy::new-without-default -A clippy::ignored-unit-patterns

TEST?=
TEST_ARGS?=-p hemul
VASM6502_OLDSTYLE?=$(PWD)/bin/vasm6502_oldstyle

.PHONY: dev
dev: lint test

.PHONY: test
test:
	@ which hexdump > /dev/null || (echo "hexdump is not installed" && false)
	VASM6502_OLDSTYLE=$(VASM6502_OLDSTYLE) cargo test ${TEST_ARGS} ${TEST}

.PHONY: coverage
coverage:
	@ which hexdump > /dev/null || (echo "hexdump is not installed" && false)
	rm -f tarpaulin-report.html
	VASM6502_OLDSTYLE=$(VASM6502_OLDSTYLE) cargo tarpaulin --out html ${TEST_ARGS} -- ${TEST}
	xdg-open tarpaulin-report.html

.PHONY: lint
lint: lint-fmt lint-clippy

.PHONY: lint-fmt
lint-fmt:
	cargo fmt -- --check

.PHONY: lint-clippy
lint-clippy:
	cargo clippy -- -D warnings --no-deps $(CLIPPY_ARGS)

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
