# Data Access Layer (DAL)

This document explains how schema, migrations, SQL, and generated Rust code
fit together, and why the boundary is drawn where it is.

## Where the schema lives

The desired database schema is declared under `db/schema/products.sql`
(Atlas's declarative SQL schema format — one `CREATE TABLE` etc. per file,
one file per table/domain as more are added). This is the single source of
truth for what the database *should* look like: table shapes, columns,
constraints, indexes. Nobody hand-writes `ALTER TABLE` migrations against
this directly; Atlas diffs the declared schema against the current database
state and generates the migration for you.

Atlas's own configuration (`atlas.hcl`) lives at the repo root
(`ahlan-commerce/atlas.hcl`) and points Atlas at `db/schema/` as the
desired-state source and `db/migrations/` as the migration directory.

## Where migrations live

Generated, versioned migration files live under `db/migrations/`. These are
Atlas's output, not something edited by hand. Each migration is a numbered,
immutable SQL file representing one schema change, plus a checksum file
(`atlas.sum`) that Atlas uses to detect drift or out-of-order edits.

`db/docker-init/` holds the scripts/SQL used to bootstrap a local Postgres
container (extensions, roles, initial DB creation) — this is separate from
Atlas migrations and only runs once when a fresh database container is
created.

## Where SQL query files live

Application queries — the actual `SELECT`/`INSERT`/`UPDATE` statements the
app runs — live under `db/queries/products/` (one subfolder per domain as
more are added), one `.sql` file per query:

```
db/queries/products/create_product.sql
db/queries/products/list_products.sql
db/queries/products/list_published_products.sql
db/queries/products/update_product_publication.sql
```

These are plain `.sql` files annotated with Cornucopia's query-name comment
syntax (`--! query_name : type`). They are hand-written and reviewed like
any other code — this is the "SQL-first" part of the DAL: queries are real
SQL, not built up through a query-builder DSL.

> `db/queries/products/test_null.sql` currently sits alongside the four
> contract queries above. If this was a scratch file used while working out
> nullable-column handling, remove it before merging — Cornucopia will
> generate bindings for *every* `.sql` file in this directory, so stray
> files become real (if unused) generated code, and the query-contract
> only lists the four files above.

## How Atlas and Cornucopia differ

They operate on opposite ends of the same schema and solve different
problems:

| | Atlas | Cornucopia |
|---|---|---|
| Input | `db/schema/*.sql` (desired schema) | `db/queries/**/*.sql` (hand-written queries) + live DB schema |
| Output | Versioned migration files in `db/migrations/` | Generated Rust structs + typed query-calling functions |
| Answers | "How do I get the database from its current shape to the shape I want?" | "Given this exact query against this exact schema, what Rust types come out?" |
| Runs against | The database's current vs. declared schema | The database's *current* schema, to type-check your queries |

Atlas owns schema evolution. Cornucopia owns turning SQL text into
compile-time-checked Rust bindings. Cornucopia never changes the schema —
it only reads it (against a real or ephemeral DB) to validate and type your
queries.

## When to run Atlas

Run Atlas (`atlas migrate diff` / `atlas migrate apply`, via `atlas.hcl` at
the repo root) whenever you change `db/schema/*.sql` — i.e. whenever a
table, column, index, or constraint needs to change. This produces a new
migration file under `db/migrations/`, which then gets applied to a
dev/test database.

## When to run Cornucopia

Run `make cornucopia-generate` whenever:
- You add, remove, or edit a `.sql` file under `db/queries/`, or
- You've just run an Atlas migration that changes a table/column a query
  depends on (so the generated Rust types stay accurate).

Cornucopia generation is idempotent and safe to rerun at any time — it
just regenerates `packages/catalog-db/generated/` from the current schema
and query files. Rerun it in CI to catch drift between the checked-in
generated code and what the queries/schema currently say.

## Why handlers do not own persistence

Handlers (HTTP layer) are responsible for parsing requests, calling into
domain/DAL functions, and mapping results/errors to HTTP responses. They do
not contain SQL and do not call Cornucopia-generated query functions
directly, for a few reasons:

1. **Testability.** DAL functions (`PgCatalog::create_product`, etc.) can be
   integration-tested against a real database independently of HTTP
   framing, request parsing, or routing.
2. **Single point of schema-coupling.** Only the DAL package
   (`packages/catalog-db`) knows about Postgres-specific types
   (`tokio_postgres`, `deadpool_postgres`, Cornucopia's generated row
   structs, `FixedOffset` vs `Utc` conversions). If the database or driver
   changes, only this layer changes — handlers and domain logic are
   unaffected.
3. **Consistent error semantics.** The DAL translates low-level database
   errors (unique violations, no-rows-returned, pool exhaustion) into a
   small domain-level error enum (`CatalogDbError`). Handlers (via
   `AppError`) then map *that* into HTTP status codes, rather than every
   handler having to know what a Postgres `SqlState` code means.
4. **Enforces the boundary at compile time.** Generated query functions
   live behind `PgCatalog`'s public API rather than being re-exported, so
   handler code physically cannot import and call
   `catalog_db_queries::queries::products::*` directly — it can only go
   through the DAL's typed, domain-shaped methods.

## Regenerating after a schema change (worked example)

1. Edit `db/schema/products.sql` to add/change a column.
2. `make atlas-migrate` (or equivalent) — Atlas diffs against
   `db/migrations/` and writes a new migration file, then applies it to
   your dev DB.
3. Update the relevant `.sql` file(s) under `db/queries/products/` if the
   query needs to reference the new column.
4. `make cornucopia-generate` — regenerates
   `packages/catalog-db/generated/`, which will fail to compile if a query
   references a column/type that no longer matches the schema.
5. Update `PgCatalog` methods in `packages/catalog-db/src/dal/products.rs`
   if the domain-facing `Product`/`ProductCreate` struct needs the new
   field.
