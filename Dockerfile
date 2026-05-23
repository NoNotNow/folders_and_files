# Build stage
FROM rust:1.80 as builder

WORKDIR /usr/src/app
COPY . .

# Build the application
RUN cargo build --release

# Final stage
FROM ubuntu:24.04

# Install necessary libraries (like libssl if needed, but for std::fs it might not be)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/files_and_folders /usr/local/bin/files_and_folders

# Set the entrypoint
ENTRYPOINT ["files_and_folders"]
