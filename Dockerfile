# DANEEL - Core Cognitive Loop
# Runtime-only image (binary built externally via `make build`)
#
# Build binary first: make build
# Then: docker build -t timmy-daneel .

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy pre-built binary (built with `make build`)
COPY target/release/daneel /app/daneel

# fastembed cache directory (mounted as volume)
RUN mkdir -p /root/.cache/fastembed

EXPOSE 3001

ENTRYPOINT ["/app/daneel"]
