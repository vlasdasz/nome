include build/common.mk

run:
	cargo run

fix:
	cargo fix --allow-dirty --allow-staged --all

lint:
	cargo clippy --workspace --all-targets -- -D warnings

ci:
	typos
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets -- -D warnings
	cargo machete
