# Rust Clean Architecture Auth API

クリーンアーキテクチャの原則に基づき、**Actix Web** と **SeaORM** を使用して構築された認証システムです。

## 🚀 特徴

* **Layered Architecture**: ドメイン層をインフラ層（SeaORM）から完全に分離。
* **Secure Auth**: `Argon2` によるパスワードハッシュ化と `JWT` トークン認証。
* **Dockerized Workflows**: アプリ実行用とツール用（CLI）のプロファイルを分けた効率的な開発環境。

---

## 🏗 アーキテクチャ構成

| レイヤー           | 役割                                               | 依存先                          |
| ------------------ | -------------------------------------------------- | ------------------------------- |
| **Domain**         | ビジネスルール、エンティティ、リポジトリの定義     | なし                            |
| **Service**        | ユースケースの実装（認証ロジック、ハッシュ化など） | Domain                          |
| **Infrastructure** | 具体的な実装（SeaORMによるDB操作、JWT発行など）    | Domain, Service                 |
| **API (Main)**     | Actix Web ハンドラー、DI、サーバー設定             | Domain, Service, Infrastructure |

---

## 🛠 セットアップ手順

### 1. 環境準備

```bash
cp .env.example .env

```

### 2. ツール（CLI）とアプリのビルド

本プロジェクトでは、DB操作用のツールと実行環境を個別にビルドします。

```bash
make build        # アプリケーションのビルド
make build-tools  # sea-orm-cli 等のツール用ビルド

```

### 3. コンテナの起動と初期化

```bash
make up           # コンテナ起動 (app, db)
make migrate-up   # 初期テーブルの作成

```

---

## 🔄 開発ワークフロー

データベースに変更を加え、それをコードに反映させる標準的な流れは以下の通りです。

1. **マイグレーションの作成**

```bash
make migrate-generate name=create_posts_table

```

`migration/src/m202XXXX_XXXXXX_name.rs` を編集してテーブル定義を書きます。
2. **スキーマの適用**

```bash
make migrate-up

```

1. **エンティティの自動生成**
DBの最新状態からRustの構造体を生成し、インフラ層に反映します。

```bash
make generate-entity

```

1. **ロジックの実装と確認**
`Domain` / `Service` を実装し、ホットリロードで動作を確認します。

```bash
make watch

```

---

## 💻 開発コマンド (Makefile)

### Docker 操作

* `make build-tools`: ツール専用プロファイルを使用して CLI 環境を構築します。
* `make shell`: アプリコンテナ内に入り、`cargo` コマンド等を直接実行します。
* `make db-shell`: `psql` を起動し、データベースの中身を直接クエリします。

### SeaORM 操作

* `make migrate-status`: マイグレーションの適用状況を確認します。
* `make generate-entity`: DBスキーマから `src/infrastructure/persistence/seaorm/entities` 内にモデルを自動出力します。

---

## 📂 ディレクトリ構造

```text
.
├── migration/          # SeaORM マイグレーション（独立したクレート）
├── src/
│   ├── domain/         # 純粋なビジネスロジックとインターフェース
│   │   ├── repository/ # リポジトリトレイトの定義
│   │   └── user.rs     # Userドメインモデル
│   ├── service/        # ユースケース（AuthServiceなど）
│   ├── infrastructure/ # 技術詳細（DB実装など）
│   │   └── persistence/
│   │       └── seaorm/ # SeaORM固有の実装
│   │           └── entities/ # 自動生成されたDBモデル
│   └── main.rs         # Actix Webの設定、DI、サーバー起動
├── Cargo.toml          # Rustプロジェクトの依存関係・ビルド設定
├── Cargo.lock          # 依存ライブラリの厳密なバージョンロック
├── compose.yml         # Docker Compose構成（app, db, tools）
├── Dockerfile          # マルチステージビルドによる実行環境定義
├── Makefile            # 開発ワークフローの自動化コマンド集
├── .env                # 環境変数（データベースURL、JWT秘密鍵など）
└── README.md           # 本ドキュメント

```

---

## 📡 API エンドポイント

| メソッド | パス           | 説明                  |
| -------- | -------------- | --------------------- |
| `POST`   | `/auth/signup` | ユーザー登録          |
| `POST`   | `/auth/login`  | ログイン（JWTを返却） |
