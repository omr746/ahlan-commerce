# Guard Review

## clean-code-guard findings

Finding: `apps/api/src/compat/external_product.rs:106` — Correctness / Boundary safety: Unchecked integer multiplication and addition on untrusted external input [`Ok(whole * 100 + frac_cents)`].
Decision: rejected
Reason: Product price will not exceed maximum integer limits in practice; domain constraints and upstream pricing ensure reasonable price bounds.
Follow-up: None. Retain current arithmetic.

Finding: `apps/api/src/compat/external_product.rs:81-107` — Rule 2 (Function size & abstraction levels): `parse_price_to_cents` exceeds the 20-line ceiling (27 lines) and mixes string validation, slice matching, and scaling arithmetic [`fn parse_price_to_cents(price: &str) -> Result<i64, AdapterError>`].
Decision: rejected
Reason: The function is a 27-line private, self-contained helper with low cyclomatic complexity and shallow nesting (≤2). Splitting it adds unnecessary indirection without improving readability or reusability (KISS/YAGNI).
Follow-up: None. Keep `parse_price_to_cents` as a cohesive, private parsing function.

Finding: `apps/api/src/compat/external_product.rs:57` — Idiomatic Rust conversions: `map_external_to_native` is implemented as an ad-hoc function rather than implementing the standard `TryFrom<&ExternalProduct>` trait [`pub fn map_external_to_native(external: &ExternalProduct) -> Result<ProductCreateRequest, AdapterError>`].
Decision: rejected
Reason: `map_external_to_native` was specified in Task 16.2 and is already tested across integration tests. Implementing `TryFrom` adds trait boilerplate without any current caller or generic abstraction requiring trait-based dispatch.
Follow-up: None. Retain `pub fn map_external_to_native` as the explicit entry point.

## test-guard findings

### apps/api/tests/adapter_external_product_test.rs

Finding: `apps/api/tests/adapter_external_product_test.rs::description_is_always_none_since_the_external_shape_has_no_such_field` — Rule 4 violation: Duplicate test that duplicates `external_id_has_no_native_equivalent_and_is_dropped`.
Decision: accepted
Reason: Both test functions execute identical logic: `load_fixture()`, call `map_external_to_native(&external)`, and assert `assert_eq!(native.description, None);`. Retaining two identical tests adds maintenance cost and redundant disk I/O without any distinct coverage (violates Rule 4).
Follow-up: Delete `description_is_always_none_since_the_external_shape_has_no_such_field`, and ensure `assert_eq!(native.description, None);` is asserted in `maps_every_field_from_the_committed_fixture`.

Finding: `apps/api/tests/adapter_external_product_test.rs::external_id_has_no_native_equivalent_and_is_dropped` — Rule 1 violation: Purports to test that `external_id` is dropped, but actually asserts `native.description == None`.
Decision: accepted
Reason: The test asserts `native.description` rather than proving that `external_id` is ignored (violates Rule 1). To test observable behavior rather than compiler types, the test should prove that changing `external_id` does not alter the mapped output.
Follow-up: Update the test body to mutate `external.external_id = "completely-different-id".to_string()` and assert that the resulting `ProductCreateRequest` is strictly equal to the result from mapping the unmodified fixture.

Finding: `apps/api/tests/adapter_external_product_test.rs::a_whole_number_external_price_maps_correctly` — Rule 3 violation: Field variants (`price = "30"`, `is_visible = false`) tested across separate functions re-reading fixture JSON from disk instead of a data-driven test.
Decision: rejected
Reason: Test Guard Rule 3 explicitly allows separate tests for genuinely different scenarios with distinct assertions (error path vs boolean mapping vs integer cents mapping). In standard Rust test suites without extra macro dependencies (such as `rstest`), separate small `#[test]` functions provide isolated, descriptive failure reporting.
Follow-up: None. Retain the separate scenario tests.

### apps/api/tests/native_product_create_test.rs

Finding: `apps/api/tests/native_product_create_test.rs::native_validation_rejects_*` — Rule 3 violation: Five separate test functions mutate a single field and assert `assert!(request.validate().is_err())`.
Decision: rejected
Reason: Having separate named tests for distinct domain constraints (empty title, empty handle, invalid handle characters, negative price, negative inventory) produces clear output in `cargo test` indicating exactly which domain requirement failed. Consolidating them into a single loop stops on the first failure and obscures test identity.
Follow-up: None. Retain individual test functions per validation boundary condition.

