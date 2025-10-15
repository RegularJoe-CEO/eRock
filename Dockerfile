# ---- Build stage ----
FROM rust:1-bookworm AS builder
WORKDIR /app
# Copy everything (simple + reliable for workspace builds)
COPY . .
# Compile release for the edge crate
RUN cargo build --manifest-path edge/Cargo.toml --release --locked

# ---- Runtime stage ----
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates tzdata \
 && rm -rf /var/lib/apt/lists/*
# Non-root user
RUN useradd -m -u 10001 appuser
WORKDIR /app
# Copy the optimized binary
COPY --from=builder /app/edge/target/release/erock_edge /usr/local/bin/erock_edge
ENV RUST_LOG=info
EXPOSE 8080
USER appuser
HEALTHCHECK --interval=30s --timeout=2s --start-period=10s \
  CMD wget -qO- http://127.0.0.1:8080/health | grep '"status":"ok"' || exit 1
ENTRYPOINT ["/usr/local/bin/erock_edge"]
