# Deployment Preparation

## Purpose

This document defines the build, start, migration, health check, and runtime configuration requirements for deploying Ahlan Commerce.

The goal is to make every runtime process explicit before deployment.

---

## Runtime Configuration

The application requires the following environment variables:

| Variable | Service | Purpose |
|---|---|---|
| `API_BIND_ADDR` | API | Address and port used by the API HTTP server |
| `DATABASE_URL` | API, Worker, Migrator | PostgreSQL connection string |
| `REDIS_URL` | API, Worker | Redis connection string |
| `ADMIN_PUBLIC_API_URL` | Admin | Public API URL used by the Admin frontend |

Example configuration:

```env
API_BIND_ADDR=0.0.0.0:3000
DATABASE_URL=postgres://user:password@postgres:5432/ahlan_commerce
REDIS_URL=redis://redis:6379
ADMIN_PUBLIC_API_URL=https://api.example.com
```

Production secrets must not be committed to the repository.

`.env.example` contains placeholder values only. Actual production values must be supplied by the deployment environment.

The API validates required environment variables during startup. Missing or invalid required configuration causes startup to fail with a clear error.

---

# Build And Start Commands

## API

### Build

```bash
cargo build --release -p api
```

Resulting binary: `target/release/api`

### Start

```bash
./target/release/api
```

Reads: `API_BIND_ADDR`, `DATABASE_URL`, `REDIS_URL`

Local dev: `make run` -> `cargo run -p api --bin api`

---

## Import Worker

### Build

```bash
cargo build --release -p import-worker
```

Resulting binary: `target/release/import-worker`

### Start

```bash
./target/release/import-worker
```

Local dev: `make run-worker` -> `cargo run -p import-worker`

The worker uses the required database and Redis configuration needed by its runtime responsibilities.

---

## Migrator

### Build

```bash
cargo build --release -p migrator
```

Resulting binary: `target/release/migrator`

### Start (one-off, not a long-running service)

```bash
./target/release/migrator
```

or, against the container image:

```bash
docker run --rm -e DATABASE_URL="$DATABASE_URL" ahlan-commerce:local /app/migrator
```

The migrator reads only `DATABASE_URL`. It embeds the project's migrations
(`refinery::embed_migrations!("../../db/migrations")`) directly into the
binary at compile time, so it needs no Atlas CLI, no scratch `dev`
database, and no network access to any migration registry — just the
target Postgres. It's idempotent: running it against an already-current
schema applies 0 migrations and exits `0`, so it's safe to run on every
deploy, not just the first.

---

## Admin Frontend

### Build

```bash
cd apps/admin
npm ci
npm run build
```

Requires: `ADMIN_PUBLIC_API_URL`, supplied through the Admin deployment
configuration, never committed.

---

# Database

## Start PostgreSQL

```bash
make db-start
```
Runs `docker compose up -d --wait`, blocking until Postgres reports healthy.

## Stop PostgreSQL

```bash
make db-stop
```
Runs `docker compose down`. The named volume is preserved.

---

# Database Migrations

The project uses **two different migration tools for two different jobs**
— do not conflate them.

## Atlas — local authoring only

Atlas is how migrations are written and generated, on a developer
machine, against a local scratch `dev` database. It is not used in
deployment.

```bash
make migrate          # atlas migrate apply --env local
make migrate-diff name=add_sku   # atlas migrate diff add_sku --env local
```

- `atlas migrate diff` creates/plans a migration from a schema difference.
- `atlas migrate apply` executes pending migrations against the *local*
  database.

## Migrator — deployment runtime

`atlas migrate apply` needs the Atlas CLI and a scratch `dev` database,
neither of which exists in a deployed container. In deployment, migrations
are applied by the `migrator` binary instead: it embeds the exact same
`db/migrations/*.sql` files Atlas already generated
(`refinery::embed_migrations!`, compiled into the binary at build time)
and applies whichever haven't run yet against `DATABASE_URL`. No Atlas
CLI, no scratch database, no registry access required — just this one
binary and the target Postgres.

Run it as a one-off command, using the same image the API/worker run
from, before (re)starting either:

```bash
docker run --rm -e DATABASE_URL="$DATABASE_URL" ahlan-commerce:latest /app/migrator
```

It's idempotent — running it against an already-current schema exits `0`
with "Applied migrations: 0," so it's safe to run on every deploy, not
just the first.

Atlas and `migrator` read the same `db/migrations/*.sql` directory, so
there remains exactly one source of truth for the schema. Atlas produces
the files; `migrator` applies them wherever the app actually runs.

