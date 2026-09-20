# Plan: Compatibility Adapter for External Product Import

**Depends on:** `specs/compatibility-prd.md` (approved),
`specs/compatibility-adr.md` (approved)

## Approach

Implement the "native API plus adapter" decision from the ADR as one new,
isolated module that sits in front of the existing native product-create
path. No existing native code changes shape; only new code is added.

## Architecture

External caller
|
v
apps/api/src/compat/product_import.rs <- NEW: this plan's entire surface
| 1. Deserialize the external payload shape
| 2. Translate external fields -> ProductCreateRequest
| 3. Re-map any translation-specific errors to read against
| external field names
|
v
ProductCreateRequest::validate() <- REUSED, unchanged
|
v
Catalog::create_product(...) <- REUSED, unchanged
|
v
Same native response shape (ProductResponse) or same native error shape


## Phases

**Phase 1 — External shape + translation (no HTTP surface yet)**
Define the external DTO and the pure translation function
(`external payload -> Result<ProductCreateRequest, CompatError>`), fully
unit-testable with no router, no database, no network.

**Phase 2 — Wire to native create**
Call `state.catalog.create_product(...)` with the translated input, reusing
`AppState` exactly as the native handler does.

**Phase 3 — Expose an entry point**
Add one new route (e.g. `POST /api/compat/products`) that does nothing but
call Phase 1 + Phase 2 in sequence and format the response.

**Phase 4 — Error re-mapping**
Ensure a validation failure on a translated field reports the *external*
field name, not the native one, per the ADR's negative consequence.

**Phase 5 — Docs**
Add the new route to the OpenAPI surface (Chapter 14 machinery — a new
`#[utoipa::path]` annotation, regenerate via `make docs-api`), and a short
section in `docs/api.md` linking to it.

## Explicitly deferred (per "Do Not Add Yet")

- Any code before the PRD is approved.
- The ADR before the PRD is reviewed.
- Any deployment/infrastructure change.
- Guard-skills or automated review tooling — nothing to review yet.

## Verification strategy

Each phase produces something independently checkable without needing the
next phase to exist yet (see `tasks.md` for the exact command per task):
unit tests for translation logic (Phase 1) before any HTTP wiring exists;
a curl-level check for the route (Phase 3) before docs are touched
(Phase 5).