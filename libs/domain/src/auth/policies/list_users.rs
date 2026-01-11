use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::UserRole,
};

pub struct ListUsersPolicy;

impl<'a> Policy<'a> for ListUsersPolicy {
    // 管理者のみがユーザー一覧を閲覧できる
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        match ctx.actor_role {
            UserRole::Admin => Ok(()),               // 管理者は閲覧可能
            _ => Err(AuthorizationError::Forbidden), // その他の役割は拒否
        }
    }
}
