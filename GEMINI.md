# GEMINI.md

## 1. プロジェクト概要

* **言語:** Rust (Edition 2024+)
* **アーキテクチャ:** ヘキサゴナルアーキテクチャ（Ports and Adapters）
* **設計思想:** 関数型ドメイン駆動設計 (Functional DDD)
* **参照モデル:** Domain Modeling Made Functional (DMMF) の手法を Rust の型システムで実装する。

## 2. 実装原則 (Core Principles)

### A. 型によるドメインモデリング (Algebraic Data Types)

* **Making Illegal States Unrepresentable:** 不正な状態を型システムで表現不可能にする。
* **ADTsの活用:** `struct` と `enum` を組み合わせてドメインを表現する。
* **NewType パターン:** `struct Email(String);` のように、プリミティブ型をラップしてバリデーション済みの値であることを保証する。
* **Enum による状態表現:** `bool` フラグではなく、`enum OrderStatus { Pending, Paid, Shipped }` のように明示的な状態遷移を定義する。

### B. ビジネスワークフロー (Functions as Workflows)

* **ワークフローの定義:** 1つのビジネスユースケースを「入力型を受け取り、出力型を返す純粋関数」として定義する。
* シグネチャ例: `fn place_order(command: PlaceOrderCommand) -> Result<OrderPlacedEvent, OrderError>`

* **パイプライン処理:** 各ステップを小さな関数に分割し、`and_then` やメソッドチェーンで合成する。
* **副作用の分離:** ビジネスロジック（純粋関数）と、DB・外部APIなどの副作用（Adapters）を厳格に分離する。

### C. ヘキサゴナルアーキテクチャ

* **Domain Layer:** 依存関係を持たない。ビジネスモデルとワークフロー（Trait定義含む）のみを配置。
* **Application Layer (UseCases):** ドメインワークフローを呼び出し、オーケストレーションを行う。
* **Infrastructure Layer (Adapters):** 外部依存（DB, HTTP, FileSystem）の実装。
* **Ports (Traits):** Domain または Application 層で定義し、Infrastructure 層で実装する。

## 3. コーディング規約 (Rules & Constraints)

* **No Panic:** `unwrap()` や `expect()` は原則禁止。エラーは `Result<T, E>` で伝播させる。
* **Domain Errors:** エラーもドメインの一部として `enum` で定義する。
* **Immutability:** 原則として `mut` を避け、シャドウイングや値の再生成による不変性を優先する。
* **Validation:** 「コンストラクタ」でバリデーションを行い、型を生成した時点で有効であることを保証する。

## 4. プライバシーとセキュリティ (Privacy Guidelines)

* **機密情報の除外:** APIキー、パスワード、接続文字列をプロンプトに含めない。
* **PIIの保護:** 顧客名やメールアドレスなどの個人情報（PII）はダミーデータに置き換えて送信する。
* **コード送信の最小化:** 解決したいロジックに直接関係のない機密ロジックは抽象化して伝える。

## 5. Gemini への指示 (Instructions for Gemini)

* 新機能の実装を依頼された際は、まず「型定義 (Domain Models)」から提案すること。
* ロジックを書く際は、まず「ワークフローの関数シグネチャ」を定義すること。
* コードレビューを依頼された際は、上記の「実装原則」に反していないか、特に「不正な状態を許容していないか」を重点的にチェックすること。
