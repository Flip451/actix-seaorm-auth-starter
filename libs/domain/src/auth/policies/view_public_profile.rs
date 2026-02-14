use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::UserId,
};

#[derive(Clone, Copy)]
pub struct ViewPublicProfilePayload {
    pub target_id: UserId,
}

pub struct ViewPublicProfilePolicy(ViewPublicProfilePayload);

impl ViewPublicProfilePolicy {
    pub fn new(payload: ViewPublicProfilePayload) -> Self {
        Self(payload)
    }
}

impl Policy for ViewPublicProfilePolicy {
    // 任意のログイン済みユーザーは任意のユーザーのプロフィールを閲覧できる
    fn check(&self, _ctx: &AuthorizationContext) -> Result<(), AuthorizationError> {
        let _target_ids = self.0.target_id;

        Ok(())
    }
}
