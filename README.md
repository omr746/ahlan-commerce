# Ahlan Commerce

> **Ahlan Commerce** is a small-scale version of a larger project, developed as a training model to gain practical experience and prepare for working on the full-scale project.

# Chapter 03 - In-Memory Product API

## Folder shape (Task 03.1)

```
product-api/
  Cargo.toml            # workspace root
  apps/api/              # the Axum binary - the only crate that knows about HTTP
  packages/catalog/       # the domain library - product rules, no web framework
```

`apps/api` depends on `packages/catalog`. `packages/catalog` depends on nothing
web-related. That direction only ever points one way.

## Running

```
cd apps/api  # or run from the workspace root with -p api
cargo run -p api
```

Config comes from env vars (Task 03.2):

```
APP_HOST=0.0.0.0 APP_PORT=3000 cargo run -p api
```

## Curl checks (Task 03.3 "Done when")

```bash
curl -s http://127.0.0.1:3000/health
# {"status":"ok"}

curl -s -X POST http://127.0.0.1:3000/api/products \
  -H 'content-type: application/json' \
  -d '{"title":"T-Shirt","handle":"t-shirt","price_cents":1999,"inventory_quantity":50,"published":true}'
# 201 Created
# {"id":"01a009f7-...","title":"T-Shirt","handle":"t-shirt","price_cents":1999,
#  "inventory_quantity":50,"published":true,
#  "created_at":"2026-08-16T09:47:18Z","updated_at":"2026-08-16T09:47:18Z"}

curl -s http://127.0.0.1:3000/api/products
# [{"id":"01a009f7-...", ...}]
```

