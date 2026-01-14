use domain::user::{IdGenerator, UserId};
use sea_orm::prelude::Uuid;

pub struct UuidUserIdGenerator;

impl IdGenerator for UuidUserIdGenerator {
    fn generate(&self) -> UserId {
        Uuid::new_v4().into()
    }
}
