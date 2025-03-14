FROM rust:1.85-alpine AS builder

# Add build metadata
ARG TARGETPLATFORM
ARG TARGETARCH

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev pkgconfig

# Create a new empty shell project
WORKDIR /usr/src/rustdok
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/rustdok*

# Now copy only the necessary source code
COPY src ./src

# Create an empty tests.rs file if it doesn't exist
# This is only needed for the build process and won't be included in the final image
RUN if [ ! -f src/tests.rs ]; then touch src/tests.rs; fi

# Build the application
RUN cargo build --release

# Create a minimal runtime image
FROM alpine:3.21.3 AS runtime

# Install runtime dependencies
RUN apk add --no-cache ca-certificates libgcc

# Copy only the binary from the builder stage
WORKDIR /app
COPY --from=builder /usr/src/rustdok/target/release/rustdok-server .

# Set environment variables
ENV RUST_LOG=info

# Expose the port the app runs on
EXPOSE 8080

# Run the binary
CMD ["./rustdok-server"] 