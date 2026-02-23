# Build stage
FROM rust:1.70-alpine as builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev pkgconfig

WORKDIR /app

# Copy cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build the application
RUN cargo build --release

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache openssl ca-certificates

# Create non-root user
RUN addgroup -S mysystem && adduser -S mysystem -G mysystem

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/mysystem-backend /app/mysystem-backend
COPY --from=builder /app/migrations /app/migrations

# Set permissions
RUN chown -R mysystem:mysystem /app

USER mysystem

# Expose port
EXPOSE 3000

# Environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1
ENV DATABASE_URL="mysql://mysystem_user:mysystem_password@mysql:3306/mysystem"

# Run the application
CMD ["./mysystem-backend"]