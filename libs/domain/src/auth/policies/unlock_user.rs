use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{UserId, UserRole},
};

#[derive(Clone, Copy)]
pub struct UnlockUserPayload {
    pub target_id: UserId,
}

pub struct UnlockUserPolicy(UnlockUserPayload);

impl UnlockUserPolicy {
    pub fn new(payload: UnlockUserPayload) -> Self {
        Self(payload)
    }
}

impl Policy for UnlockUserPolicy {
    // 管理者は自分以外のユーザーのロックを解除できる
    fn check(&self, ctx: &AuthorizationContext) -> Result<(), AuthorizationError> {
        let target_id = self.0.target_id;

        // 自分自身を利用再開にすることはできない
        if ctx.actor_id == target_id {
            return Err(AuthorizationError::CannotUnlockSelf);
        }

        // ここでさらに詳細な権限チェックを行うことができます
        match ctx.actor_role {
            UserRole::Admin => Ok(()),               // 管理者は利用再開可能
            _ => Err(AuthorizationError::Forbidden), // その他の役割は拒否
        }
    }
}
