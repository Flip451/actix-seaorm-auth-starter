use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // user テーブルに role カラムを追加
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("role"))
                            .string()
                            .not_null()
                            .default("user"), // デフォルト値を設定
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ロールバック時は role カラムを削除
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(Alias::new("role"))
                    .to_owned(),
            )
            .await
    }
}

/// テーブル名の定義（既存のものと合わせる）
#[derive(DeriveIden)]
enum User {
    Table,
}