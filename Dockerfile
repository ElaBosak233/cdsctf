FROM rust:latest AS backend

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

RUN apt update && \
    apt install -y musl-tools musl-dev clang pkg-config lld

COPY . .

RUN cargo fetch && \
    cargo build --release --bin cds-server --target x86_64-unknown-linux-musl && \
    cp /app/target/x86_64-unknown-linux-musl/release/cds-server /usr/local/bin/cds-server

FROM node:25 AS frontend

WORKDIR /app

COPY ./web .

RUN npm install && \
    npm run build && \
    mkdir -p /var/www/html && \
    cp /app/dist/. /var/www/html -r

FROM alpine:latest

WORKDIR /app

COPY --from=backend /usr/local/bin/cds-server ./cds-server
COPY --from=frontend /var/www/html ./dist

EXPOSE 8888

CMD ["./cds-server"]
