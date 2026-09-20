.PHONY: build run docs-api-check docs-api test health run-worker migrate migrate-diff db-start db-stop db-logs start stop cornucopia-generate redis-health redis-logs

APP_PORT ?= 3000

# Starts local Postgres via Docker Compose and waits until it reports
# healthy (per the healthcheck in docker-compose.yml) before returning -
# so a `make migrate` or `make test` run right after this never races
# against a database that's still starting up. First run also creates
# the Atlas scratch "dev" database (see db/docker-init/).
start:
	$(MAKE) db-start
	mprocs
stop:
	$(MAKE) db-stop
run-worker:
	cargo run -p import-worker
db-start:
	docker compose up -d --wait

# Stops the Postgres container. Data survives (named volume, not
# removed) - `make db-start` again resumes with the same data.
db-stop:
	docker compose down

# Compiles every crate in the workspace.
build:
	cargo build --workspace

# Runs the API locally on APP_PORT (default 3000).
run:
	cargo run -p api --bin api

db-logs:
	docker compose logs -f postgres
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
cornucopia-generate:
	cornucopia live "$$DATABASE_URL" \
		--queries-path db/queries \
		--destination packages/catalog-db/generated \
		--async true

# Proves Redis is reachable — same idea as `health` for the API.
redis-health:
	docker compose exec redis redis-cli ping

# Tails Redis's logs, same pattern as db-logs.
redis-logs:
	docker compose logs -f redis
docs-api:
	cargo run -p api --bin export-docs

docs-api-check:
	@set -e; \
	tmp="$$(mktemp -d)"; \
	trap 'rm -rf "$$tmp"' EXIT; \
	mkdir -p "$$tmp/docs/generated"; \
	cp -r docs/generated "$$tmp/committed"; \
	cargo run -q -p api --bin export-docs; \
	if ! diff -u "$$tmp/committed/openapi.json" docs/generated/openapi.json; then \
		echo ""; \
		echo "ERROR: docs/generated/openapi.json is stale."; \
		echo "The REST API shape changed but the generated spec was not updated."; \
		echo "Run 'make docs-api' and commit the result."; \
		cp "$$tmp/committed/openapi.json" docs/generated/openapi.json; \
		exit 1; \
	fi; \
	if ! diff -u "$$tmp/committed/schema.graphql" docs/generated/schema.graphql; then \
		echo ""; \
		echo "ERROR: docs/generated/schema.graphql is stale."; \
		echo "The GraphQL schema changed but the exported SDL was not updated."; \
		echo "Run 'make docs-api' and commit the result."; \
		cp "$$tmp/committed/schema.graphql" docs/generated/schema.graphql; \
		exit 1; \
	fi; \
	echo "generated docs are up to date"