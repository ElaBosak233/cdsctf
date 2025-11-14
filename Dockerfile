FROM rust:1.91 AS backend

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

RUN apt update && \
    apt install -y musl-tools musl-dev clang pkg-config lld

COPY . .

RUN cargo fetch && \
    cargo build --release --bin cds-server --target x86_64-unknown-linux-musl && \
    cp /app/target/x86_64-unknown-linux-musl/release/cds-server /usr/local/bin/cds-server

FROM node:25 AS frontend

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

RUN npm install -g corepack
RUN corepack enable

WORKDIR /app
COPY ./web/package.json ./web/pnpm-lock.yaml ./

RUN --mount=type=cache,id=pnpm,target=/pnpm/store \
    pnpm install --frozen-lockfile

COPY ./web .

RUN pnpm build && \
    mkdir -p /var/www/html && \
    cp -r /app/dist/. /var/www/html

FROM alpine:3

WORKDIR /app

COPY --from=backend /usr/local/bin/cds-server ./cds-server
COPY --from=frontend /var/www/html ./dist

EXPOSE 8888

HEALTHCHECK --interval=5m --timeout=3s --start-period=10s --retries=1 \
    CMD curl -fsSL http://127.0.0.1:8888/healthz || exit 1

CMD ["./cds-server"]
