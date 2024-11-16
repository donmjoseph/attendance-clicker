# Use rust:alpine as the base image to have Cargo available in the runtime
FROM rust:alpine3.20 as builder

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

# Expose the necessary port
EXPOSE 3000

# Command to start the application
CMD ["cargo", "run"]
