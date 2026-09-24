# syntax=docker/dockerfile:1

FROM rust:1.94-slim AS build
WORKDIR /app

COPY . .

RUN cargo build --release -p api -p import-worker -p migrator

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=build /app/target/release/api /app/api
COPY --from=build /app/target/release/import-worker /app/import-worker
COPY --from=build /app/target/release/migrator /app/migrator

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -sf http://127.0.0.1:3000/health || exit 1

# No CMD.
# API:      /app/api
# Worker:   /app/import-worker
# Migrator: /app/migrator   (one-off, run before API/worker start — see deployment-prep.md)