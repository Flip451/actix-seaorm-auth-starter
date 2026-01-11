use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{User, UserRole},
};

pub struct PromoteToAdminPolicy<'a> {
    pub target: &'a User,
}

impl<'a> Policy<'a> for PromoteToAdminPolicy<'a> {
    // 管理者は任意のユーザーを管理者に昇格できる
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        match ctx.actor_role {
            UserRole::Admin => Ok(()),               // 管理者は昇格可能
            _ => Err(AuthorizationError::Forbidden), // その他の役割は拒否
        }
    }
}
