# syntax=docker/dockerfile:1.7

FROM rust:1.95 AS rust-toolchain

ARG TARGETARCH

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends musl-tools musl-dev clang pkg-config lld && \
    case "$TARGETARCH" in \
        arm64) MUSL_TARGET=aarch64-unknown-linux-musl ;; \
        *) MUSL_TARGET=x86_64-unknown-linux-musl ;; \
    esac && \
    rustup target add "$MUSL_TARGET" && \
    echo "$MUSL_TARGET" > /musl_target && \
    cargo install --locked cargo-chef --version 0.1.73 && \
    rm -rf /var/lib/apt/lists/*

FROM rust-toolchain AS rust-planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM rust-toolchain AS rust-dependencies

COPY --from=rust-planner /app/recipe.json recipe.json

RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=cargo-target,target=/app/target \
    cargo chef cook --locked --release --recipe-path recipe.json --target "$(cat /musl_target)"

FROM rust-dependencies AS backend-build

COPY . .

RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=cargo-target,target=/app/target \
    set -e && \
    cargo test --locked --release --workspace && \
    MUSL_TARGET=$(cat /musl_target) && \
    cargo build --locked --release --bin cds-server --target "$MUSL_TARGET" && \
    cp "/app/target/$MUSL_TARGET/release/cds-server" /usr/local/bin/cds-server

FROM node:25 AS frontend-build

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

RUN npm install -g pnpm

WORKDIR /app

COPY web/package.json web/pnpm-lock.yaml web/pnpm-workspace.yaml ./

RUN --mount=type=cache,id=pnpm,target=/pnpm/store \
    pnpm install --frozen-lockfile --prefer-offline

COPY web .

RUN pnpm run build && \
    mkdir -p /var/www/html && \
    cp -r /app/dist/. /var/www/html

FROM alpine:3 AS runtime

RUN apk add --no-cache curl

WORKDIR /app

COPY --from=backend-build /usr/local/bin/cds-server ./cds-server
COPY --from=frontend-build /var/www/html ./dist

EXPOSE 8888

HEALTHCHECK --interval=5m --timeout=3s --start-period=10s --retries=1 \
    CMD curl -fsSL http://127.0.0.1:8888/healthz || exit 1

CMD ["./cds-server"]
