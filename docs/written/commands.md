# Commands

Every `make` target in the project. Nothing needed to run or develop this
project lives outside this list.

## Running the stack

| Command | What it does |
|---|---|
| `make start` | Starts Postgres + Redis, then launches `mprocs` with `api`, `worker`, `postgres`, and `redis` panes |
| `make stop` | Stops the containers (`docker compose down`); named volumes survive |
| `make run` | Runs just the API (`cargo run -p api`) on `APP_PORT`, default 3000 |
| `make run-worker` | Runs just the import worker (`cargo run -p import-worker`) |

`make run` and `make run-worker` assume Postgres and Redis are already up.

## Databases

| Command | What it does |
|---|---|
| `make db-start` | `docker compose up -d --wait` — blocks until both Postgres and Redis are healthy |
| `make db-stop` | Stops the containers; data persists |
| `make db-logs` | Tails Postgres's own server logs |
| `make redis-logs` | Tails Redis's own server logs |
| `make redis-health` | `redis-cli ping` — prints `PONG` if Redis is reachable |
| `make health` | `curl /health` — proves the API is up (API must already be running) |

## Migrations and codegen

| Command | What it does |
|---|---|
| `make migrate` | Applies pending Atlas migrations to the local database |
| `make migrate-diff name=add_sku` | Generates a new migration by diffing `db/schema` against the database |
| `make cornucopia-generate` | Regenerates typed query code from `db/queries` into `packages/catalog-db/generated` |

⚠️ `make cornucopia-generate` introspects the live schema, so run
`make migrate` first whenever a query references new tables or columns.

## Build and test

| Command | What it does |
|---|---|
| `make build` | `cargo build --workspace` |
| `make test` | `cargo test --workspace` — needs a migrated Postgres and a running Redis |

## Documentation

### `make docs-api`

Regenerates **both** generated API artifacts:

- `docs/generated/openapi.json` — the REST contract, assembled from the
  `#[utoipa::path]` annotations on the handlers registered in
  `openapi::documented_router()`.
- `docs/generated/schema.graphql` — the GraphQL SDL, exported from the
  `Query` and `Mutation` type definitions.

Run this whenever you add, remove, or change a REST endpoint, a request or
response DTO, or a GraphQL query/mutation/type — then commit the result.

Needs no database, no Redis, and no running server: both artifacts are
derived from type definitions, not from a live app.

Never hand-edit the files in `docs/generated/`. Any manual change is
overwritten by the next `make docs-api` and will be flagged by
`make docs-api-check`.

### `make docs-api-check`

Fails when the checked-in generated artifacts no longer match the current
API shape — i.e. when someone changed the API but forgot `make docs-api`.

It regenerates into a temporary directory, diffs against what's committed,
prints the diff for whichever artifact drifted, and restores the committed
version so your working tree is left untouched either way.

Exit codes: `0` when both artifacts are current, `1` with a diff and a
"run `make docs-api`" message when either is stale.

This runs in CI on every push and pull request (`.github/workflows/ci.yml`,
the `docs` job), which is what keeps the committed artifacts honest. Run it
locally before pushing to catch staleness without waiting for CI.

## Viewing docs

| Where | What |
|---|---|
| <http://127.0.0.1:3000/docs/scalar> | REST docs UI (Scalar), served from the live spec |
| `docs/generated/schema.graphql` | GraphQL schema, read directly |
| `docs/api.md` | Index of both surfaces with links and these commands |
