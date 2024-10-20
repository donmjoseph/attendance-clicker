# starting from rust:alpine
FROM rust:1.82 as builder
# doing everything inside a specific directory, we don't want to have an unreadable filesystem
WORKDIR /usr/src/interface

# copying everything from the current directory into /interface in the docker container
COPY . .

RUN cargo install --path .
EXPOSE 3000
CMD ["interface"]