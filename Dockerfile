# ---------- builder ----------
FROM rust:latest AS builder
WORKDIR /usr/src/app

# Bring in Cargo manifest first to leverage Docker cache for dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy src so `cargo build` can be invoked to cache dependencies,
# only if you want an additional caching micro-optimization:
# (uncomment if you want dependency caching before copying your actual src)
# RUN mkdir src && echo "fn main() {println!(\"hello\");}" > src/main.rs && \
#     cargo build --release || true && rm -f target/release/deps/*

# Copy source
COPY ./src ./src
COPY ./templates ./templates

# Build optimized release binary (adjust target if needed)
RUN cargo build --release

# ---------- runtime ----------
FROM debian:bookworm-slim AS runtime
# (use bookworm-slim / bullseye-slim / ubuntu:22.04 as you prefer)
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from builder stage
COPY --from=builder /usr/src/app/target/release/crypto /app/crypto

# Use a non-root user (optional but recommended)
RUN useradd --no-create-home --uid 1000 appuser && chown appuser:appuser /app/crypto
USER appuser

ENV RUST_LOG=info
ENV RUST_BACKTRACE=1
ENV PORT=3000

EXPOSE 3000
CMD ["/app/crypto"]
