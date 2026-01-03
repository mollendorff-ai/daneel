# DANEEL - Core Cognitive Loop
# Multi-stage build for minimal runtime image
#
# Build: docker build -t timmy-daneel .

# =============================================================================
# Stage 1: Build environment
# =============================================================================
FROM rust:1.85-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests first (dependency caching)
COPY Cargo.toml Cargo.lock ./

# Create dummy src for dependency build
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer is cached)
RUN cargo build --release && rm -rf src

# Copy actual source
COPY src ./src

# Build the real binary
RUN touch src/main.rs && cargo build --release

# Strip binary
RUN strip /app/target/release/daneel

# =============================================================================
# Stage 2: Runtime
# =============================================================================
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/daneel /app/daneel

# fastembed cache directory (mounted as volume)
RUN mkdir -p /root/.cache/fastembed

# Expose injection API port
EXPOSE 3001

ENTRYPOINT ["/app/daneel"]
