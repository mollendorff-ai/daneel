# DANEEL - Core Cognitive Loop
# Multi-stage build for minimal runtime image
#
# Build: docker build -t timmy-daneel .
# Size: ~50MB (static musl binary + fastembed model)

# =============================================================================
# Stage 1: Build environment
# =============================================================================
FROM rust:1.83-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig \
    perl \
    make

WORKDIR /app

# Copy manifests first (dependency caching)
COPY Cargo.toml Cargo.lock ./

# Create dummy src for dependency build
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer is cached)
ENV OPENSSL_STATIC=1
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include
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
FROM alpine:3.21

# Install runtime dependencies (CA certs for HTTPS, curl for healthcheck)
RUN apk add --no-cache ca-certificates curl

# Create non-root user
RUN addgroup -S daneel && adduser -S daneel -G daneel

WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/daneel /app/daneel

# fastembed cache directory (mounted as volume)
RUN mkdir -p /root/.cache/fastembed

# Expose injection API port
EXPOSE 3001

# Run as root (fastembed needs write access to cache)
# In production, consider pre-downloading model in build stage
ENTRYPOINT ["/app/daneel"]
