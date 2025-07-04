# Build stage
FROM rust:1.75-slim-bookworm as builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential git && \
    rm -rf /var/lib/apt/lists/*

# Copy the entire workspace
COPY . .

# Build the api
RUN cargo build --release -p api

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from builder
COPY --from=builder /usr/src/app/target/release/api /app/api

# Run the processor
CMD ["/app/api"]
