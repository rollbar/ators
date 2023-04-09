# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/ators

# Copy
COPY Cargo.toml Cargo.lock ./
COPY ators/Cargo.toml ./ators/
COPY ators/src ./ators/src

COPY atorsl/Cargo.toml ./atorsl/
COPY atorsl/build.rs ./atorsl
COPY atorsl/src ./atorsl/src

COPY fixtures/rollbar ./

# Build the application
RUN cargo build --release

# Create a new lightweight image for running the application
FROM debian:stable-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder
COPY --from=builder /usr/src/ators/target/release/ators /usr/local/bin
COPY --from=builder /usr/src/ators/rollbar /usr/src

# Set the entrypoint to run your app
ENTRYPOINT ["ators"]
