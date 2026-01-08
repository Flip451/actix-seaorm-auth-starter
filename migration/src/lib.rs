pub use sea_orm_migration::prelude::*;

mod m20251219_072129_create_user_table;
mod m20251224_071855_add_role_to_user;
mod m20260104_042952_replace_is_active_with_status;
mod m20260107_121138_create_outbox_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251219_072129_create_user_table::Migration),
            Box::new(m20251224_071855_add_role_to_user::Migration),
            Box::new(m20260104_042952_replace_is_active_with_status::Migration),
            Box::new(m20260107_121138_create_outbox_table::Migration),
        ]
    }
}
