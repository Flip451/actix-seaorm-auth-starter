use domain::user::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(derive_more::Debug, Deserialize, Serialize)]
pub struct SignupInput {
    pub username: String,
    #[debug(skip)]
    pub email: String,
    #[debug(skip)]
    pub password: String,
}

#[derive(derive_more::Debug, Serialize)]
pub struct SignupOutput {
    pub user_id: Uuid,
}

impl From<User> for SignupOutput {
    fn from(user: User) -> Self {
        SignupOutput {
            user_id: user.id().into(),
        }
    }
}

#[derive(derive_more::Debug, Deserialize)]
pub struct LoginInput {
    #[debug(skip)]
    pub email: String,
    #[debug(skip)]
    pub password: String,
}

#[derive(derive_more::Debug, Serialize)]
pub struct LoginOutput {
    #[debug(skip)]
    pub token: String,
}
