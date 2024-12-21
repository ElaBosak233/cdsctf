FROM rust:latest AS builder

WORKDIR /app

RUN rustup install nightly && rustup default nightly

COPY . .

RUN cargo fetch

RUN rustup target add x86_64-unknown-linux-musl

RUN apt update && apt install -y musl-tools musl-dev pkg-config

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cdsctf .

EXPOSE 8888

CMD ["./cdsctf"]
