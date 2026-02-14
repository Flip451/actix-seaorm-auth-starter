use crate::auth::policy::{AuthorizationContext, AuthorizationError, Policy};

#[derive(Clone, Copy)]
pub struct ViewPublicProfilePayload;

pub struct ViewPublicProfilePolicy(#[allow(dead_code)] ViewPublicProfilePayload);

impl ViewPublicProfilePolicy {
    pub fn new(payload: ViewPublicProfilePayload) -> Self {
        Self(payload)
    }
}

impl Policy for ViewPublicProfilePolicy {
    // 任意のログイン済みユーザーは任意のユーザーのプロフィールを閲覧できる
    fn check(&self, _ctx: &AuthorizationContext) -> Result<(), AuthorizationError> {
        Ok(())
    }
}
