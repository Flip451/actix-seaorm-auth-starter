# 変数定義
DOCKER_COMPOSE = docker compose
# BuildKit を強制的に有効化するための環境変数
export DOCKER_BUILDKIT=1
export COMPOSE_DOCKER_CLI_BUILD=1
BUILD_ARGS = --build-arg BUILDKIT_INLINE_CACHE=1

APP_SERVICE = app
DB_SERVICE = db
CLI_SERVICE = sea-orm-cli
ENTITY_OUTPUT = libs/infrastructure/src/persistence/seaorm/entities

.PHONY: help build build-no-cache up down restart logs ps shell db-shell build-tools migrate-generate migrate-up migrate-down migrate-status generate-entity add run watch fmt lint test check check-all-features ci clean

help: ## ヘルプを表示
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Docker 操作
build: ## BuildKit を使用してキャッシュマウントを有効にしながらビルド
	$(DOCKER_COMPOSE) build $(BUILD_ARGS)

build-no-cache: ## イメージのレイヤーキャッシュを無視してビルド (BuildKit の cache mount は保持されます)
	$(DOCKER_COMPOSE) build --no-cache

up: ## コンテナの起動
	$(DOCKER_COMPOSE) up -d

down: ## コンテナの停止
	$(DOCKER_COMPOSE) down

restart: ## コンテナの再起動
	$(DOCKER_COMPOSE) restart

logs: ## ログの表示
	$(DOCKER_COMPOSE) logs -f $(APP_SERVICE)

ps: ## コンテナの状態確認
	$(DOCKER_COMPOSE) ps

shell: ## appコンテナにシェルで入る
	$(DOCKER_COMPOSE) run --rm $(APP_SERVICE) bash

db-shell: ## DBコンテナのpsqlに入る
	$(DOCKER_COMPOSE) exec db psql -U user -d myapp

# SeaORM 操作
build-tools: ## CLI ツールをビルド
	$(DOCKER_COMPOSE) --profile tools build $(BUILD_ARGS) $(CLI_SERVICE)

migrate-generate: ## 新規マイグレーションファイルを作成
	@if [ -z "$(name)" ]; then \
		echo "エラー: name 引数が必要です。 (例: make migrate-generate name=create_user)"; \
		exit 1; \
	fi
	$(DOCKER_COMPOSE) --profile tools run --rm $(CLI_SERVICE) migrate generate $(name)
	sudo chown -R $(shell id -u):$(shell id -g) migration

migrate-up: ## マイグレーションの実行
	$(DOCKER_COMPOSE) --profile tools run --rm $(CLI_SERVICE) migrate up

migrate-down: ## マイグレーションのロールバック(1ステップ)
	$(DOCKER_COMPOSE) --profile tools run --rm $(CLI_SERVICE) migrate down

migrate-status: ## マイグレーションの状態確認
	$(DOCKER_COMPOSE) --profile tools run --rm $(CLI_SERVICE) migrate status

generate-entity: ## DBからエンティティを自動生成
	$(DOCKER_COMPOSE) --profile tools run --rm $(CLI_SERVICE) generate entity \
		--with-serde serialize \
		-o $(ENTITY_OUTPUT)
	@# 生成されたファイルの所有権をホストユーザーに戻す
	sudo chown -R $(shell id -u):$(shell id -g) $(ENTITY_OUTPUT)

# アプリケーションの起動
run: ## アプリケーションの実行
	$(DOCKER_COMPOSE) exec $(APP_SERVICE) cargo run

watch: ## ホットリロード有効で実行 (cargo-watchが必要)
	$(DOCKER_COMPOSE) exec $(APP_SERVICE) cargo watch -x run

fmt: ## コードのフォーマット
	$(DOCKER_COMPOSE) --profile tools run --rm --entrypoint cargo $(CLI_SERVICE) fmt --all

lint: ## コードの静的解析
	$(DOCKER_COMPOSE) --profile tools run --rm --entrypoint cargo $(CLI_SERVICE) clippy --all -- -D warnings

add: ## 依存クレートを追加 (使用例: make add d=serde)
	@if [ -z "$(d)" ]; then \
		echo "エラー: d (dependency) の指定が必要です。"; \
		echo "使用例: make add d=\"uuid --features v4\""; \
		exit 1; \
	fi
	$(DOCKER_COMPOSE) --profile tools run --rm --entrypoint cargo $(CLI_SERVICE) add $(d)
	@# 変更されたCargo関連ファイルの所有権をホストユーザーに戻す (rootで書き換わるため)
	sudo find . \( -name 'Cargo.toml' -o -name 'Cargo.lock' \) -exec chown $(shell id -u):$(shell id -g) {} +

# テスト実行
test: ## アプリケーションのテスト
	$(DOCKER_COMPOSE) run --rm $(APP_SERVICE) cargo test --workspace

# cargo check
check: ## 本番ビルドが通るか確認
	$(DOCKER_COMPOSE) run --rm $(APP_SERVICE) cargo check --workspace --release

# cargo check --features api-docs
check-all-features: ## APIドキュメント生成が通るか確認
	$(DOCKER_COMPOSE) run --rm $(APP_SERVICE) cargo check --workspace --all-features

# fmt, lint, test, check をまとめて実行
ci: fmt lint test check check-all-features ## CI用: フォーマット, 静的解析, テスト, チェックを実行
	@echo "CI checks passed."

# キャッシュのクリーンアップなど
clean: ## ボリュームの削除, target ディレクトリの削除
	$(DOCKER_COMPOSE) down --volumes --remove-orphans
	sudo rm -rf target/