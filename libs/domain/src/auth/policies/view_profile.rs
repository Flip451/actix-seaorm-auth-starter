use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::User,
};

#[derive(Clone, Copy)]
pub struct ViewProfilePayload<'a> {
    pub target: &'a User,
}

pub struct ViewProfilePolicy<'a>(ViewProfilePayload<'a>);

impl<'a> ViewProfilePolicy<'a> {
    pub fn new(payload: ViewProfilePayload<'a>) -> Self {
        Self(payload)
    }
}

impl<'a> Policy<'a> for ViewProfilePolicy<'a> {
    // 任意のログイン済みユーザーは任意のユーザーのプロフィールを閲覧できる
    fn check(&self, _ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        let _target = self.0.target;

        Ok(())
    }
}
