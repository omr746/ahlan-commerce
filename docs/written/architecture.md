# Architecture

## Workspace layout

```
apps/
  api/              HTTP server: REST, GraphQL, storefront HTML, docs UI
  import-worker/    Background process: polls and runs import jobs
packages/
  catalog/          Domain types (Product, ImportJob) + Clock/IdGenerator traits
  catalog-db/       PostgreSQL access via cornucopia-generated queries
  cache/            Redis primitives + cache key constants
  storefront/       Render context builder + HTML renderer (pure, no I/O)
db/
  schema/           Desired-state SQL (Atlas source of truth)
  migrations/       Atlas-generated migration files
  queries/          cornucopia query definitions
```

## Dependency direction

```
apps/api ──────┬──> packages/catalog-db ──> packages/catalog
               ├──> packages/cache
               ├──> packages/storefront ──> packages/catalog
               └──> packages/catalog

apps/import-worker ──┬──> packages/catalog-db
                     ├──> packages/catalog
                     └──> apps/api (for dto validation + AppState)
```

Domain types (`catalog`) depend on nothing. `storefront` depends only on
domain types — it can't reach a database or a cache even by accident.
`catalog-db` and `cache` are the only crates that do I/O.

⚠️ One wrinkle worth knowing: `import-worker` depends on `apps/api` to
reuse `ProductCreateRequest::validate()` and `AppState`. That's a
deliberate tradeoff — it avoids duplicating Chapter 04's validation rules
in two places, at the cost of a binary depending on another binary's
library target. If that coupling becomes a problem, the fix is to move the
DTO and its validation down into `packages/catalog`.

## Two processes, one codebase

`api` and `import-worker` are separate binaries, each with its own
`main()`, each running as its own OS process with its own connection pool.

This matters for reading logs: a `tracing` event appears in the pane of
whichever **process executed it**, never based on which crate defined it.
Cache logs triggered by an HTTP request appear in the `api` pane even
though the code lives in `packages/cache`.

## Request paths

### JSON API — `GET /api/products`
```
axum handler (apps/api)
  -> PgCatalog::list_products (packages/catalog-db)
    -> cornucopia generated query -> PostgreSQL
  -> ProductResponse DTO -> JSON
```

### Storefront page — `GET /products/{handle}`

Three responsibilities, deliberately in three places:

```
apps/api/src/storefront_handler.rs        OWNS FRESHNESS
  |  cache-aside: try Redis, TTL 300s on write, invalidation on product writes
  |
  ├─> cache::Cache::get_json / set_json   (packages/cache)
  |
  ├─> PgCatalog::get_published_product_by_handle   LOADS DATA
  |     (packages/catalog-db) -- the only code touching PostgreSQL
  |
  └─> ProductPageContext::from_product + render_product_page   RENDERS HTML
        (packages/storefront) -- pure functions, no I/O, no clock,
        no awareness that a cache exists
```

Read path: Redis GET → hit returns cached HTML; miss, Redis error, or
unparseable JSON all fall through to Postgres + render → Redis SET with
300s TTL → return HTML. A Redis SET failure is logged and the HTML is
still returned.

Freshness is primarily **explicit invalidation** (`create_product` and
`update_product` delete the key). TTL is a safety net for anything
invalidation misses, not the main mechanism.

### Import job
```
POST /api/import-jobs (apps/api)  -> inserts a 'queued' row, returns 202
                                     (does NOT read or validate the file)

import-worker loop (separate process)
  -> claim_next_queued_job: queued -> running, attempts += 1 atomically
  -> read + parse the JSON file
  -> per product: validate, then PgCatalog::create_product
  -> mark_succeeded, or mark_failed with a safe last_error
```

The API only validates that the request is structurally valid. The worker
owns file validation and import. A missing file makes the *job* fail; it
doesn't fail the enqueue request.

## Key design decisions and tradeoffs

**Plain SQL, not an ORM.** Atlas manages schema as declarative SQL;
cornucopia generates typed Rust from hand-written queries. Cost: two
codegen steps and an ordering constraint (migrate before generate).
Benefit: the SQL that runs is the SQL in the repo.

**Cache failures never fail a request.** Every `cache` operation returns
`Option`/`()` rather than `Result`, so callers can't accidentally
propagate a Redis outage into a 500. Cost: a caller can't distinguish "not
cached" from "Redis is down" — intentional, since the correct response is
identical either way.

**404s are not cached.** A missing or unpublished handle can become
published at any moment; caching the negative would serve a stale 404 for
up to the full 300s TTL.

**Missing and unpublished return the same 404.** An unpublished product's
existence isn't discoverable from the storefront.

**Worker retries are operator-triggered.** `failed -> queued` happens via
`POST /api/import-jobs/{id}/retry`, not automatically, so `failed` keeps
meaning "needs attention" rather than "will silently retry." Attempts cap
at 3, enforced in the claim query's `WHERE attempts < 3`.

**Crash recovery via a staleness sweep.** A worker that dies mid-import
leaves a row stuck in `running`, which nothing would otherwise touch
(`running -> queued` isn't an allowed transition). The worker sweeps rows
stuck in `running` past 5 minutes to `failed` — a legal transition — from
which the normal retry path applies.

**Docs are generated from types, not from a running app.** Both
`openapi.json` and `schema.graphql` are derived from type definitions, so
`make docs-api` and CI's staleness check need no database and no server.
