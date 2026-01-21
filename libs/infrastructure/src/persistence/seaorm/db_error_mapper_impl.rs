use sea_orm::{DbErr, RuntimeErr};

use crate::persistence::db_error_mapper::DbErrorMapper;

impl DbErrorMapper for DbErr {
    fn is_unique_violation(&self) -> bool {
        // バックエンド（Postgres/MySQL）に応じたコード判定を行う
        // Postgresなら "23505", MySQLなら "1062" など
        if let DbErr::Query(RuntimeErr::SqlxError(e)) = self {
            e.as_database_error()
                .map(|d| d.code().as_deref() == Some("23505"))
                .unwrap_or(false)
        } else {
            false
        }
    }

    fn constraint_name(&self) -> Option<&str> {
        if let DbErr::Query(RuntimeErr::SqlxError(sqlx_err)) = &self
            && let Some(db_err) = sqlx_err.as_database_error()
        {
            return db_err.constraint();
        }
        None
    }
}
