# Commands

Every target below is defined in the `Makefile` at the project root. Run
`make <target>` from there.

## `make db-start`

Starts local Postgres (both `ahlan_commerce` and Atlas's scratch dev database) via Docker Compose and waits until it's ready to accept connections. Wraps: `docker compose up -d --wait`

## `make db-stop`

Stops the Postgres container without deleting its data. Wraps: `docker compose down`

## `make build`

Compiles every crate in the workspace. Wraps: `cargo build --workspace`

## `make run`

Starts the API locally on `APP_PORT` (default `3000`). Wraps: `cargo run -p api`

## `make test`

Runs every unit and integration test in the workspace. Wraps: `cargo test --workspace`

## `make health`

Checks that an already-running API instance is up by requesting `/health`. Wraps: `curl -sf http://127.0.0.1:$(APP_PORT)/health`

## `make migrate`

Applies any pending Atlas migrations to the local database. Wraps: `atlas migrate apply --env local`

## `make migrate-diff name=<migration_name>`

Generates a new Atlas migration by diffing `db/schema` against the target database. Wraps: `atlas migrate diff $(name) --env local`