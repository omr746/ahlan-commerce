# Atlas Command Notes

## Why now

Task 04.2 did a schema change by hand: edit Rust code, remember to also
run `ALTER TABLE` against the real database, in the right order, with no
tool checking any of it. It worked, but it worked because one person did
one two-column change carefully. `docs/manual-schema-change.md` has the
receipts on what that actually took to get right, and what it looks like
when it goes wrong (a captured, real "old code silently drops your data
against the new schema" demonstration). Atlas is introduced now,
immediately after feeling that, to replace "a person remembers to run the
right SQL in the right order" with "a tool tracks what's been applied and
refuses to let two people's changes silently diverge."

## A note on this environment

I was not able to get a working `atlas` binary running in this sandbox -
its official binary host and its Go module dependency tree both route
through domains outside this environment's network allowlist (tried
downloading the release binary directly, building from source with over
a dozen manual dependency remaps, and the npm-distributed wrapper - all
three hit the same wall at different points). That's a property of this
sandbox, not of Atlas.

Everything below that doesn't strictly require the binary was still done
for real: `db/schema/products.sql` and the migration file in
`db/migrations/` were verified by applying each with `psql` against
throwaway scratch databases and confirming they produce byte-identical
`\d products` output (see "What was actually verified" below). What's
missing is `atlas.sum` (the checksum file only the real tool should ever
write - see `db/migrations/README.md`) and an actual run of the two
commands below. Run them for real once you have `atlas` installed
somewhere with normal network access; the expected output described here
is accurate to how Atlas behaves, even though I couldn't capture it live.

## Minimum files

```
atlas.hcl              # project config: schema source, target/dev DBs, migration dir
db/schema/products.sql  # desired-state schema (what the table should look like)
db/migrations/          # generated migration files + atlas.sum
```

## Minimum commands

### `atlas migrate diff initial_products --env local`

Reads `db/schema/products.sql` (the `src` in `atlas.hcl`'s `local` env),
spins up the `dev` database as scratch space, replays every migration
already in `db/migrations/` there to reconstruct "what the database looks
like so far," diffs that against `src`, and - if there's a difference -
writes a new file into `db/migrations/` named
`<timestamp>_initial_products.sql`, plus updates `atlas.sum`.

For a brand-new project (empty `db/migrations/`), the diff is "nothing"
to "the whole `products` table," so the generated file is one `CREATE
TABLE` statement. That's exactly what's in
`db/migrations/20260822213000_initial_products.sql` in this project -
written by hand here, but verified to apply cleanly and produce a schema
structurally identical to `db/schema/products.sql` (see below).

### `atlas migrate apply --env local`

Connects to `url` (the real target - `ahlan_commerce`), checks its
`atlas_schema_revisions` table (which Atlas creates the first time it
touches a database) to see which migrations have already run, and runs
every migration file in `db/migrations/` that isn't recorded there yet,
in order, verifying each file's checksum against `atlas.sum` first.

Expected output for a fresh database, something like:

```
Migrating to version 20260822213000 (1 migration in total):

  -- migrating version 20260822213000
    -> CREATE TABLE "products" (...)
  -- ok (12.4ms)

  -------------------------
  -- 14.1ms
  -- 1 migration
  -- 1 sql statement
```

## What was actually verified

Since I couldn't run the real binary, I verified the *content* of the
files directly:

```bash
$ createdb atlas_migration_verify
$ psql atlas_migration_verify -f db/migrations/20260822213000_initial_products.sql
CREATE TABLE

$ createdb schema_source_verify
$ psql schema_source_verify -f db/schema/products.sql
CREATE TABLE

$ psql atlas_migration_verify -c '\d products'   # vs
$ psql schema_source_verify -c '\d products'
```

Both produced byte-identical output - same columns, same types, same
nullability, same `PRIMARY KEY`/`UNIQUE` constraints. The migration file
is a correct implementation of the desired schema; what's unverified is
specifically the Atlas tool's own bookkeeping (`atlas_schema_revisions`,
`atlas.sum`), not the SQL itself.

## Adopting Atlas onto the real `ahlan_commerce` database

`ahlan_commerce` already has a `products` table with exactly this shape -
it got there by hand in Task 04.2. Running `atlas migrate apply --env
local` against it as-is would try to `CREATE TABLE "products"` a second
time and fail (`relation "products" already exists`), because Atlas has
no record of that table's history - it was never told this database is
already at the "initial_products" version.

The real fix is baselining (`atlas migrate apply --baseline
<version>`), which tells Atlas "treat this database as already being at
this migration, don't try to re-run it" without touching any data. That's
deliberately out of scope here - the "Minimum commands" for this chapter
assume a fresh database, and baselining an existing one is worth its own
pass once it's needed for real. Noting it here so it doesn't come as a
surprise.

## Schema file vs. migration file

`db/schema/products.sql` answers "what should the table look like right
now" - one file, always overwritten to reflect the current desired state.
Nobody ever runs it directly against a real database by hand; it exists
so `atlas migrate diff` has something to compare against.

Each file in `db/migrations/` answers "what SQL, run in this exact order,
gets a database from empty to that current state" - an append-only log.
Changing the desired schema means editing `products.sql` and running
`migrate diff` again, which appends a *new* migration file describing
just the delta (e.g. a future `..._add_sku.sql` with an `ALTER TABLE`) -
it does not rewrite the first one. The rule "do not hand-edit
production-applied migration files after they are generated" exists
because every already-applied migration file is a permanent historical
record of SQL some real database actually ran; editing it after the fact
makes that record a lie about what happened, without changing what
already happened to any database that already ran it.

## Why migration history matters

`atlas_schema_revisions` (the table Atlas keeps on the target database)
is the thing that makes "did database X already get this change" a fact
Atlas can check, instead of a fact a person has to remember - which is
precisely what went wrong in Task 04.2: `PgCatalog`'s own `create table if
not exists` bootstrap *looked* like it was tracking whether the schema
was current, but it wasn't - it only ever checked "does a table with this
name exist at all," which is a much weaker question than "does this
database have every migration that's supposed to be applied to it." That
gap is exactly how the Task 04.2 failure mode was possible: the app
started up fine, logged nothing alarming, and the first real signal of
trouble was a request failing.

With real migration history: two different databases (a teammate's local
setup, staging, production) can be asked "what version are you at" and
compared directly. A migration that's already been applied is a fact
recorded in the database itself, not a thing someone has to remember they
ran, or infer from whether a column happens to exist. And the ordering
hazard from Task 04.2's reflection - deploy code first (loud failure) vs.
deploy schema first (silent failure) - has a third option once history is
tracked: `atlas migrate apply` becomes a single, scriptable, checkable
step that a deploy pipeline runs *before* the new code starts, so the
question "is the schema ready for this code" has a real answer instead of
a hoped-for one.
