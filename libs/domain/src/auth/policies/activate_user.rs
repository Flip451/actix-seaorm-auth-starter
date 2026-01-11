use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{User, UserRole},
};

pub struct ActivateUserPolicy<'a> {
    pub target: &'a User,
}

impl<'a> Policy<'a> for ActivateUserPolicy<'a> {
    // 管理者は任意のユーザーを利用再開できる
    // ユーザーは自分自身を利用再開できる
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        let target = self.target;

        match ctx.actor_role {
            UserRole::Admin => Ok(()), // 管理者は利用再開可能
            UserRole::User => {
                if ctx.actor_id == target.id() {
                    Ok(()) // ユーザーは自分自身を利用再開可能
                } else {
                    Err(AuthorizationError::Forbidden) // その他のケースは拒否
                }
            }
        }
    }
}
