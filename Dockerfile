# DANEEL - Core Cognitive Loop
# Multi-stage build: compiles inside Docker for correct architecture
#
# Usage: docker build -t daneel .

# === Build Stage ===
# Trixie (testing) has GCC 14+ needed by ort-sys 2.x pre-built ONNX Runtime
FROM debian:trixie AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Rust via rustup (gets latest stable)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /build

# Copy manifests and source
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build
RUN cargo build --release

# === Runtime Stage ===
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/daneel /app/daneel

# fastembed cache directory (mounted as volume)
RUN mkdir -p /root/.cache/fastembed

EXPOSE 3001

ENTRYPOINT ["/app/daneel"]
