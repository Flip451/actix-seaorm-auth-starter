pub use sea_orm_migration::prelude::*;

mod m20251219_072129_create_user_table;
mod m20251224_071855_add_role_to_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251219_072129_create_user_table::Migration),
            Box::new(m20251224_071855_add_role_to_user::Migration),
        ]
    }
}
