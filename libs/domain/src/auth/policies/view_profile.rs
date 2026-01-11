use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{User, UserRole},
};

pub struct ViewProfilePolicy<'a> {
    pub target: &'a User,
}

impl<'a> Policy<'a> for ViewProfilePolicy<'a> {
    // 管理者は任意のユーザーのプロフィールを閲覧できる
    // ユーザーは自分自身のプロフィールを閲覧できる
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        let target = self.target;

        match ctx.actor_role {
            UserRole::Admin => Ok(()), // 管理者はプロフィール閲覧可能
            UserRole::User => {
                if ctx.actor_id == target.id() {
                    Ok(()) // ユーザーは自分自身のプロフィール閲覧可能
                } else {
                    Err(AuthorizationError::Forbidden) // その他のケースは拒否
                }
            }
        }
    }
}
