# Manual Schema Change: adding `description` and `published_at`

This is a record of what actually happened doing this change by hand
against a database that already had one row in it - not a clean-slate
walkthrough. Every command below and every error message is a real
capture from doing this, not a reconstruction.

## Starting state

```
$ psql -d ahlan_commerce -c '\d products'
       Column       |           Type
--------------------+--------------------------
 id                 | uuid
 title              | text
 handle             | text
 price_cents        | integer
 inventory_quantity | integer
 published           | boolean
 created_at         | timestamptz
 updated_at         | timestamptz

$ psql -d ahlan_commerce -c 'select id, title, published from products;'
                  id                  |    title    | published
--------------------------------------+-------------+-----------
 01a02683-35c2-7113-8af2-f9d86b8b46a7 | Ceramic Mug | t
```

One existing, already-published row, from Chapter 04.1.

## Step 1 (the mistake, done on purpose): update the Rust code first

Updated `Product`/`ProductCreate` (packages/catalog), the DTOs
(apps/api/src/dto.rs), and every SQL string in `postgres.rs` to reference
`description` and `published_at` - **without touching the database.**

Built and ran the new binary against the *old*, un-migrated database.
Startup itself succeeded, silently:

```
INFO api: listening addr=0.0.0.0:3020
```

`PgCatalog::ensure_schema`'s `create table if not exists` is a no-op
against a table that already exists - it does not add missing columns.
There is no signal at boot that code and schema have drifted apart.

The failure only showed up on the first real request:

```
$ curl http://127.0.0.1:3020/api/products
{"error":{"code":"dependency_unavailable","message":"a required dependency is unavailable","request_id":"01a02a41-a481-73e0-b548-2a08b03ebb5f"}}
# HTTP 503
```

And the engineer-facing log line (never shown to the client) had the real
cause:

```
ERROR api::error: request failed with an internal error
  request_id=01a02a41-a481-73e0-b548-2a08b03ebb5f
  error_code="dependency_unavailable"
  root_cause="* product storage query failed
* error returned from database: column \"description\" does not exist"
```

`POST /api/products` failed the same way, with Postgres's own message
naming the exact problem:

```
error returned from database: column "description" of relation "products" does not exist
```

This is the "code and schema disagree" failure mode: loud, correctly
mapped to `dependency_unavailable` (503) by the error-handling work from
Chapter 03A, safe (no column name or SQL leaked to the client) - but every
single request to `/api/products` was down until the schema caught up.

## Step 2: the actual manual migration

```sql
ALTER TABLE products ADD COLUMN description text;
ALTER TABLE products ADD COLUMN published_at timestamptz;
```

Both nullable, no `DEFAULT`, no `NOT NULL` - matching the contract
("missing or null description is stored as null"; `published_at` is only
ever set by application code, never a database default). Nullable
`ADD COLUMN` on Postgres 11+ doesn't rewrite the table, so this was fast
even in principle for a much bigger table.

```
$ psql -d ahlan_commerce -c '\d products'
       Column       |           Type
--------------------+--------------------------
 ...
 description        | text
 published_at       | timestamptz
```

## Step 3: what happened to the existing row

```
$ psql -d ahlan_commerce -c "select title, published, published_at from products where handle='ceramic-mug';"
    title    | published | published_at
-------------+-----------+--------------
 Ceramic Mug | t         |
```

**`published = true` and `published_at = null`, on the same row.** This is
a state the new code never produces going forward (`create_product`
always sets `published_at` when `published` is `true`) - but it's exactly
what an `ADD COLUMN` with no backfill produces for anything that existed
before the migration. We deliberately did not invent a `published_at` for
this row (e.g. backfilling it with `created_at`) - we don't actually know
when it was published, and writing down a guess as if it were a fact would
be worse than an honest `null`. The API reflects this honestly too:

```
$ curl http://127.0.0.1:3021/api/products
[{"id":"01a02683-...","title":"Ceramic Mug","published":true,"published_at":null,"description":null,...}]
```

## Step 4: confirmed working, post-migration

```
$ curl -X POST .../api/products -d '{"title":"Coffee Mug","handle":"coffee-mug","description":"Ceramic mug for daily coffee.","price_cents":2500,"inventory_quantity":12,"published":true}'
{"...","description":"Ceramic mug for daily coffee.","published_at":"2026-08-22T16:16:01.55...Z",...}

$ curl -X POST .../api/products -d '{"title":"Draft Product","handle":"draft-product","price_cents":100,"inventory_quantity":1,"published":false}'
{"...","description":null,"published_at":null,...}
```

Validation rules confirmed too (all return the standard error envelope,
not a raw framework rejection):

```
$ curl -X POST ... -d '{"price_cents":-500,...}'
{"error":{"code":"validation_failed","message":"price_cents must be greater than or equal to 0.",...}}  # 400

$ curl -X POST ... -d '{"handle":"Bad-Handle",...}'
{"error":{"code":"validation_failed","message":"Product handle must be lowercase letters, numbers, and hyphens only.",...}}  # 400
```

