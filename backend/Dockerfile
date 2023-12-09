FROM rust:alpine as builder
RUN apk update && apk add openssl-dev pkgconfig musl musl-dev gcc
RUN mkdir backend
COPY . .
RUN cargo install --path .

FROM alpine:latest
RUN apk update && apk add openssl
COPY ./email-templates /email-templates
COPY --from=builder /usr/local/cargo/bin/the-backend /usr/local/bin/the-backend
ENTRYPOINT the-backend
