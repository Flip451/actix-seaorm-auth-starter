use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::User,
};

#[derive(Clone, Copy)]
pub struct ChangeEmailPayload<'a> {
    pub target: &'a User,
}

pub struct ChangeEmailPolicy<'a>(ChangeEmailPayload<'a>);

impl<'a> ChangeEmailPolicy<'a> {
    pub fn new(payload: ChangeEmailPayload<'a>) -> Self {
        Self(payload)
    }
}

impl<'a> Policy<'a> for ChangeEmailPolicy<'a> {
    // ユーザーは自分自身のメールアドレスを変更できる
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        let target = self.0.target;

        if ctx.actor_id == target.id() {
            Ok(()) // ユーザーは自分自身のメールアドレスを変更可能
        } else {
            Err(AuthorizationError::Forbidden) // その他のケースは拒否
        }
    }
}