## Step 5: the scarier direction - old code against the new schema

To check the *other* ordering hazard, we ran the **previous (Chapter
04.1) binary** - built before any of this change, kept around
specifically for this test - against the now-migrated database.

```
$ ./api-chapter-04-1-binary   # old code, new schema
INFO api: listening addr=0.0.0.0:3022
```

Starts fine. Serves `GET /api/products` fine (200, existing rows
returned - just without the two new fields, since the old response DTO
doesn't know they exist). Then:

```
$ curl -X POST .../api/products \
  -d '{"title":"Silent Product","handle":"silent-product","description":"This will be silently dropped.","price_cents":999,"inventory_quantity":3,"published":true}'
{"id":"01a02a42-...","title":"Silent Product","handle":"silent-product","price_cents":999,"inventory_quantity":3,"published":true,"created_at":"...","updated_at":"..."}
# HTTP 201 - clean, no error, no warning anywhere
```

The log for this request is completely unremarkable:

```
INFO api::app: product created request_id=... product_id=... product_handle=silent-product
INFO api::observability: request completed status=201 latency_ms=1.83
```

And the actual row in the database:

```
$ psql -d ahlan_commerce -c "select title, description, published_at from products where handle='silent-product';"
     title      | description | published_at
----------------+-------------+--------------
 Silent Product |             |
```

The client explicitly sent a `description` and `published: true`. Both
were silently discarded. Nothing failed. Nothing logged a warning. A
health check hitting this endpoint would report green the entire time.

## Reflection

### What did you have to remember?

- That `PgCatalog::ensure_schema`'s `create table if not exists` only
  helps a *fresh* database - it does nothing for a table that already
  exists, so "restart the app" is not a migration strategy once there's
  real data.
- Every place a column name appears as a literal string: the `INSERT`
  column list, the `INSERT` `VALUES` placeholder list (and keeping the
  *positional* order of `.bind()` calls lined up with it), the `SELECT`
  column list (twice - `list_products` and `get_product` each have their
  own), and `product_from_row`'s field-by-field mapping. Nothing checks
  at compile time that these four places agree with each other or with
  the database - "plain SQL, no codegen" (Chapter 04.1's own tradeoff)
  means this is entirely on the person doing the change.
- The domain struct (`Product`), the "create" struct (`ProductCreate`),
  and both DTOs (`ProductCreateRequest`, `ProductResponse`) each needed
  the new fields added by hand, in four different files, and they don't
  all have the same shape (`ProductResponse` needs the field even though
  a client never *sends* it; `ProductCreateRequest` needs it as optional
  input).
- That existing rows get `null` for new nullable columns with no
  backfill, and that treating "unpublished" and "no `published_at`"
  as always meaning the same thing was no longer safe once legacy rows
  existed - an invariant the new code relies on that data written before
  the migration doesn't satisfy.
- The order to actually do this in: code first crashes loudly and
  immediately on every request; database first (which we didn't do here,
  but reasoned through and then verified in Step 5) doesn't crash at all -
  it just quietly stops persisting the new fields until the code catches
  up. Loud-first was the safer mistake to make.

### What broke when Rust code and DB schema disagreed?

Every request to `/api/products` (both `GET` and `POST`) failed with a
`503 dependency_unavailable`, for the entire window between deploying the
new code and applying the `ALTER TABLE` statements. The cause (`column
"description" does not exist`) was visible in the log but never in the
response body - Chapter 03A's discipline (never leak internal errors to
clients) held up correctly even under a real failure, not just the
simulated one from that chapter's example.

### Why would this be dangerous in production?

Because the *other* ordering - schema changed, code not yet deployed, or
old code still running somewhere (a canary, a not-yet-restarted replica,
a rollback) - produces **no error at all**. Step 5 above is the proof: a
`201 Created`, a clean log line, a healthy-looking status code, while
customer-submitted data (`description`) is silently discarded and a
business rule (`published_at` should exist for a published product) is
silently violated. A loud failure gets noticed and fixed within minutes.
A silent one gets noticed whenever someone happens to check the actual
data - which, in real incidents, is often "when a customer complains,"
weeks later, after a rollback or an old instance served thousands of
requests this way. The fix a real migration tool (Atlas, `sqlx migrate`,
etc.) provides isn't "makes ALTER TABLE easier" - it's "makes it possible
to deploy schema and code changes together, atomically, so this ordering
question stops being something a human has to get right by hand every
time."

## Atlas Schema Migration

Atlas was introduced to solve the **schema-change discipline** problem.

The desired database schema is defined in `db/schema/products.sql`, Atlas generates versioned migrations under `db/migrations/`, and migrations are applied to PostgreSQL using:

```bash
atlas migrate diff initial_products --env local
atlas migrate apply --env local
```

Atlas makes schema changes explicit, versioned, and reproducible. However, Atlas does **not** provide Rust query safety or type-check Rust SQL usage. Atlas manages the database schema and migration history, while Rust/SQLx remains responsible for query correctness and type safety.
