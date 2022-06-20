FROM rust:1.59-alpine AS builder

RUN apk add --no-cache build-base

WORKDIR /usr/src/thrustin
COPY . .

# Installs to /usr/local/cargo/bin based on CARGO_HOME environment variable
# https://github.com/rust-lang/docker-rust/blob/682bb532b4a3ba9f9d57f5a1ba3cb4ddd51fd127/1.57.0/alpine3.14/Dockerfile#L8
RUN cargo install --path .

FROM rust:1.59-alpine

# For running health checks, I think wget may not be sophisticated enough to test websockets
RUN apk add --no-cache curl

WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/thrustin /usr/local/bin/thrustin

ENV RUST_LOG debug

CMD [ "thrustin"]