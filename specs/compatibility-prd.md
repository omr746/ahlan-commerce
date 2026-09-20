# PRD: Compatibility Adapter for External Product Import

**Status:** Draft — awaiting mentor approval
**Author:** Omar Ahmed
**Related discussion:** "Add one compatibility adapter for importing an
external product payload into native product create behavior"

## Problem

Ahlan's product-create path (`ProductCreateRequest` → `Catalog::create_product`)
only accepts Ahlan's own field shape (`title`, `handle`, `price_cents`,
`inventory_quantity`, `published`, `description`). An external system that
wants to bring products into Ahlan today has no way to do that without a
human hand-remapping fields for every request, which doesn't scale and is
easy to get wrong.

## User Story

As an integration engineer connecting an external commerce platform to
Ahlan, I want to submit a product payload in the external platform's shape
and have it accepted and created as a native Ahlan product, so that I don't
have to hand-translate every field myself and so that Ahlan's own create
path stays the single source of truth for what a valid product is.

## Goals

- Accept one external product payload shape and translate it into a native
  `ProductCreate`.
- Reuse Ahlan's existing product-create behavior exactly — the same
  validation rules, the same duplicate-handle handling, the same
  `Clock`/`IdGenerator` boundaries — rather than reimplementing any of it.
- Make it obvious, from reading the adapter code alone, which fields map
  where and what happens to fields that don't map.

## Non-Goals / Out of Scope

- Supporting more than one external payload shape. This PRD is scoped to
  **one** adapter for **one** external shape.
- Two-way sync (exporting Ahlan products back to the external shape).
- Bulk/batch import of external payloads (Chapter 11's file-based import
  job is a separate, already-shipped mechanism; this is a single-payload
  adapter).
- Product update/publish via the external shape — create only.
- Authentication, webhooks, or any transport concern for how the external
  payload arrives. This PRD covers the translation only.
- Any change to `apps/api`'s existing native REST or GraphQL create
  endpoints. The adapter is additive.
- Deployment or infrastructure changes.

## Acceptance Criteria

1. Given a well-formed external product payload, when it is submitted to
   the adapter, then a native product is created via the same
   `Catalog::create_product` path the native API already uses, with the
   same validation and duplicate-handle rules applied.
2. Given an external payload missing a field required by Ahlan's native
   validation (Chapter 04 rules), when it is submitted, then the adapter
   returns the same class of validation error the native API returns for
   the same underlying problem — not a separate, adapter-specific error
   shape.
3. Given an external payload whose product handle already exists in Ahlan,
   when it is submitted, then the adapter returns the same duplicate-handle
   outcome the native create path already returns.
4. Given an external payload with fields Ahlan's native model has no place
   for, when it is submitted, then those fields are ignored and the import
   still succeeds — an adapter's job is to translate what maps, not to
   reject payloads for having extra data.
5. Given the external payload's price is not already integer cents (exact
   representation TBD — see Open Questions), when it is submitted, then
   the adapter converts it to `price_cents` using one documented,
   deterministic rule, not an approximation that could silently round
   money incorrectly.
6. The adapter introduces no new product-creation code path in `catalog`
   or `catalog-db` — it only translates a payload shape and then calls the
   existing native create function.

## Edge Cases

- External payload has a `published`-equivalent field with a different
  name, different type (e.g. a status string like `"active"`/`"draft"`
  instead of a boolean), or is missing entirely (needs a defined default).
- External payload's price field is a decimal string (`"25.00"`), a float,
  or already an integer — the adapter must pick one and document it.
- External payload's handle contains characters Ahlan's handle validation
  rejects (uppercase, spaces, underscores) — does the adapter normalize
  (e.g. lowercase + hyphenate) or reject as-is? Recommendation: reject
  as-is and surface the same validation error the native path would, to
  avoid silently creating a different handle than what the external system
  thinks it sent.
- External payload has no description field at all vs. an explicit empty
  string — both should map to Ahlan's `Option<String>` in a single
  consistent way (recommend: absent → `None`, empty string → `Some("")`,
  since these mean different things to the external system's author).
- External payload's inventory/price fields are negative or absurdly large
  — must hit the same bounds checks the native validator already enforces,
  not a separate check.
- Duplicate handle within the same batch, if this adapter is ever called
  in a loop by something upstream (e.g. Chapter 11's import worker) — out
  of scope for this PRD's single-payload adapter, but worth flagging since
  it's a plausible caller later.
- External payload is malformed JSON, or valid JSON but not shaped like
  the external contract at all (e.g. an empty object) — must fail with a
  clear, distinct error rather than a confusing validation message about
  fields the payload never had a chance to provide.

## Open Questions (must be resolved before ADR/plan)

- **What is the exact external payload shape?** This PRD does not commit
  to a concrete external schema yet — that needs either a real sample
  payload from the actual external system being integrated, or an explicit
  decision to model it on a well-known reference shape (e.g. a
  Shopify-style product JSON) for the purposes of this exercise. The ADR
  and plan cannot be finalized without this.
- What deterministic price-conversion rule should the adapter use if the
  external price is a decimal string?
- Does "compatibility adapter" mean one new endpoint (e.g.
  `POST /api/compat/products`), or a library function callable from
  multiple places? This PRD assumes a new endpoint for concreteness, but
  the ADR is where that's formally decided.

## Approval

- [ ] Mentor has reviewed and approved this PRD.
- [ ] Open questions above are resolved or explicitly deferred with a
      documented default.

**Per the process rules for this chapter: no ADR work and no implementation
work begins until this PRD is approved here.**