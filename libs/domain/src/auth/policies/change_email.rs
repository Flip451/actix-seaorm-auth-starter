use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{User, UserRole},
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
    // 管理者は任意のユーザーのメールアドレスを変更できる
    // ユーザーは自分自身のメールアドレスを変更できる
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        let target = self.0.target;

        match ctx.actor_role {
            UserRole::Admin => Ok(()), // 管理者は任意のユーザーのメールアドレスを変更可能
            UserRole::User => {
                if ctx.actor_id == target.id() {
                    Ok(()) // ユーザーは自分自身のメールアドレスを変更可能
                } else {
                    Err(AuthorizationError::Forbidden) // その他のケースは拒否
                }
            }
        }
    }
}
