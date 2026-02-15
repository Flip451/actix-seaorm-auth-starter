# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.92.0

# 1. 共通ベース (chef)
# cmake は sccache や一部の Rust 依存関係（-sys クレート）のネイティブビルドに必要
FROM lukemathwalker/cargo-chef:latest-rust-${RUST_VERSION} AS chef
WORKDIR /app
RUN apt-get update && apt-get install -y \
    libpq-dev \
    mold \
    clang \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# 2. ツールビルド専用ステージ（キャッシュ効率化の要）
# ソースコードをコピーする前にツールをインストールすることで、コード修正の影響を受けない
FROM chef AS tools-builder
ARG SCCACHE_VERSION=0.14.0
ARG CARGO_WATCH_VERSION=8.5.3
ARG SEA_ORM_VERSION=1.1.19

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    cargo install --locked --version ${SCCACHE_VERSION} sccache --root /usr/local && \
    cargo install --locked --version ${CARGO_WATCH_VERSION} cargo-watch && \
    cargo install --locked --version ${SEA_ORM_VERSION} sea-orm-cli

# 3. レシピ作成 (planner)
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# 4. 依存関係ビルド (builder)
FROM chef AS builder
ARG APP_NAME=myapp
# 事前ビルドした sccache をコピー
COPY --from=tools-builder /usr/local/bin/sccache /usr/local/bin/sccache
ENV RUSTC_WRAPPER=/usr/local/bin/sccache \
    SCCACHE_DIR=/opt/sccache \
    SCCACHE_IDLE_TIMEOUT=0

COPY --from=planner /app/recipe.json recipe.json

# 三種のキャッシュマウント（registry, target, sccache）で高速化
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    --mount=type=cache,target=/opt/sccache,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

# アプリ本体のビルド
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    --mount=type=cache,target=/opt/sccache,sharing=locked \
    cargo build --release --bin ${APP_NAME} && \
    cp ./target/release/${APP_NAME} /bin/server

# 5. アプリ開発用ステージ (dev) - docker compose の app サービスで使用
FROM builder AS dev
COPY --from=tools-builder /usr/local/cargo/bin/cargo-watch /usr/local/bin/

# 6. 運用ツール用ステージ (tools) - sea-orm-cli サービスで使用
FROM builder AS tools
COPY --from=tools-builder /usr/local/cargo/bin/sea-orm-cli /usr/local/bin/
RUN rustup component add --toolchain ${RUST_VERSION} rustfmt clippy
ENTRYPOINT ["sea-orm-cli"]

# 7. 本番実行用 (runtime)
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
WORKDIR /app
COPY --from=builder /bin/server /app/server
EXPOSE 8080
ENTRYPOINT ["/app/server"]