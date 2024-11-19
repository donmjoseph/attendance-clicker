# Use rust:alpine as the base image to have Cargo available in the runtime
FROM rust:alpine3.20 AS builder

# Set the working directory inside the container
WORKDIR /usr/src/interface

# Install dependencies
# openssl-dev and pkgconfig required for sqlx, I believe
RUN apk update --no-cache && \
    apk upgrade --no-cache && \
    apk add --no-cache build-base openssl-dev pkgconfig

# Copy the entire source code (this includes Cargo.toml, Cargo.lock, and source files)
COPY . .

# Fetch dependencies (this will now work since the source code is available)
RUN cargo fetch

# try to install openssl for rust to run successfully
RUN apk add --no-cache openssl

# Expose the necessary port
EXPOSE 3000

# Command to start the application
CMD ["cargo", "run"]

# # Stage 1: Build the application
# FROM rust:alpine3.20 as builder

# # Set the working directory inside the container
# WORKDIR /usr/src/interface

# # Install build dependencies
# RUN apk update --no-cache && \
#     apk upgrade --no-cache && \
#     apk add --no-cache build-base musl-dev openssl-dev pkgconfig

# # Copy the entire source code
# COPY . .

# # Fetch dependencies and build the application in release mode
# RUN cargo build --release --verbose

# # Stage 2: Create a minimal runtime image
# FROM alpine:3.20

# # Set the working directory inside the container
# WORKDIR /usr/src/interface

# # Install runtime dependencies
# RUN apk add --no-cache openssl

# # Copy the built application from the builder stage
# COPY --from=builder /usr/src/interface/target/release/interface .

# # Expose the necessary port
# EXPOSE 3000

# # Command to run the application
# CMD ["./interface"]
