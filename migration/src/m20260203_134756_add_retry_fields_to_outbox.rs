use domain::shared::outbox_event::entity::OutboxEventStatusKind;
use sea_orm_migration::prelude::*;

use crate::constants::Indices;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // カラム追加
        manager
            .alter_table(
                Table::alter()
                    .table(Outbox::Table)
                    // リトライ回数 (デフォルト0)
                    .add_column(
                        ColumnDef::new(Outbox::RetryCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    // 次回実行予定時刻
                    .add_column(
                        ColumnDef::new(Outbox::NextAttemptAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    // 最終試行時刻
                    .add_column(
                        ColumnDef::new(Outbox::LastAttemptedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // インデックス追加
        manager
            .create_index(
                Index::create()
                    .name::<&'static str>(Indices::OutboxProcessQueue.into())
                    .table(Outbox::Table)
                    .col(Outbox::NextAttemptAt) // ソート対象のカラム
                    .and_where(Expr::col(Outbox::Status).is_in([
                        OutboxEventStatusKind::Pending.to_string(),
                        OutboxEventStatusKind::Failed.to_string(),
                    ]))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // インデックスの削除
        manager
            .drop_index(
                Index::drop()
                    .name::<&'static str>(Indices::OutboxProcessQueue.into())
                    .table(Outbox::Table)
                    .to_owned(),
            )
            .await?;

        // カラムの削除
        manager
            .alter_table(
                Table::alter()
                    .table(Outbox::Table)
                    .drop_column(Outbox::LastAttemptedAt)
                    .drop_column(Outbox::NextAttemptAt)
                    .drop_column(Outbox::RetryCount)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Outbox {
    Table,
    Status,
    RetryCount,
    NextAttemptAt,
    LastAttemptedAt,
}
