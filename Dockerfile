#
# === Development Stage ===
FROM rust:latest AS dev

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create user for VS Code
RUN useradd -m -s /bin/bash vscode
USER vscode
WORKDIR /workspace


#
# === Build Stage ===
FROM rust:latest AS build

WORKDIR /workspace
COPY . .
RUN cargo build --release

#
# === Production Stage ===
FROM debian:bullseye-slim AS prod

# Install runtime dependencies (if needed)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary from dev stage
COPY --from=build /workspace/target/release/crypto /usr/local/bin/crypto

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/crypto"]