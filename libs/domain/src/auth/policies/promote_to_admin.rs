use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{User, UserRole},
};

#[derive(Clone, Copy)]
pub struct PromoteToAdminPayload<'a> {
    pub target: &'a User,
}

pub struct PromoteToAdminPolicy<'a>(PromoteToAdminPayload<'a>);

impl<'a> PromoteToAdminPolicy<'a> {
    pub fn new(payload: PromoteToAdminPayload<'a>) -> Self {
        Self(payload)
    }
}

impl<'a> Policy<'a> for PromoteToAdminPolicy<'a> {
    // 管理者は任意のユーザーを管理者に昇格できる
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        let _target = self.0.target;

        match ctx.actor_role {
            UserRole::Admin => Ok(()),               // 管理者は昇格可能
            _ => Err(AuthorizationError::Forbidden), // その他の役割は拒否
        }
    }
}
