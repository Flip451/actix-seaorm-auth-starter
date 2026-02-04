use sea_orm_migration::{
    prelude::*,
    sea_orm::{DbBackend, Statement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 既存のすべてのステータスを小文字に変換します (PostgreSQL)
        // 例: 'PENDING' -> 'pending', 'PERMANENTLY_FAILED' -> 'permanently_failed'
        // snake_case への変更であれば、多くの場合 LOWER() 関数で対応可能です。
        let sql = "UPDATE outbox SET status = LOWER(status)";

        db.execute(Statement::from_string(DbBackend::Postgres, sql))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // ロールバック時は大文字に戻します
        let sql = "UPDATE outbox SET status = UPPER(status)";

        db.execute(Statement::from_string(DbBackend::Postgres, sql))
            .await?;

        Ok(())
    }
}
