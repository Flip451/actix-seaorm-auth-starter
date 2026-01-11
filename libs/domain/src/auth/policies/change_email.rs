use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::User,
};

pub struct ChangeEmailPolicy<'a> {
    pub target: &'a User,
}

impl<'a> Policy<'a> for ChangeEmailPolicy<'a> {
    // ユーザーは自分自身のメールアドレスを変更できる
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        let target = self.target;

        if ctx.actor_id == target.id() {
            Ok(()) // ユーザーは自分自身のメールアドレスを変更可能
        } else {
            Err(AuthorizationError::Forbidden) // その他のケースは拒否
        }
    }
}
