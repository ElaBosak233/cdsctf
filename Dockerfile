FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml ./

RUN cargo fetch

COPY . .

RUN rustup target add x86_64-unknown-linux-musl

RUN apt update && apt install -y musl-tools musl-dev pkg-config

RUN cargo build --release --target x86_64-unknown-linux-musl

RUN strip target/x86_64-unknown-linux-musl/release/cdsctf

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cdsctf .

EXPOSE 8888

CMD ["./cdsctf"]
