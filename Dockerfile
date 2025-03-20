FROM rust:latest AS builder

WORKDIR /app

RUN rustup install nightly && \
    rustup default nightly && \
    rustup target add x86_64-unknown-linux-musl

RUN apt update && \
    apt install -y musl-tools musl-dev clang pkg-config lld

COPY . .

RUN --mount=type=cache,target=/app/target cargo fetch && \
    cargo build --release --bin cds-server --target x86_64-unknown-linux-musl

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cdsctf .

EXPOSE 8888

CMD ["./cds-server"]
