# Use official Rust image
FROM rust:1.75-slim-bullseye AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    libmysqlclient-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy project files
COPY . .

# Build the project
RUN cargo build --release

# Final stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    libmysqlclient21 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy built binary from builder stage
COPY --from=builder /app/target/release/slack-attendance-backend /usr/local/bin/

# Copy .env and other necessary files
COPY .env /app/.env
COPY migrations /app/migrations

# Set working directory
WORKDIR /app

# Expose port
EXPOSE 8080

# Set environment
ENV RUST_LOG=info

# Run the application
CMD ["slack-attendance-backend"]
