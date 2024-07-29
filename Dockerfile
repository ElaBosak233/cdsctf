FROM rust:latest AS backend

WORKDIR /app

COPY Cargo.toml ./

RUN cargo fetch

COPY . .

RUN rustup target add x86_64-unknown-linux-musl

RUN apt update && apt install -y musl-tools musl-dev pkg-config

RUN cargo build --release --target x86_64-unknown-linux-musl

RUN strip target/x86_64-unknown-linux-musl/release/cloudsdale

FROM node:20 AS frontend

COPY ./web /app
    
WORKDIR /app
    
RUN npm install
RUN npm run build

FROM alpine:latest

WORKDIR /app

COPY --from=backend /app/target/x86_64-unknown-linux-musl/release/cloudsdale .
COPY --from=frontend /app/dist ./dist

EXPOSE 8888

CMD ["./cloudsdale"]
