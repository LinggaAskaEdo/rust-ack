.PHONY: build
build: update clean
	cargo build

.PHONY: update
update:
	cargo update

.PHONY: clean
clean:
	cargo clean

.PHONY: format
format:
	cargo fmt
	cargo clippy --fix --allow-dirty --allow-staged
	cargo check

.PHONY: all
all: update clean format build
	cargo run

.PHONY: watch
watch: format build
	cargo install cargo-watch
	cargo watch -x run

.PHONY: run
run:
	cargo run

.PHONY: hasher-format
hasher-format:
	(cd password_hasher && cargo fmt && cargo clippy --fix --allow-dirty --allow-staged && cargo check)

.PHONY: hasher-run
hasher-run:
	(cd password_hasher && cargo run)

.PHONY: hasher-all
hasher-all:
	(cd password_hasher && cargo clean && cargo update && cargo fmt && cargo clippy --fix --allow-dirty --allow-staged && cargo check && cargo run)

.PHONY: test-quick
test-quick:
	(cd etc/test && ./quick_test.sh)

.PHONY: test-api
test-api:
	(cd etc/test && ./api_test.sh)

.PHONY: test-load
test-load:
	(cd etc/test && ./load_test.sh 100)