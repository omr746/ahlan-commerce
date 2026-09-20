# ADR: Compatibility Adapter Architecture

**Status:** Proposed — awaiting mentor approval
**Depends on:** `specs/compatibility-prd.md` (must be approved first)

## Context

The approved PRD calls for accepting one external product payload shape
and creating a native Ahlan product from it. There are several ways to
structure where the "external shape" knowledge lives in the codebase, and
they have materially different long-term costs. This ADR is about *where
the translation lives and how much of the external shape the rest of the
system is allowed to know about* — not about the specific field mapping,
which is a plan/tasks-level concern once this is approved.

## Decision

**Native API plus a thin adapter.** The adapter is a single, isolated
translation layer: external payload in, native `ProductCreate` out, then
it calls the *exact same* `Catalog::create_product` function the native
REST and GraphQL create paths already call. No other code in `catalog`,
`catalog-db`, or the native handlers is aware that an external shape
exists.

Concretely: one new module (e.g. `apps/api/src/compat/product_import.rs`),
one new DTO representing the external shape, one `From`/`TryFrom`
conversion into `ProductCreateRequest` (or directly into `ProductCreate`),
and one thin handler/entry point that calls the conversion and then the
existing create path.

## Considered Alternatives

### 1. Public API clone
Stand up a second, parallel product-create implementation that mimics the
external system's own API surface end-to-end (its own validation, its own
response shape, its own error format), independent of Ahlan's native path.

**Rejected because:** this duplicates every validation and creation rule
that already exists in `catalog`, guaranteeing the two paths drift the
first time either one changes. It also means a product created via the
"external-compatible" surface could end up validated differently than one
created natively — directly undermining the PRD's goal that native
create behavior stays the single source of truth.

### 2. Raw passthrough
Accept the external payload and forward it with minimal or no translation
directly into product creation, coercing types loosely at the point of
use (e.g. `serde_json::Value` all the way through).

**Rejected because:** this pushes translation and validation
responsibility into the create path itself, which then has to understand
two different shapes' worth of "what does published even mean here."
It also makes the acceptance criterion "same validation errors as the
native path" much harder to guarantee, since a loosely-typed passthrough
tends to produce its own ad hoc error messages at the point of failure
rather than reusing `ProductCreateRequest::validate()`.

### 3. Native handlers that understand external shapes
Modify the existing native `create_product` handler (REST and/or GraphQL)
to detect and accept either shape directly, branching internally on which
one it received.

**Rejected because:** this permanently couples the native API's own code
to a specific external system's shape. Every future external integration
would either need its own branch bolted onto the same handler, or its own
adapter anyway — at which point the branching in the native handler was
pure added complexity with no benefit. It also violates the Non-Goal in
the PRD that native REST/GraphQL create endpoints are not to be changed.

## Consequences

**Positive**
- Native create behavior (validation, duplicate-handle handling, ID/clock
  boundaries) is reused verbatim — zero duplicated business logic.
- The external shape is isolated to one module. Adding a second external
  integration later means adding a second adapter module, not touching
  this one or the native path.
- Adapter-level bugs (a bad field mapping) can't corrupt native data
  integrity, since every payload still passes through the same validation
  the native path enforces.

**Negative**
- Two shapes now exist in the codebase's mental model (native and
  external), which is one more thing for a new contributor to learn,
  even though only one lives in the adapter module.
- If the external system's shape changes, the adapter needs a matching
  update — this is an ongoing maintenance surface, not a one-time cost.
- The adapter's own error handling has to actively re-map native
  validation errors so they read sensibly against the *external* field
  names (e.g. a native "handle is required" error should reference
  whatever field the external payload calls it), or the acceptance
  criterion about consistent errors becomes technically true but
  practically confusing to the adapter's caller.

**Neutral**
- This ADR does not decide the exact external payload shape or the
  specific field mapping — that's Task 15.3's plan/tasks, once the PRD's
  open question about the real external shape is resolved.

## Approval

- [ ] Mentor has reviewed and approved this ADR.

**Per the process rules for this chapter: no implementation begins until
this ADR is approved here.**