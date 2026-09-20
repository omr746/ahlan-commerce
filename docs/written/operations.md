# Operations

How to observe and diagnose the running system.

## Where logs appear

A `tracing` event prints to the stdout of **whichever process executed
it** — not based on which crate the code lives in. Both binaries call
`init_tracing()` at startup, which installs the subscriber that formats
and filters events for that process.

| Pane | Contains |
|---|---|
| `api` | HTTP request logs, product writes, **all cache hit/miss/fallback logs**, storefront renders |
| `worker` | Job claims, import outcomes, retries, crash-recovery sweeps |
| `postgres` | Postgres server's own logs — connections, checkpoints. Not application logs. |
| `redis` | Redis server's own logs. Not your cache logs. |

Cache logs live in the `api` pane because `packages/cache` is a library —
it has no process of its own, so its events surface wherever the caller
runs. If `import-worker` ever calls into `cache`, those appear in `worker`.

Adjust verbosity with `RUST_LOG`:
```bash
RUST_LOG=info make run
RUST_LOG=api=debug,cache=debug,catalog_db=info make run
```
⚠️ A per-crate filter silently drops crates it doesn't name — `RUST_LOG=api=debug`
hides all `cache` logs.

## Log fields

**Requests** carry `request_id`, which correlates every log line for one
request and appears in error response bodies. Quote it in any bug report.

**Cache operations** carry `cache_key` and, on reads, `cache_event`
(`hit` or `miss`). Redis errors are passed through a redaction step that
strips credentials from anything URL-shaped before it reaches a log line.

**Import jobs** carry `job_id`, `attempt`, and `status`; failures add
`error_code`. File contents are never logged.

## Health checks

```bash
make health         # API  -> {"status":"ok"}
make redis-health   # Redis -> PONG
docker compose ps   # both services should be "running (healthy)"
```

## Diagnosing cache behavior

Watch the `api` pane and load a storefront page twice:
```bash
curl http://127.0.0.1:3000/products/coffee-mug   # cache miss, then a DB query
curl http://127.0.0.1:3000/products/coffee-mug   # cache hit, no DB query
```

Confirm invalidation works — after a publication change, the next load
must be a miss:
```bash
curl -X PATCH http://127.0.0.1:3000/api/products/<id> \
  -H 'content-type: application/json' -d '{"published":true}'
curl http://127.0.0.1:3000/products/coffee-mug   # cache miss again
```

Confirm Redis-down degradation — the page must still serve, as a 200:
```bash
docker compose stop redis
curl -i http://127.0.0.1:3000/products/coffee-mug   # 200, logs "cache unavailable, falling back to source"
docker compose start redis
```

Inspect a cached entry directly:
```bash
docker compose exec redis redis-cli GET "storefront:product-page:coffee-mug"
docker compose exec redis redis-cli TTL "storefront:product-page:coffee-mug"   # <= 300
```

## Diagnosing import jobs

```bash
curl "http://127.0.0.1:3000/api/import-jobs?status=failed"
curl "http://127.0.0.1:3000/api/import-jobs?status=queued"
```

`last_error` on a failed job carries a safe message — including the
duplicate handle when that's the cause. Retry:
```bash
curl -X POST http://127.0.0.1:3000/api/import-jobs/<job-id>/retry
```
This moves `failed -> queued` without resetting `attempts`. A job that has
used all 3 attempts can't be requeued; the retry returns an error saying so.

### Crash recovery

If the worker dies mid-import, its job stays `running` and nothing would
otherwise touch it — `running -> queued` isn't an allowed transition and
the worker only claims `queued` rows.

The worker sweeps rows stuck in `running` for over 5 minutes to `failed`
(a legal transition) with `last_error` noting the crash, logging
`error_code = "worker_crash_recovered"`. From there the normal retry path
applies. Net effect: a crash surfaces as a visible failure rather than a
silently stuck row, and never silently re-runs.

To recover immediately rather than waiting for the sweep, restart the
worker and wait up to ~30 seconds for its next sweep tick.

## Common failures

| Symptom | Likely cause |
|---|---|
| API won't start, "check DATABASE_URL" | Postgres not up (`make db-start`) or migrations unapplied (`make migrate`) |
| API starts but every request 500s | Migrations not applied — tables missing |
| Storefront page serves stale HTML | Invalidation didn't fire; check for a `cache delete failed` log, else wait out the 300s TTL |
| Jobs sit in `queued` forever | Worker isn't running (`make run-worker`), or every job is at 3 attempts |
| `cargo test` fails on cache tests | Redis not running (`make redis-health`) |
| CI fails on `make docs-api-check` | API changed without regenerating — run `make docs-api` and commit |

## Data and state

Postgres data and Redis data both live in named Docker volumes, so
`make db-stop` / `make db-start` preserves them. To wipe and start clean:

```bash
docker compose down -v    # removes volumes -- destroys all local data
make db-start
make migrate
```

Redis holds only derived cache entries. Flushing it is always safe —
every key can be recomputed from Postgres:
```bash
docker compose exec redis redis-cli FLUSHALL
```
