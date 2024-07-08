# Rust 製の Web アプリケーション

Rust の使い方全般を実装するためのリポジトリです。

## 試したこと

- Docker を使って Rust の開発環境を構築
- actix-web を使って Web アプリケーションを作成
- cargo-watch を使ってファイルの変更を検知して自動でビルド
- 開発環境と本番ビルド環境の切り替え（実装途中）
- ルーティング、サービス、コントローラーモデルの実装

## モデル（テーブル）の作成からマイグレーションまで

1. マイグレーションファイルの作成:
   `diesel migration generate create_<table_name>` でマイグレーションファイルを作成する。
2. マイグレーションファイルの編集:
   `up.sql` にテーブルの作成、`down.sql` にテーブルの削除を記述する。
3. マイグレーションの実行:
   `diesel migration run` でマイグレーションを実行する。
4. モデルの作成:
   `src/schema.rs` にテーブルの定義を記述する。
   docker-compose exec app diesel print-schema > src/schema.rs
5. モデルの追加:
   `src/models.rs` にテーブルのモデルを追加する。

