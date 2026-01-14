use std::sync::Arc;

use crate::user::{
    HashedPassword, User, UserRepositoryError, UserUniquenessService, service::IdGenerator,
};

pub struct UserFactory<'a> {
    id_generator: Arc<dyn IdGenerator>,
    uniqueness_service: UserUniquenessService<'a>,
}

impl<'a> UserFactory<'a> {
    pub fn new(
        id_generator: Arc<dyn IdGenerator>,
        uniqueness_service: UserUniquenessService<'a>,
    ) -> Self {
        Self {
            id_generator,
            uniqueness_service,
        }
    }

    pub async fn create_new_user(
        &self,
        username: &str,
        email: &str,
        password: HashedPassword,
    ) -> Result<User, UserRepositoryError> {
        let user_info = self
            .uniqueness_service
            .ensure_unique(username, email)
            .await?;

        let id = self.id_generator.generate();

        Ok(User::new(id, user_info, password)?)
    }
}
