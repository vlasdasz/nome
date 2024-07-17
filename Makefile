
include build/common.mk

ui:
	cargo run -p ui-test
	cargo run -p ui-test --release

fix:
	cargo fix --allow-dirty --allow-staged --all

lint:
	cargo clippy \
      -- \
      \
      -W clippy::all \
      -W clippy::pedantic \
      \
      -A clippy::missing_panics_doc \
      \
      -D warnings


.PHONY: mobile
