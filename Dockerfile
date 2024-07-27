FROM rust:latest AS backend

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

RUN rustup target add x86_64-unknown-linux-musl

RUN apt update && apt install -y musl-tools musl-dev pkg-config libssl-dev ca-certificates

ENV OPENSSL_DIR=/usr
ENV OPENSSL_INCLUDE_DIR=/usr/include
ENV OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM node:20 AS frontend

COPY ./web /app
    
WORKDIR /app
    
RUN npm install
RUN npm run build

FROM alpine:latest

RUN apk --no-cache add ca-certificates

WORKDIR /app

COPY --from=backend /app/target/x86_64-unknown-linux-musl/release/cloudsdale .
COPY --from=frontend /app/dist ./dist

EXPOSE 8888

CMD ["./cloudsdale"]
