use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::User,
};

#[derive(Clone, Copy)]
pub struct ViewPublicProfilePayload<'a> {
    pub target: &'a User,
}

pub struct ViewPublicProfilePolicy<'a>(ViewPublicProfilePayload<'a>);

impl<'a> ViewPublicProfilePolicy<'a> {
    pub fn new(payload: ViewPublicProfilePayload<'a>) -> Self {
        Self(payload)
    }
}

impl<'a> Policy<'a> for ViewPublicProfilePolicy<'a> {
    // 任意のログイン済みユーザーは任意のユーザーのプロフィールを閲覧できる
    fn check(&self, _ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        Ok(())
    }
}
