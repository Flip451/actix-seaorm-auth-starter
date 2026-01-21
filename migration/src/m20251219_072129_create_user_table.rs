use sea_orm_migration::prelude::*;

use crate::constants::UniqueConstraints;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    // 1. UUIDを主キーに（推測不可能なID）
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    // 2. ユニーク制約（重複登録をDBレベルで防止）
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null())
                    // 3. パスワード（ハッシュ化した文字列を格納）
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    // 4. アカウント状態（論理削除や凍結用）
                    .col(
                        ColumnDef::new(User::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    // 5. 監査用タイムスタンプ（JST等のタイムゾーン対応）
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // emailカラムに対するユニークインデックスを作成
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name(UniqueConstraints::UserEmailKey.to_string())
                    .table(User::Table)
                    .col(User::Email)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // usernameカラムに対するユニークインデックスを作成
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name(UniqueConstraints::UserUsernameKey.to_string())
                    .table(User::Table)
                    .col(User::Username)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// テーブル名とカラム名の定義
#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
