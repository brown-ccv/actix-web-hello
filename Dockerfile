# Use the official Rust image as the base image
FROM rust:1.67.1 as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Create a new stage for the final image
FROM debian:buster-slim

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/actix-hello .

# Set the entrypoint to run the binary
ENTRYPOINT ["./actix-hello"]


