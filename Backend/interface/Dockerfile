# Use rust:alpine as the base image
FROM rust:alpine3.20 AS builder

# Set the working directory inside the container
WORKDIR /usr/src/interface

# Install dependencies
RUN apk update --no-cache && \
    apk upgrade --no-cache && \
    apk add --no-cache build-base

# Copy the entire source code
COPY . .

# Fetch dependencies
RUN cargo fetch

# Expose the necessary port
EXPOSE 3000

# Command to start the application
CMD ["cargo", "run"]

