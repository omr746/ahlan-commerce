# Setup

Everything needed to go from a fresh clone to a running stack. No hidden
steps — if a command isn't here, you shouldn't need it.

## Prerequisites

| Tool | Why | Install |
|---|---|---|
| Rust (stable) | Builds the workspace | <https://rustup.rs> |
| Docker + Compose | Runs Postgres and Redis | <https://docs.docker.com/get-docker/> |
| Atlas | Applies database migrations | `curl -sSf https://atlasgo.sh \| sh` |
| cornucopia | Generates typed SQL query code | `cargo install cornucopia` |
| mprocs | Runs the whole stack in one terminal | `cargo install mprocs` |

## 1. Environment

`DATABASE_URL` is required. `REDIS_URL` and `APP_PORT` have defaults.

```bash
export DATABASE_URL="postgres://postgres:132456@127.0.0.1:5432/ahlan-commerce?sslmode=disable"
export REDIS_URL="redis://127.0.0.1:6379"
export APP_PORT=3000
```

Put these in your shell profile or a `.env` you source — they're needed by
`make migrate`, `make cornucopia-generate`, and both binaries.

## 2. Start the databases

```bash
make db-start
```

Runs `docker compose up -d --wait`, which blocks until **both** Postgres
and Redis report healthy. The first run also creates the Atlas scratch
database via `db/docker-init/`.

Verify:
```bash
make redis-health     # -> PONG
```

## 3. Apply migrations

```bash
make migrate
```

## 4. Generate query code

```bash
make cornucopia-generate
```

Reads `db/queries/*.sql` against the live database and writes typed Rust
into `packages/catalog-db/generated`. Re-run this whenever you add or
change a `.sql` query file.

⚠️ Order matters: migrations must be applied **before** this, because
cornucopia introspects the real schema to infer types.

## 5. Build

```bash
make build
```

## 6. Run everything

```bash
make start
```

Starts the databases (if not already up) and launches `mprocs` with four
panes:

| Pane | Command | What it shows |
|---|---|---|
| `api` | `cargo run -p api` | HTTP server + all application logs |
| `worker` | `cargo run -p import-worker` | Import job processing logs |
| `postgres` | `docker compose logs -f postgres` | Postgres server's own logs |
| `redis` | `docker compose logs -f redis` | Redis server's own logs |

Press `q` in mprocs to stop all panes. `make stop` stops the containers.

## 7. Verify it works

```bash
make health                                          # -> {"status":"ok"}
curl http://127.0.0.1:3000/api/products              # -> []
open http://127.0.0.1:3000/docs/scalar               # REST docs UI
```

Create a product and view its storefront page:
```bash
curl -X POST http://127.0.0.1:3000/api/products \
  -H 'content-type: application/json' \
  -d '{"title":"Coffee Mug","handle":"coffee-mug","price_cents":2500,"inventory_quantity":12,"published":true}'

open http://127.0.0.1:3000/products/coffee-mug
```

Run an import job:
```bash
curl -X POST http://127.0.0.1:3000/api/import-jobs \
  -H 'content-type: application/json' \
  -d '{"input_path":"fixtures/products-import.json"}'
```
Watch the `worker` pane — the job moves `queued` → `running` → `succeeded`.

## 8. Tests

```bash
make test
```

Needs Postgres (migrated) and Redis running. The cache fallback tests
deliberately point at an unreachable address and pass regardless.

## Troubleshooting

**`service "redis" is not running`** — `docker compose up -d --wait` was
run before Redis was added to `docker-compose.yml`. Re-run `make db-start`.

**`make redis-health` fails but Postgres works** — check
`docker compose ps`; if `redis` is absent entirely, the `redis:` block is
probably mis-indented in `docker-compose.yml` (Compose won't error, it
just won't create the service).

**cornucopia fails with unknown table/column** — run `make migrate` first.

**No logs in the `api` pane** — check the filter level in `init_tracing()`
and `RUST_LOG`; a per-crate filter like `RUST_LOG=api=debug` silently
drops logs from `cache` and `catalog-db`.
