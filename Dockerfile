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

# 2. レシピ作成 (planner)
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# 3. 依存関係 & バイナリビルド (builder)
FROM chef AS builder
ARG APP_NAME=myapp
# sccache を導入してコンパイル結果を再利用
ARG SCCACHE_VERSION=0.14.0
RUN cargo install --locked --version ${SCCACHE_VERSION} sccache --root /usr/local

# sccache 関連の環境変数を設定
ENV RUSTC_WRAPPER=/usr/local/bin/sccache SCCACHE_DIR=/opt/sccache SCCACHE_IDLE_TIMEOUT=0

COPY --from=planner /app/recipe.json recipe.json

# 依存関係のビルド（SCCACHE_DIR をマウント）
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    --mount=type=cache,target=/opt/sccache,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

# アプリ本体のビルド（ここでも SCCACHE_DIR をマウント）
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    --mount=type=cache,target=/opt/sccache,sharing=locked \
    cargo build --release --bin ${APP_NAME} && \
    cp ./target/release/${APP_NAME} /bin/server

# 4. アプリ開発用ステージ (app サービスが使用)
FROM builder AS dev
ARG CARGO_WATCH_VERSION=8.5.3
# アプリのホットリロードに必要なツールのみをインストール
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    cargo install --locked --version ${CARGO_WATCH_VERSION} cargo-watch

# 5. ツール専用ステージ (sea-orm-cli サービスが使用)
FROM builder AS tools
# 開発に必要なツールをキャッシュを効かせてインストール
ARG SEA_ORM_VERSION=1.1.19
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    cargo install --locked --version ${SEA_ORM_VERSION} sea-orm-cli
RUN rustup component add --toolchain ${RUST_VERSION} rustfmt clippy
ENTRYPOINT ["sea-orm-cli"]

# 6. 本番実行用 (runtime)
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
WORKDIR /app
COPY --from=builder /bin/server /app/server
EXPOSE 8080
ENTRYPOINT ["/app/server"]