Migration execution must be completed before application code depends on
newly introduced schema — i.e. run `migrator` before (re)starting
API/worker containers with code that expects the new schema.

---

# Health Check

```text
GET /health
```

```bash
make health   # curl -sf http://127.0.0.1:3000/health
```

For deployment, the health check should target the API service and its
configured port.

---

# Redis

```bash
make redis-health   # docker compose exec redis redis-cli ping -> PONG
make redis-logs
```

---

# Runtime Processes

| Process | Responsibility | Build | Start |
|---|---|---|---|
| API | HTTP REST/GraphQL API | `cargo build --release -p api` | `./target/release/api` |
| Worker | Background import processing | `cargo build --release -p import-worker` | `./target/release/import-worker` |
| Migrator | Applies pending migrations (one-off, not long-running) | `cargo build --release -p migrator` | `./target/release/migrator` |
| Admin | Frontend application | `npm ci && npm run build` | Frontend hosting runtime |
| PostgreSQL | Persistent relational database | Docker/service image | PostgreSQL service |
| Redis | Cache and worker coordination | Docker/service image | Redis service |

---

# Container Image (Docker)

A single multi-stage Dockerfile (repo root) builds the API, Worker, and
Migrator binaries from committed source in one image:

**Build stage:** `rust:1.94-slim`, runs
`cargo build --release -p api -p import-worker -p migrator` against the
committed workspace. No `.env` file or secret value is ever copied into
the build context or baked into a layer.

**Runtime stage:** `debian:bookworm-slim`, copies only the three compiled
binaries (`/app/api`, `/app/import-worker`, `/app/migrator`) out of the
build stage — no compiler toolchain, no source, in the final image.

```bash
docker build -t ahlan-commerce:latest .
```

Run each process from the same image with an explicit start command — no
default `CMD` is set in the image, so a service is never accidentally
running the wrong binary:

- API service start command: `/app/api`
- Worker service start command: `/app/import-worker`
- Migrator (one-off) command: `/app/migrator`

All three read `DATABASE_URL` / `REDIS_URL` (and, for the API,
`API_BIND_ADDR`) from the container's runtime environment, not from the
image, and fail startup immediately with a clear error if a required var
is missing.

The image declares a `HEALTHCHECK` that runs
`curl -sf http://127.0.0.1:3000/health`, matching `make health`. This
check is meaningful for the API service only — it has no effect when the
same image is run as the worker or the migrator, since neither owns an
HTTP endpoint (their liveness/success is exit-code based).

The image does not run migrations *automatically* — no `CMD` means
nothing starts on its own — but it DOES contain the migration tool:
`/app/migrator`, built from the same image as `/app/api` and
`/app/import-worker`. Run it as a one-off container command against the
deployment `DATABASE_URL` before the API/worker containers are (re)started
with new code. This replaces needing the Atlas CLI installed anywhere
near the deployment host.

---

# Local Development Commands

```bash
make start          # postgres + mprocs
make stop
make build           # cargo build --workspace
make run             # api
make run-worker      # worker
make test
make health
make redis-health
```

---

# Deployment Contract

```text
1. Build API
       ↓
2. Build Worker
       ↓
3. Build Migrator
       ↓
4. Build Admin
       ↓
5. Provide required environment variables
       ↓
6. Run database migrations (migrator, against DATABASE_URL)
       ↓
7. Start API
       ↓
8. Start Worker
       ↓
9. Serve Admin frontend
       ↓
10. Run API health check
```

Runtime boundaries:

```text
API
 ├── API_BIND_ADDR
 ├── DATABASE_URL
 └── REDIS_URL

Worker
 ├── DATABASE_URL
 └── REDIS_URL

Admin
 └── ADMIN_PUBLIC_API_URL

Migrator
 └── DATABASE_URL
```

Each service should receive only the environment variables required for
its responsibilities.

---

# Restart Independence

| Process | Can restart independently? | Notes |
|---|---|---|
| API | Yes | No in-memory state shared with worker/admin. |
| Worker | Yes | In-flight jobs should be idempotent/retry-safe; API unaffected. |
| Admin | Yes (redeploy of static assets) | No backend process to restart. |
| PostgreSQL | No — restart briefly interrupts API/worker DB calls | API/worker should retry/reconnect, not crash-loop, on transient DB unavailability. |
| Redis | No — restart briefly interrupts API/worker cache/queue calls | Same expectation: reconnect, don't crash-loop. |
| Migrator (`/app/migrator`) | N/A — one-off, not a long-running service | Re-running against an already-migrated database is a no-op (idempotent — see Database Migrations above). |