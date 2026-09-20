# Chapter 10 Frontend Client Choice

## Decision

The Ahlan admin app (`apps/admin`) uses:

- **TanStack Router** — owns URL and screen state (`/products`, future `/products/:id`, etc.)
- **TanStack Query** — owns server state: loading, error, success, caching, retries, and invalidation
- **A deliberately thin GraphQL request wrapper** (`src/api/graphql-client.ts`) — owns only the transport call (POST the query, parse JSON, throw on errors)

Apollo Client is **not** used in the default implementation.

## Why this split, not a bigger client

The learning goal for this chapter is that a candidate can point at the code and say, precisely:

- "This line is where the query starts loading."
- "This line is where the cache gets invalidated after a write."
- "This line is where the HTTP request actually goes out."

A single larger GraphQL client (Apollo) collapses these three concerns into one library with its own cache model, its own normalization rules, and its own set of hooks. That's a legitimate choice for a production app, but it hides the transport/cache/URL boundary that this exercise exists to teach. If the wrapper and the cache are the same object, "where does server state live" becomes a harder question to answer, not an easier one.

Keeping the wrapper thin also means there is exactly **one** server-state cache owner in this codebase: TanStack Query. Mixing Apollo's cache and TanStack Query's cache in one small exercise would require explaining why two caches exist and how they don't fight each other — a real and useful lesson, but not this one.

## Tradeoffs

| Choice | Pros | Cons | Better for Ahlan |
|---|---|---|---|
| Thin GraphQL wrapper + TanStack Query | Clear cache ownership, low cognitive load | Fewer GraphQL-specific client features (no built-in normalized cache, no subscriptions) | **Required** |
| Apollo Client | Rich GraphQL client: cache, devtools, subscriptions | More concepts, a second cache model to explain alongside TanStack Query | Optional mentor extension only |

## Mentor extension (not implemented here)

A mentor who wants stronger frontend parity with a production stack can, **after** the default Task 10.1–10.4 flow passes, add a separate extension that:

1. Replaces `src/api/graphql-client.ts` and the raw query/mutation strings in `src/api/products.ts` with an Apollo `ApolloClient` instance and generated hooks.
2. Keeps the same product list/create behavior and the same `/products` route.
3. Is done as its own diff, reviewed separately from the required Ahlan path, so the two cache-ownership models are never live in the same small exercise at once.

This keeps the required path teaching one clear idea, while leaving room for a mentor to show the "bigger client" tradeoff explicitly, on purpose, as a follow-up rather than a default.
