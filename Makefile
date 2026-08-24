.PHONY: build run test health migrate migrate-diff

APP_PORT ?= 3000

# Compiles every crate in the workspace.
build:
	cargo build --workspace

# Runs the API locally on APP_PORT (default 3000).
run:
	cargo run -p api

# Runs every unit and integration test in the workspace. The apps/api
# integration tests need a reachable Postgres with migrations already
# applied - run `make migrate` first if the products table doesn't exist.
test:
	cargo test --workspace

# Checks that a running instance is up by hitting /health. Assumes the
# API is already running (e.g. via `make run` in another terminal).
health:
	curl -sf http://127.0.0.1:$(APP_PORT)/health

# Applies any pending Atlas migrations to the local database.
migrate:
	atlas migrate apply --env local

# Generates a new Atlas migration by diffing db/schema against the
# target database. Usage: make migrate-diff name=add_sku
migrate-diff:
	atlas migrate diff $(name) --env local