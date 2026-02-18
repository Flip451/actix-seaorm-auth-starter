use std::sync::Arc;

use crate::{
    shared::service::clock::Clock,
    user::{HashedPassword, User, UserIdGenerator, UserRepositoryError, service::UniqueUserInfo},
};

pub struct UserFactory {
    clock: Arc<dyn Clock>,
}

impl UserFactory {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self { clock }
    }

    pub fn create_new_user(
        &self,
        user_id_generator: Arc<dyn UserIdGenerator>,
        user_info: UniqueUserInfo,
        password: HashedPassword,
    ) -> Result<User, UserRepositoryError> {
        let now = self.clock.now();
        let user_id = user_id_generator.generate()?;

        Ok(User::new(user_id, user_info, password, now)?)
    }
}
