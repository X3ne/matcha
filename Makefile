prog := matcha-back

debug ?= 0

$(info debug is $(debug))

ifneq ($(debug), 0)
  release :=
  target :=debug
  extension :=debug
  rust_log :=debug
else
  release :=--release
  target :=release
  extension :=
  rust_log :=info
endif

build:
	cargo build $(release)

dev:
	RUST_LOG=$(rust_log) cargo watch -x "run -- $(prog) $(ARGS)"

test:
	cargo test

sync_test:
	cargo test -- --test-threads 1

db_run:
	docker-compose up -d

db_down:
	docker-compose down

db_migration:
	sqlx migrate run

db_reset:
	sqlx database reset

help:
	@echo "usage: make $(prog) [debug=1]"

.DEFAULT_GOAL := dev
