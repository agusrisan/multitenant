# Multi-stage Dockerfile for production deployment
# Stage 1: Frontend build
FROM node:20-alpine AS frontend
WORKDIR /app/resources

# Copy package files
COPY resources/package*.json ./

# Install dependencies
RUN npm install

# Copy frontend source
COPY resources/ ./

# Build frontend for production
RUN npm run build

# Stage 2: Backend build
FROM rust:1.75-slim AS backend
WORKDIR /app

# Install required system dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main to cache dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src/ ./src/
COPY migrations/ ./migrations/

# Build the actual application
RUN cargo build --release

# Stage 3: Runtime
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app

# Copy binary from backend stage
COPY --from=backend /app/target/release/multitenant /usr/local/bin/multitenant

# Copy frontend build from frontend stage
COPY --from=frontend /app/resources/dist /app/resources/dist

# Copy migrations
COPY --from=backend /app/migrations /app/migrations

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run the application
CMD ["multitenant"]
