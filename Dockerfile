# ── Stage 1: Build backend ──────────────────────────────────
FROM rust:1.84-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY migration/Cargo.toml migration/Cargo.toml

# Create dummy src files so cargo can fetch & compile deps
RUN mkdir -p backend/src migration/src \
    && echo "fn main() {}" > backend/src/main.rs \
    && echo "fn main() {}" > migration/src/lib.rs \
    && cargo build --release --package backend 2>/dev/null || true

# Copy real sources and rebuild
COPY backend/ backend/
COPY migration/ migration/
RUN touch backend/src/main.rs && cargo build --release --package backend

# ── Stage 2: Runtime ────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /usr/local/bin/mistylog-backend

ENV RUST_LOG=info
EXPOSE 8090

CMD ["mistylog-backend"]
