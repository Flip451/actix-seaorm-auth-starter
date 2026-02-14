use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{UserId, UserRole},
};

#[derive(Clone, Copy)]
pub struct SuspendUserPayload {
    pub target_id: UserId,
    pub target_role: UserRole,
}

pub struct SuspendUserPolicy(SuspendUserPayload);

impl SuspendUserPolicy {
    pub fn new(payload: SuspendUserPayload) -> Self {
        Self(payload)
    }
}

impl Policy for SuspendUserPolicy {
    // 管理者は自分以外の非管理者ユーザーを停止できる
    fn check(&self, ctx: &AuthorizationContext) -> Result<(), AuthorizationError> {
        let target_id = self.0.target_id;
        let target_role = self.0.target_role;
        // 自分自身を利用停止にすることはできない
        if ctx.actor_id == target_id {
            return Err(AuthorizationError::CannotSuspendSelf);
        }
        // 管理者を管理者が停止することはできない
        if target_role == UserRole::Admin {
            return Err(AuthorizationError::CannotSuspendAdmin);
        }
        // ここでさらに詳細な権限チェックを行うことができます
        match ctx.actor_role {
            UserRole::Admin => Ok(()),               // 管理者は利用停止可能
            _ => Err(AuthorizationError::Forbidden), // その他の役割は拒否
        }
    }
}
