use std::sync::Arc;

use crate::{
    shared::service::clock::Clock,
    user::{
        HashedPassword, User, UserRepositoryError, UserUniquenessService, service::IdGenerator,
    },
};

pub struct UserFactory {
    id_generator: Arc<dyn IdGenerator>,
    clock: Arc<dyn Clock>,
}

impl UserFactory {
    pub fn new(id_generator: Arc<dyn IdGenerator>, clock: Arc<dyn Clock>) -> Self {
        Self {
            id_generator,
            clock,
        }
    }

    pub async fn create_new_user<'a>(
        &self,
        uniqueness_service: UserUniquenessService<'a>,
        username: &str,
        email: &str,
        password: HashedPassword,
    ) -> Result<User, UserRepositoryError> {
        let user_info = uniqueness_service.ensure_unique(username, email).await?;

        let id = self.id_generator.generate();
        let now = self.clock.now();

        Ok(User::new(id, user_info, password, now)?)
    }
}
