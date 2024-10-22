# starting from rust:alpine
FROM rust:alpine3.20 as builder
# doing everything inside a specific directory, we don't want to have an unreadable filesystem
WORKDIR /usr/src/interface

RUN apk update --no-cache
RUN apk upgrade --no-cache
RUN apk add build-base
# copying everything from the current directory into /interface in the docker container
COPY . .

RUN cargo install --path .
EXPOSE 3000
CMD ["interface"]