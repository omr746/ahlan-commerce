# Tasks: Compatibility Adapter for External Product Import

Ordered. Each task is independently verifiable — you can confirm it's done
without needing a later task to exist yet.

---

## Task 1 — Define the external payload DTO

**Do:** In `apps/api/src/compat/mod.rs` (new module), define a struct
representing the external payload's exact field shape (per the PRD's
resolved Open Question on the real external schema), with `Deserialize`.

**Expected file/output:** `apps/api/src/compat/mod.rs` containing one new
public struct, e.g. `ExternalProductPayload`.

**Verify:** `cargo build -p api` compiles with the new module registered
in `apps/api/src/lib.rs` (`pub mod compat;`), producing no output beyond a
successful build — nothing calls this struct yet.

---

## Task 2 — Write the pure translation function

**Do:** Add `fn translate(payload: ExternalProductPayload) -> Result<ProductCreateRequest, CompatError>`
in the same module. No I/O, no `AppState`, no database — a pure function,
matching the plan's Phase 1.

**Expected file/output:** `translate()` function plus a `CompatError` enum
covering: malformed/missing required field, unmappable price format,
invalid handle characters.

**Verify:** Unit tests in the same file (`#[cfg(test)] mod tests`)
covering every edge case listed in the PRD:
- valid payload → `Ok(ProductCreateRequest)` with every field correctly mapped
- missing required external field → `Err`
- unmappable price shape → `Err`
- extra/unknown external fields present → still `Ok` (ignored, not rejected)
- absent description vs. empty-string description → map to `None` vs `Some("")` respectively

Run: `cargo test -p api compat::` — all pass, no database needed.

---

## Task 3 — Wire translation to native create

**Do:** Add a function that calls `translate()` then
`state.catalog.create_product(...)`, reusing `state.ids` and `state.clock`
exactly as the native handler does.

**Expected file/output:** A function in `apps/api/src/compat/mod.rs`, e.g.
`async fn import_external_product(state: &AppState, payload: ExternalProductPayload) -> Result<Product, AppError>`.

**Verify:** A `#[tokio::test]` (or equivalent used elsewhere in this repo)
against a real migrated database confirming: a valid external payload
results in a row in `products` with the expected native field values, and
a duplicate handle produces the same `CatalogDbError::DuplicateHandle`
outcome the native path produces. Run with `make redis-health` and a
migrated `make migrate` database up.

---

## Task 4 — Expose the HTTP entry point

**Do:** Add `POST /api/compat/products`, calling Task 3's function and
returning `ProductResponse` (native shape) on success, `AppError` on
failure — same response types the native `create_product` handler uses.

**Expected file/output:** A new handler + route constant, registered in
`create_router` alongside the existing routes.

**Verify:**
```bash
curl -X POST http://127.0.0.1:3000/api/compat/products \
  -H 'content-type: application/json' \
  -d '<a real external-shaped payload>'
```
returns `201` with a `ProductResponse` body, and the product is visible at
`GET /api/products`. Re-running the same request returns the same
duplicate-handle error the native `POST /api/products` would.

---

## Task 5 — Re-map errors to external field names

**Do:** Where `translate()` or the native validator reports a field-level
error, ensure the error message names the *external* field the caller
actually sent, per the ADR's negative consequence.

**Expected file/output:** Error messages in `CompatError`'s `Display`
impl (or wherever `AppError::validation` is constructed for this path)
reference external field names, not `ProductCreateRequest`'s internal ones.

**Verify:** A unit test asserting that submitting an external payload
missing its price-equivalent field produces an error message naming that
external field, not `price_cents`.

---

## Task 6 — Document the new route

**Do:** Add a `#[utoipa::path(...)]` annotation to the new handler,
register it in `openapi::documented_router()`, and add a short section to
`docs/api.md` linking to it.

**Expected file/output:** Updated `apps/api/src/openapi.rs`,
`docs/generated/openapi.json` (regenerated), `docs/api.md`.

**Verify:**
```bash
make docs-api
make docs-api-check    # passes -- artifact matches current shape
```
and `/docs/scalar` shows the new endpoint.

---

## Explicitly out of scope for these tasks

- Any change to the native `POST /api/products` or GraphQL `productCreate`.
- Deployment or CI/CD changes beyond the existing `make docs-api-check`.
- A second external shape/adapter.