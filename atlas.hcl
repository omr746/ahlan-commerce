# Atlas project configuration. Every `atlas` command below is run with
# `--env local`, which points at this block - the env name is how Atlas
# knows which schema, which databases, and which migration directory a
# command applies to (a later `staging`/`prod` env would point at
# different database URLs but the same schema source and migration dir).

env "local" {
  # The desired schema, in plain SQL - not HCL. Atlas supports both; we
  # use SQL here because Chapter 04's whole stance has been "plain SQL,
  # not a DSL that generates SQL for you" (see db/schema/products.sql).
  src = "file://db/schema/products.sql"

  # The database Atlas actually manages: where `migrate apply` applies
  # pending migrations, and where Atlas keeps its own migration-history
  # table (`atlas_schema_revisions`) recording which migrations have
  # already run.
  url = "postgres://postgres:132456@127.0.0.1:5432/ahlan-commerce?sslmode=disable"

  # A scratch database Atlas is free to create, wipe, and recreate at
  # will. `migrate diff` needs somewhere to actually build the "current
  # migrations applied so far" state so it can compute an accurate diff
  # against `src` - it does this in `dev`, never in `url`, so computing a
  # diff never touches real data.
  dev = "postgres://postgres:132456@127.0.0.1:5432/ahlan-commerce_atlas_dev?sslmode=disable"

  migration {
    # Where generated migration files (and atlas.sum, the checksum file
    # covering them) live.
    dir = "file://db/migrations"
  }
}