`id` is a UUIDv7 (sortable by creation time). `created_at`/`updated_at` come
from the application clock, not the database (there isn't one yet).

## Tests

```
cargo test --workspace
```

- `packages/catalog`: unit tests on `Catalog` using `FixedIdGenerator` +
  `FixedClock`, so IDs and timestamps are exact, assertable values instead of
  "roughly now". Includes duplicate-handle and not-found domain checks.
- `apps/api`: HTTP-level tests (via `tower::ServiceExt::oneshot`) that drive
  the real router/handlers with fixed id/clock doubles injected into
  `AppState`, and a plain curl-based check above for a human to run by hand.
  Also covers all four error-contract paths (validation, duplicate, not
  found, malformed id) by status/code/shape only.
- `apps/api/src/error.rs`: unit tests on `AppError` mapping directly,
  including the internal/dependency-unavailable paths, asserting the
  response body never contains the real internal detail.

## Chapter 03A - Error handling

### Public error contract (Task 03A.1)

Every failure returns the envelope from `error-contract.md`:

```json
{
  "error": {
    "code": "validation_failed",
    "message": "Product title is required.",
    "request_id": "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d001"
  }
}
```

`packages/catalog/src/error.rs` defines `CatalogError` (domain-level: duplicate
handle, not found - no HTTP knowledge). `apps/api/src/error.rs` defines
`AppError`, which owns the public contract (`code()`, `status()`,
`public_message()`) and maps `CatalogError` into it via `AppError::from_catalog`.

```bash
# validation_failed - 400
curl -s -X POST http://127.0.0.1:3000/api/products -H 'content-type: application/json' \
  -d '{"title":"","handle":"x","price_cents":100,"inventory_quantity":1,"published":true}'

# duplicate_product_handle - 409 (run the create curl above twice with the same handle)

# not_found - 404
curl -s http://127.0.0.1:3000/api/products/018f5a3e-0000-7000-8000-0000000000ff
```

### Preserving internal causes (Task 03A.2)

`AppError::dependency_unavailable` / `AppError::internal` require a
`rootcause::Report` as their cause. `AppError::log()` is the *only* place
that cause is ever printed - the response body (`AppError::body()`) is built
solely from `code()`/`public_message()`/`request_id()` and never touches
`cause`.

```bash
cargo run -p api --example simulate_root_cause 2> docs/logs/root-cause-error.log
```

See `docs/logs/root-cause-error.log` for a captured run - the log line has
the real cause (a simulated cache connection failure), the stdout/response
body does not.

> **Toolchain note:** `rootcause` 0.13's MSRV is Rust **1.89+**. If your
> `rustc --version` is older, `cargo build` will fail while compiling
> `rootcause`'s dependency `hashbrown` with an `edition2024` error - that's
> the crate's MSRV, not a bug in this code. Upgrade with `rustup update
> stable` (or your platform's Rust install method) and rebuild.

### Error mapping tests (Task 03A.3)

`apps/api/src/app.rs` has HTTP-level tests for all four required codes
(`empty_title_returns_validation_failed`, `duplicate_handle_returns_409`,
`missing_product_returns_404`, `malformed_id_returns_validation_failed_not_500`),
asserting only status code + `error.code` + envelope shape - never internal
error text. `apps/api/src/error.rs` has direct unit tests on `AppError`,
including `dependency_unavailable`/`internal_error`, asserting the public
message never contains the simulated internal detail.

## Chapter 03B - Tracing and structured logs

### Startup (Task 03B.1)

`apps/api/src/observability.rs::init_tracing()` configures `tracing-subscriber`
from `RUST_LOG` once, in `main.rs`, before anything else runs:

```bash
RUST_LOG=info cargo run -p api
# 2026-...  INFO api: listening addr=0.0.0.0:3000

RUST_LOG=warn cargo run -p api
# (no "listening" line - it's INFO, filtered out by "warn")
```

### Request spans (Task 03B.2)

`observability::with_request_tracing` wraps the router with three
`tower-http` layers: `SetRequestIdLayer` (mints a UUIDv7 request id per
request via `MakeRequestUuid`), a customized `TraceLayer` (builds the
`http_request` span with `request_id`/`method`/`route`, then on response
records `status`/`latency_ms`/`error_code`), and `PropagateRequestIdLayer`
(echoes the id back as an `x-request-id` response header).

```bash
curl -s http://127.0.0.1:3000/health
# logs: http_request{request_id=... method=GET route=/health status=200 latency_ms=0.3}: request completed
```

Captured examples: `docs/logs/success-request.log`, `docs/logs/failed-request.log`.

### Domain events (Task 03B.3)

Handlers read the request id the tracing layer already assigned (via
`observability::current_request_id`) instead of minting their own - so the
same value appears in the HTTP span, the error envelope's `request_id`, and
any domain event for that request. `create_product` emits a `product
created` event with `product_id`/`product_handle` (identifiers only, never
the full request body). `AppError::log()` emits a `warn` (client-caused
failures) or `error` (internal/dependency failures, with the full
`rootcause::Report` chain) event carrying `error_code` from
`error-contract.md` - never a free-form label.

```bash
grep 01a019c8-6936 docs/logs/failed-request.log
# shows the create's http_request span AND its "product created" event,
# joined by the shared request_id
```

> Same toolchain note as 03A: this needs Rust 1.89+ for `rootcause`.

## Chapter 04.1 - Persisting with plain SQL

### What changed

`packages/catalog::PgCatalog` (in `postgres.rs`) is the Postgres-backed twin of
the in-memory `Catalog` from Chapter 03: the same three operations
(`create_product`/`list_products`/`get_product`), the same `Product`/
`ProductCreate` shapes, the same `CatalogError` variants (plus a new
`Storage` variant for genuinely unexpected DB failures). `AppState.catalog`
now holds a `PgCatalog` instead of `Arc<Mutex<Catalog>>`; handlers `.await`
it instead of locking a `Vec`. Because `Product`/`ProductCreate`/
`CatalogError` didn't change shape, `apps/api/src/dto.rs` needed **zero**
changes - the DTO/domain split from Chapter 03 paid for itself here.

Schema (as of Chapter 04.1/04.2): originally `db/schema.sql`, applied by
hand or via an app-level `create table if not exists` bootstrap at
startup. **As of Chapter 04.3, both are retired** - schema now lives at
`db/schema/products.sql` and is entirely Atlas's responsibility (see
below). `PgCatalog::connect()` no longer touches schema at all.

No migration-tracking table maintained by the app, no `sqlx::migrate!()`,
no `query!` macro - every SQL statement in `postgres.rs` is a literal
`&str`. Migration *tracking* is now Atlas's job (Chapter 04.3); the app
was never going to be the thing that owns that.

### Running against Postgres

```bash
createdb ahlan_commerce   # or: psql -c 'create database ahlan_commerce;'
atlas migrate apply --env local    # apply schema (Chapter 04.3) - see docs/atlas-command-notes.md
DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/ahlan_commerce cargo run -p api
```

`DATABASE_URL` defaults to that same local connection string if unset (see
`config.rs`) - convenient for local dev, override it for anything else.

### Verified: restarting the API does not lose products

```bash
$ cargo run -p api &                                    # process 1
$ curl -X POST .../api/products -d '{"handle":"ceramic-mug", ...}'
{"id":"01a02683-35c2-...", "handle":"ceramic-mug", ...}
$ kill %1                                                # process fully exits
$ cargo run -p api &                                     # process 2, brand new
$ curl .../api/products
[{"id":"01a02683-35c2-...", "handle":"ceramic-mug", ...}]   # still there
```

### What a connection pool does

`sqlx::PgPool` (created once in `PgCatalog::connect`, at startup) keeps a
small set of already-open TCP connections to Postgres ready to hand out.
Opening a fresh TCP connection and doing Postgres's authentication
handshake for every single request would be slow and would exhaust
Postgres's own `max_connections` limit under load. Instead: a handler asks
the pool for a connection, uses it for one query, and returns it to the
pool - the pool decides whether to keep it open for reuse or open a new one
up to its configured maximum (`max_connections(10)` here). `PgPool` is a
cheap, `Arc`-backed handle; cloning it (which `AppState::clone()` does on
every request, since `AppState: Clone`) does not open a new pool or a new
connection - every clone shares the same underlying set of connections.

### Why plain SQL worked for the first version

There's exactly one table and three operations. An ORM's value is managing
relationships, generating queries for arbitrary filters, and abstracting
over dialects - none of which exist yet here. `sqlx::query`/`query_as`
against literal SQL strings gives:
- **No hidden query** - what you read in `postgres.rs` is exactly what
  Postgres executes, easy to `EXPLAIN` or paste into `psql` directly.
- **No build-time coupling to a live database** - unlike `sqlx::query!`
  (which needs `DATABASE_URL` reachable at compile time to type-check
  against the real schema), plain `sqlx::query()` type-checks like any
  other Rust code; the "does this SQL/schema actually match" check happens
  at runtime, which is an acceptable trade for a one-table project.
- **Constraints do real work** - the `unique` constraint on `handle` is the
  actual enforcement mechanism for `duplicate_product_handle`, not a
  Rust-side check we'd have to keep in sync with the schema by hand.

This stops being the right call once there are many tables, joins, and
callers who need compile-time-checked queries or a shared query-builder
layer - that's a deliberate later chapter, not a gap in this one.

### Tests

`packages/catalog`'s in-memory `Catalog` unit tests remain fully hermetic -
no database needed. `apps/api`'s HTTP tests are now genuine integration
tests against a real Postgres:

```bash
# defaults to postgres://postgres:postgres@127.0.0.1:5432/ahlan_commerce
TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/ahlan_commerce \
  cargo test --workspace
```

Each test generates a unique product handle (`test-product-{uuid}`) rather
than truncating the table, so tests are safe to run concurrently against
the same live database without colliding.

## Chapter 04.2 - Changing the schema the painful way

Added `description text` (nullable) and `published_at timestamptz`
(nullable) to `products`, by hand, against a database that already had
data in it. Full account of what that actually involved - including a
real, captured `column "description" does not exist` failure and a real
demonstration of the *old* binary silently dropping data against the
*new* schema - is in `docs/manual-schema-change.md`. Short version:

- `db/schema.sql` (superseded by `db/schema/products.sql` as of Chapter
  04.3) described the current full desired shape at the time, but never
  described what to do to an existing database.
- The actual migration was two manual statements, run by hand:
  ```sql
  alter table products add column description text;
  alter table products add column published_at timestamptz;
  ```
- `PgCatalog::ensure_schema`'s `create table if not exists` bootstrap from
  Chapter 04.1 (also since removed) did not help here at all - it only
  ever handled brand-new databases, never schema evolution on an existing
  one. That gap is exactly what Chapter 04.3 (Atlas) exists to close.
- Product API rules added: `description` optional (missing/null both
  stored as `null`), `handle` must be lowercase/URL-safe, `price_cents`/
  `inventory_quantity` must be `>= 0` (validated by the app, not left to
  silently wrap on a bad cast), `published_at` set from the clock exactly
  when `published: true` at creation, `null` otherwise.

## Chapter 04.3 - Introducing Atlas

Replaces "a person remembers to run the right SQL in the right order"
(Task 04.2's pain) with a tool that tracks what's been applied.

```
atlas.hcl               # project config: schema source, target/dev DBs, migration dir
db/schema/products.sql   # desired-state schema
db/migrations/           # generated migration files (+ atlas.sum, once real)
```

```bash
atlas migrate diff initial_products --env local   # generate migrations from the schema file
atlas migrate apply --env local                   # apply pending migrations to the target DB
```

Full account - including what could and couldn't be verified in this
particular environment (I could not get a working `atlas` binary running
here; the SQL itself was verified directly with `psql` against scratch
databases instead) - is in `docs/atlas-command-notes.md`.

**Architectural change:** `PgCatalog::connect` no longer creates or
checks the schema at all - the Chapter 04.1 `create table if not exists`
bootstrap is gone. Schema is entirely Atlas's responsibility now; the app
just connects and queries. Run `atlas migrate apply --env local` before
`cargo run -p api` (and before running tests) if the `products` table
doesn't exist yet.

## Chapter 05.2 - Documenting the commands

A `Makefile` at the project root wraps the raw commands used throughout
this README so far, so nobody has to remember `atlas migrate apply --env
local` (or the rest) by hand:

​```bash
make build          # cargo build --workspace
make run            # cargo run -p api
make test           # cargo test --workspace
make health          # curl -sf http://127.0.0.1:$(APP_PORT)/health
make migrate         # atlas migrate apply --env local
make migrate-diff name=add_sku   # atlas migrate diff add_sku --env local
​```

Every target's one-sentence explanation plus the exact raw command it
wraps is in `docs/commands.md`. Deliberately not added yet: mprocs,
Redis, a worker target, or a Cornucopia regeneration target - all later
chapters.