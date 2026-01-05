use std::sync::Arc;

use sea_orm::{DatabaseConnection, DatabaseTransaction};

pub trait Connectable<T: sea_orm::ConnectionTrait> {
    fn connect(&self) -> &T;
}

impl Connectable<DatabaseConnection> for Arc<DatabaseConnection> {
    fn connect(&self) -> &DatabaseConnection {
        self.as_ref()
    }
}

impl Connectable<DatabaseTransaction> for &DatabaseTransaction {
    fn connect(&self) -> &DatabaseTransaction {
        self
    }
}
