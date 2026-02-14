use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{UserId, UserRole},
};

#[derive(Clone, Copy)]
pub struct PromoteToAdminPayload {
    pub target_id: UserId,
}

pub struct PromoteToAdminPolicy(PromoteToAdminPayload);

impl PromoteToAdminPolicy {
    pub fn new(payload: PromoteToAdminPayload) -> Self {
        Self(payload)
    }
}

impl Policy for PromoteToAdminPolicy {
    // 管理者は任意のユーザーを管理者に昇格できる
    fn check(&self, ctx: &AuthorizationContext) -> Result<(), AuthorizationError> {
        let _target_id = self.0.target_id;

        match ctx.actor_role {
            UserRole::Admin => Ok(()),               // 管理者は昇格可能
            _ => Err(AuthorizationError::Forbidden), // その他の役割は拒否
        }
    }
}
