FROM rust:slim-bookworm as builder
RUN apt-get update && apt-get install -y openssl libssl-dev pkg-config gcc
RUN rustup set profile minimal && rustup toolchain install nightly-x86_64-unknown-linux-gnu && rustup default nightly-x86_64-unknown-linux-gnu
COPY . .
RUN cargo install --locked --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl
COPY ./email-templates /email-templates
COPY --from=builder /usr/local/cargo/bin/the-backend /usr/local/bin/the-backend
ENTRYPOINT the-backend
