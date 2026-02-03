use domain::shared::outbox_event::entity::OutboxEventStatusKind;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Outbox::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Outbox::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Outbox::EventType).string().not_null()) // 例: "UserEvent::Suspended"
                    .col(ColumnDef::new(Outbox::Payload).json_binary().not_null()) // イベントの中身
                    .col(
                        ColumnDef::new(Outbox::Status)
                            .string()
                            .not_null()
                            .default(OutboxEventStatusKind::Pending.to_string()),
                    ) // PENDING, PUBLISHED, FAILED, COMPLETED
                    .col(ColumnDef::new(Outbox::TraceId).string().null()) // OpenTelemetryのTraceID
                    .col(
                        ColumnDef::new(Outbox::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Outbox::ProcessedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Outbox::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Outbox {
    Table,
    Id,
    EventType,
    Payload,
    Status,
    TraceId,
    CreatedAt,
    ProcessedAt,
}
