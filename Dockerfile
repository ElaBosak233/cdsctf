FROM rust:1.94 AS backend

ARG TARGETARCH

WORKDIR /app

RUN apt update && \
    apt install -y musl-tools musl-dev clang pkg-config lld && \
    case "$TARGETARCH" in \
        arm64) MUSL_TARGET=aarch64-unknown-linux-musl ;; \
        *) MUSL_TARGET=x86_64-unknown-linux-musl ;; \
    esac && \
    rustup target add $MUSL_TARGET && \
    echo $MUSL_TARGET > /musl_target

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo fetch

COPY . .

RUN MUSL_TARGET=$(cat /musl_target) && \
    cargo build --release --bin cds-server --target $MUSL_TARGET && \
    cp /app/target/$MUSL_TARGET/release/cds-server /usr/local/bin/cds-server

FROM node:25 AS frontend

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

RUN npm install -g pnpm

WORKDIR /app
COPY ./web/package.json ./web/pnpm-lock.yaml ./

RUN --mount=type=cache,id=pnpm,target=/pnpm/store \
    pnpm install --frozen-lockfile

COPY ./web .

RUN pnpm build && \
    mkdir -p /var/www/html && \
    cp -r /app/dist/. /var/www/html

FROM alpine:3

RUN apk add --no-cache curl

WORKDIR /app

COPY --from=backend /usr/local/bin/cds-server ./cds-server
COPY --from=frontend /var/www/html ./dist

EXPOSE 8888

HEALTHCHECK --interval=5m --timeout=3s --start-period=10s --retries=1 \
    CMD curl -fsSL http://127.0.0.1:8888/healthz || exit 1

CMD ["./cds-server"]
