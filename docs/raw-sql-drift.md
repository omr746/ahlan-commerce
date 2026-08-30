# Feeling Raw SQL Drift

Two small additions to the product API, on purpose: a query that returns
only published products, and an update path that touches `description`
and `published_at`. Nothing about the *schema* changed - Atlas has
nothing to do with either of these. Everything below is what it actually
took to add them by hand, including a real bug hit while doing it.

## Which SQL strings had to change?

Three, and one of them was a mistake found only by writing a test:

1. **A new `select`** (`list_published_products`) - a near-total copy of
   `list_products`'s column list and row mapping, plus `where published =
   true`. Copy-pasted, not derived from anything - nothing keeps this
   column list in sync with `list_products`'s if either one changes later.

2. **A new `update ... returning`** (`update_product`) - the trickiest
   one. `published_at` needed a `case` expression (set it on first
   publish, leave it alone on a later edit, clear it on unpublish) rather
   than a plain assignment, computed in one round trip so no other
   request could change the row between "read the current value" and
   "write the new one."

3. **The existing `insert` in `create_product` had to change too**,
   unexpectedly. It used to build the returned `Product` directly from
   the Rust values that went into the `insert` (id, computed timestamp,
   etc.) instead of reading back what Postgres actually stored. That
   *compiled fine and had been passing tests since Chapter 04.1* - it
   only broke once `update_product`'s test compared a product's
   `published_at` from its `create_product` response against the same
   product's `published_at` after an unrelated edit, and the two values
   - both valid `DateTime<Utc>`, both "correct" by their own type - were
   not equal:

   ```
   assertion `left == right` failed
     left: Some(2026-08-26T18:34:19.759234Z)
     right: Some(2026-08-26T18:34:19.759234509Z)
   ```

   `clock.now()` carries nanosecond precision; Postgres's `timestamptz`
   only stores microseconds. `create_product` had been silently returning
   a *more precise* timestamp than what was actually sitting in the row -
   for four chapters, undetected, because nothing before this compared
   the two. Fixed by making `create_product` use `returning` too, so both
   write paths return exactly what got persisted, never what Rust
   computed before insert.

## Which Rust structs had to change?

- **New:** `catalog::ProductUpdate` (domain) and
  `apps/api::ProductUpdateRequest` (DTO) - deliberately narrow, just
  `description` + `published`, not a general "patch anything" shape.
- **Unchanged:** `Product` and `ProductResponse` - both already had
  `description`/`published_at` from Chapter 04.2. This task added new
  *ways to read and write* those fields; it didn't need to touch the
  shapes themselves. That's the split from Chapter 03/04.2 paying off
  again - a new query or write path against existing columns is
  contained to `postgres.rs` and one new DTO, not a ripple through every
  struct that mentions a product.
- **In-memory `Catalog`** also grew `list_published_products` and
  `update_product`, purely so the domain-level unit tests (no DB needed)
  could cover the same `published_at` rules that `PgCatalog`'s SQL
  encodes - two independent implementations of the same three rules
  (set/preserve/clear), which is its own small drift risk: nothing
  forces them to agree except a human keeping both updated.

## What could compile while still being wrong?

The `create_product` precision bug above is the real example, but it's
one instance of a general shape of bug this file's whole approach is
exposed to:

- **Reordering `.bind()` calls.** `.bind(&input.description)` and
  `.bind(input.published)` are two different types, so swapping their
  *positions* in the `.bind()` chain relative to the SQL's `$1`/`$2`
  would compile - `sqlx::query(&str)` has no link between a Rust value's
  binding position and which named column the SQL placeholder is
  actually for. A `$1`/`$2` swap in `update_product`'s `set description =
  $1, published = $2` versus the `.bind()` order is invisible to the
  compiler; it would silently write the title into the description
  column or similar, correctly typed the whole way.
- **A wrong `case` branch.** If `update_product`'s `case` had instead
  been `when $2 = true then $3` (no `and published_at is null` check),
  every edit to an already-published product would silently reset its
  publish timestamp to "now." Compiles. Returns 200. The bug the tests
  above exist specifically to catch (`update_does_not_reset_published_at_
  on_a_later_edit`) - and it would have shipped without that test.
- **A missing `where`.** If `list_published_products` had been
  `list_products`'s SQL with the column list re-typed by hand but the
  `where published = true` forgotten, the function's *name* would say
  "published only," its return type would be identical, and it would
  compile and run without error - it just wouldn't filter anything.

None of these are hypothetical dangers this project happened to avoid -
the `create_product` timestamp mismatch is exactly this shape of bug,
found only because a test happened to compare the right two values.

## What could leak to production if a query was missed?

The concrete risk with `list_published_products` specifically: this is
almost certainly the query a public storefront or an unauthenticated
search index would call - "show customers what's for sale." If its
`where published = true` were missing, wrong, or silently dropped in a
future edit (e.g. someone "simplifies" it back toward `list_products`'s
query during a refactor and forgets why the extra clause was there),
**draft, embargoed, or otherwise-not-ready products would be visible to
the public** - through an endpoint whose entire job was supposed to be
keeping them hidden. No error, no 500, no log line calling this out - a
200 with the wrong rows in it, same failure shape as Chapter 04.2's
"old code silently drops new data" demonstration, just on the read side
instead of the write side this time.

The `update_product` `published_at` bug (if shipped) is lower-severity
but still real: any feature relying on "when was this actually first
published" (a public changelog, an RSS/sitemap ordered by publish date,
"new this week" sections) would silently show wrong information after
the first edit to any product - not a security leak, but a quiet,
ongoing correctness bug with no error to notice it by.

## Why didn't Atlas catch missed Rust query updates?

Because none of this was ever Atlas's job. Atlas's entire scope (Chapter
04.3) is the *shape of the table* - what columns exist, what type they
are, what constraints apply - tracked as migration history so a database
provably has (or doesn't have) a given structure. Every bug in this
document is about *query correctness*: whether the SQL this code sends
asks Postgres the right question. Both `description` and `published_at`
already existed as columns before this task started; nothing here added,
removed, or altered a column, so Atlas had literally nothing to diff and
nothing to apply. A missing `where` clause, a swapped `.bind()` position,
a `case` expression with the wrong condition - all still perfectly valid
SQL against a perfectly correct, Atlas-verified schema. Atlas can
guarantee the table exists in the shape you declared; it was never going
to check whether the twelfth query written against that table asks the
question its author meant to ask. That gap - real, and demonstrated for
real above - is exactly what a typed query layer (checked against the
schema at compile time, not just at the DDL level) would close, and is
worth its own later chapter rather than a claim this one can make for it.
