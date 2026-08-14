# Axum Runtime Model

## Overview

Axum is an asynchronous Rust web framework built around the Tokio ecosystem.

It combines several libraries to handle HTTP requests, execute asynchronous handlers, and provide middleware.

The main components are:

- **Tokio** — asynchronous runtime that executes and schedules tasks.
- **Hyper** — HTTP implementation that handles HTTP connections and requests.
- **Tower** — service and middleware abstraction.
- **Tower HTTP** — HTTP-specific middleware such as tracing, CORS, timeouts, and compression.
- **Axum** — routing, handlers, extractors, state management, and responses.

A simplified architecture looks like this:

```text
                        Client
                          │
                          │ HTTP Request
                          ▼
                       Hyper
                          │
                          ▼
                    Axum Router
                          │
                          ▼
                  Tower Middleware
                          │
             ┌────────────┼────────────┐
             │            │            │
          Tracing        CORS     Compression
             │            │            │
             └────────────┼────────────┘
                          │
                          ▼
                    Axum Handler
                          │
                          ▼
                       Response
                          │
                          ▼
                       Hyper
                          │
                          ▼
                        Client




                  Axum                  Actix Web
                  ────                  ─────────
Runtime           Tokio                 Tokio
HTTP              Hyper                 Actix HTTP
Middleware        Tower                 Actix middleware
Execution         Tokio tasks            Multiple workers
State             Explicit sharing       Per-worker application