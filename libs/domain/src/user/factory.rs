use std::sync::Arc;

use crate::user::{
    HashedPassword, User, UserRepositoryError, UserUniquenessService, service::IdGenerator,
};

pub struct UserFactory {
    id_generator: Arc<dyn IdGenerator>,
}

impl UserFactory {
    pub fn new(id_generator: Arc<dyn IdGenerator>) -> Self {
        Self { id_generator }
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

        Ok(User::new(id, user_info, password)?)
    }
}
