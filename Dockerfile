# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.92.0

# 1. 共通ベース (chef)
# cmake は、一部の Rust 依存関係（-sys クレートなど）のネイティブビルドに必要
FROM lukemathwalker/cargo-chef:latest-rust-${RUST_VERSION} AS chef
WORKDIR /app
RUN apt-get update && apt-get install -y \
    libpq-dev \
    mold \
    clang \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# 2. ツールビルド専用ステージ
# コード修正の影響を受けないよう、ソースコピー前に実行
FROM chef AS tools-builder
# ツール群は Rust ${RUST_VERSION} での動作を確認したバージョンに固定している。
# Rust のメジャー／マイナーバージョンを更新する場合は、以下のバージョンとの互換性を確認し、
# 必要に応じて一緒に更新すること（再ビルドおよびテストが必要）。
ARG SCCACHE_VERSION=0.14.0
ARG CARGO_WATCH_VERSION=8.5.3
ARG SEA_ORM_VERSION=1.1.19
ARG AICHAT_VERSION=0.30.0

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    cargo install --locked --version ${SCCACHE_VERSION} sccache --root /usr/local && \
    cargo install --locked --version ${CARGO_WATCH_VERSION} cargo-watch && \
    cargo install --locked --version ${SEA_ORM_VERSION} sea-orm-cli && \
    cargo install --locked --version ${AICHAT_VERSION} aichat

# 3. レシピ作成 (planner)
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# 4. 依存関係ビルドの基盤 (builder-base)
# ここで依存ライブラリのコンパイルを完了させ、各ステージで使い回す
FROM chef AS builder-base
COPY --from=tools-builder /usr/local/bin/sccache /usr/local/bin/sccache
ENV RUSTC_WRAPPER=/usr/local/bin/sccache \
    SCCACHE_DIR=/opt/sccache \
    SCCACHE_IDLE_TIMEOUT=600

COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    --mount=type=cache,target=/opt/sccache,sharing=shared \
    cargo chef cook --release --recipe-path recipe.json

# 5. アプリケーションビルド専用ステージ (builder)
# 本番用バイナリの作成のみに責任を持つ
FROM builder-base AS builder
ARG APP_NAME=myapp
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    --mount=type=cache,target=/opt/sccache,sharing=shared \
    cargo build --release --bin ${APP_NAME} && \
    cp ./target/release/${APP_NAME} /bin/server

# 6. アプリ開発用ステージ (dev)
# builder-base を継承するため、重い依存関係のビルドは CACHED される
FROM builder-base AS dev
COPY --from=tools-builder /usr/local/cargo/bin/cargo-watch /usr/local/bin/
COPY --from=tools-builder /usr/local/cargo/bin/aichat /usr/local/bin/
ENV AICHAT_PLATFORM=google
ENV AICHAT_CONFIG_DIR=/app/.aichat

# 7. 運用ツール用ステージ (tools)
FROM chef AS tools
COPY --from=tools-builder /usr/local/cargo/bin/sea-orm-cli /usr/local/bin/
COPY --from=tools-builder /usr/local/cargo/bin/aichat /usr/local/bin/
RUN rustup component add --toolchain ${RUST_VERSION} rustfmt clippy
ENTRYPOINT ["sea-orm-cli"]

# 8. 本番実行用 (runtime)
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
WORKDIR /app
COPY --from=builder /bin/server /app/server
EXPOSE 8080
ENTRYPOINT ["/app/server"]