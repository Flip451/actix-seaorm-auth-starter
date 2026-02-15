# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.92.0

# 1. 共通ベース (chef)
FROM lukemathwalker/cargo-chef:latest-rust-${RUST_VERSION} AS chef
WORKDIR /app
RUN apt-get update && apt-get install -y \
    libpq-dev \
    mold \
    clang \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# 2. レシピ作成 (planner)
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# 3. 依存関係 & ツールビルド (builder)
FROM chef AS builder
ARG APP_NAME=myapp
# sccache を導入してコンパイル結果を再利用
ARG SCCACHE_VERSION=0.14.0
RUN cargo install --locked --version ${SCCACHE_VERSION} sccache --root /usr/local
ENV RUSTC_WRAPPER=/usr/local/bin/sccache

COPY --from=planner /app/recipe.json recipe.json

# BuildKit のキャッシュマウントをフル活用
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo chef cook --release --recipe-path recipe.json

# アプリのビルド
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --bin ${APP_NAME} && \
    cp ./target/release/${APP_NAME} /bin/server

# 4. 開発ツール専用ステージ
FROM builder AS tools
# 開発に必要なツールをキャッシュを効かせてインストール
ARG SEA_ORM_VERSION=1.1.19
ARG CARGO_WATCH_VERSION=8.5.3

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install --locked --version ${SEA_ORM_VERSION} sea-orm-cli && \
    cargo install --locked --version ${CARGO_WATCH_VERSION} cargo-watch

RUN rustup component add rustfmt clippy
ENTRYPOINT ["sea-orm-cli"]

# 5. 本番実行用 (runtime)
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
WORKDIR /app
COPY --from=builder /bin/server /app/server
USER nonroot:nonroot
EXPOSE 8080
ENTRYPOINT ["/app/server"]