use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::User,
};

#[derive(Clone, Copy)]
pub struct ViewOwnProfilePayload<'a> {
    pub target: &'a User,
}

pub struct ViewOwnProfilePolicy<'a>(ViewOwnProfilePayload<'a>);

impl<'a> ViewOwnProfilePolicy<'a> {
    pub fn new(payload: ViewOwnProfilePayload<'a>) -> Self {
        Self(payload)
    }
}

impl<'a> Policy<'a> for ViewOwnProfilePolicy<'a> {
    // 任意のログイン済みユーザーは自分自身のプロフィールを閲覧できる
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        let target = self.0.target;

        if ctx.actor_id == target.id() {
            Ok(()) // 自分自身のプロフィールは閲覧可能
        } else {
            Err(AuthorizationError::Forbidden) // その他のケースは拒否
        }
    }
}
