# ============================================
# Stage 1: Builder
# ============================================
FROM rust:1.88-bookworm AS builder

# Set working directory
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock* ./

# Create dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src && \
    rm -f target/release/konachan-api target/release/deps/konachan_api*

# Copy actual source code
COPY src ./src

# Force recompilation of the actual source code
RUN touch src/main.rs && cargo build --release

# ============================================
# Stage 2: Runner
# ============================================
FROM debian:bookworm-slim AS runner

# Set working directory
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/konachan-api /app/konachan-api

# Copy Rocket configuration
COPY Rocket.toml /app/Rocket.toml

# Create non-root user for security
RUN useradd -r -s /bin/false appuser && \
    chown -R appuser:appuser /app

USER appuser

# Expose the default Rocket port
EXPOSE 8000

# Set environment variables
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV ROCKET_LOG_LEVEL=normal

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8000/ || exit 1

# Run the application
CMD ["./konachan-api"]
