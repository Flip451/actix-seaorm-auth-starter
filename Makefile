# 変数定義
DOCKER_COMPOSE = docker compose
APP_SERVICE = app
DB_SERVICE = db
CLI_SERVICE = sea-orm-cli
ENTITY_OUTPUT = libs/infrastructure/src/persistence/seaorm/entities

.PHONY: help build up down restart logs ps shell db-shell build-tools migrate-generate migrate-up migrate-down migrate-status generate-entity run watch test clean

help: ## ヘルプを表示
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Docker 操作
build: ## Dockerイメージのビルド
	$(DOCKER_COMPOSE) build

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
build-tools: ## SeaORM の CLI ツール用ビルドステージのビルド
	$(DOCKER_COMPOSE) --profile tools build $(CLI_SERVICE)

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

# テスト実行
test: ## アプリケーションのテスト
	$(DOCKER_COMPOSE) run --rm $(APP_SERVICE) cargo test

# キャッシュのクリーンアップなど
clean: ## ボリュームの削除, target ディレクトリの削除
	$(DOCKER_COMPOSE) down --volumes --remove-orphans
	sudo rm -rf target/