Finding: `apps/api/tests/native_product_create_test.rs::native_validation_rejects_an_empty_title` (and sibling rejection tests) — Rule 1 violation: Tests assert only `is_err()` rather than verifying the exact validation error message.
Decision: accepted
Reason: `ProductCreateRequest::validate()` returns `Result<(), String>`. Asserting only `is_err()` allows a test to pass for the wrong reason (e.g. if title validation passed but handle validation unexpectedly rejected). Asserting the returned error string confirms the exact contract (violates Rule 1).
Follow-up: Update each rejection test in `native_product_create_test.rs` to assert the specific returned error message (`Err("Product title is required.".to_string())`, etc.).

## docs-guard findings

### docs/api.md (and docs/written/api.md)

Finding: `docs/api.md` vs `docs/written/api.md` — Rule 1 / Rule 10 violation: File path mismatch between documentation references (`docs/api.md`) and disk location (`docs/written/api.md`).
Decision: needs mentor review
Reason: All primary prose documentation was organized under `docs/written/` (`architecture.md`, `commands.md`, `operations.md`, `setup.md`, `api.md`) with internal sibling relative links (e.g. `api.md` links directly to `[architecture.md](architecture.md)`). Moving `api.md` to root `docs/` would break sibling links, while keeping it in `docs/written/` conflicts with `commands.md` and spec references to `docs/api.md`.
Follow-up: Ask mentor whether the canonical location is `docs/written/api.md` (updating references in `commands.md` and specs) or if `api.md` should be moved to `docs/api.md` with updated relative links.

Finding: `docs/written/api.md:8-9` — Rule 10 violation: Broken relative links pointing to `generated/openapi.json` and `generated/schema.graphql`.
Decision: accepted
Reason: In `docs/written/api.md`, the relative links `generated/openapi.json` and `generated/schema.graphql` resolve to `docs/written/generated/...`, which does not exist, causing 404 broken links (violates Rule 10).
Follow-up: Change the markdown links in `docs/written/api.md` to `../generated/openapi.json` and `../generated/schema.graphql`.

Finding: `docs/written/api.md:57` — Rule 7 violation: Leftover prompt/task scaffolding `# Append this section to the end of your existing docs/api.md`.
Decision: accepted
Reason: Line 57 is verbatim copy-paste scaffolding from a task instruction that was accidentally committed into documentation text (violates Rule 7).
Follow-up: Delete line 57 from `docs/written/api.md`.

### README.md

Finding: `README.md:8-14` — Rule 1 violation: Folder structure describes workspace `product-api/` with only 2 crates, missing 4 crates and using obsolete repo folder name.
Decision: rejected
Reason: Lines 8–14 are under the explicit section header `## Folder shape (Task 03.1)` in `README.md`. The README was written chronologically across chapters to document learning steps. Mutating historical chapter logs would misrepresent the state at Chapter 03. Current workspace architecture is maintained in `docs/written/architecture.md`.
Follow-up: None. Leave historical chapter documentation intact.

Finding: `README.md:100` — Rule 1 / Rule 3 violation: `curl -s http://127.0.0.1:3000/api/products/018f5a3e-0000-7000-8000-0000000000ff` claims `GET /api/products/{id}` returns `not_found - 404`, but returns `405 Method Not Allowed`.
Decision: needs mentor review
Reason: Line 100 documents `GET /api/products/{id}` under Chapter 03A, but currently only `PATCH /api/products/{id}` is registered on that route in Axum. Adding a new `GET` endpoint is an unapproved feature addition, but leaving the curl example broken in the README misleads developers testing the API.
Follow-up: Request mentor guidance on whether `GET /api/products/{id}` should be implemented in `apps/api/src/handlers.rs` or if the README example should be updated/noted.

Finding: `README.md:214-220` and `README.md:277` — Rule 2 violation: Database URL claims `postgres://postgres:postgres@127.0.0.1:5432/ahlan_commerce`, contradicting `config.rs` and `docker-compose.yml`.
Decision: accepted
Reason: `docker-compose.yml` configures password `132456` and database name `ahlan-commerce` (hyphen). Running the commands in `README.md` fails authentication, and the claim on line 219 that `config.rs` defaults to `postgres:postgres` is demonstrably false (violates Rules 1 and 2).
Follow-up: Correct connection strings and defaults in `README.md` lines 216, 219, and 277 to `postgres://postgres:132456@127.0.0.1:5432/ahlan-commerce`.

Finding: `README.md:589` — Rule 6 violation: `README.md` terminates at Chapter 07, omitting Chapters 08 through 16.
Decision: rejected
Reason: Starting in Chapter 14/15, all detailed operational and architectural documentation was moved into dedicated documents under `docs/written/` (`architecture.md`, `setup.md`, `commands.md`, `operations.md`, `api.md`). Demanding that 9 additional chapters be backfilled into `README.md` duplicates existing documentation and bloats the top-level README.
Follow-up: None.
