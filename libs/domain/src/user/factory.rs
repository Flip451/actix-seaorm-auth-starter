use std::sync::Arc;

use crate::{
    shared::service::clock::Clock,
    user::{HashedPassword, IdGeneratorFactory, User, UserRepositoryError, UserUniquenessService},
};

pub struct UserFactory {
    clock: Arc<dyn Clock>,
}

impl UserFactory {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self { clock }
    }

    pub async fn create_new_user<'a>(
        &self,
        uniqueness_service: UserUniquenessService<'a>,
        user_id_generator_factory: &dyn IdGeneratorFactory,
        username: &str,
        email: &str,
        password: HashedPassword,
    ) -> Result<User, UserRepositoryError> {
        let user_info = uniqueness_service.ensure_unique(username, email).await?;
        let user_id_generator = user_id_generator_factory.create_user_id_generator();

        let id = user_id_generator.generate();
        let now = self.clock.now();

        Ok(User::new(id, user_info, password, now)?)
    }
}
