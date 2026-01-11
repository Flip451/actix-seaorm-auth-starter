use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{User, UserRole},
};

#[derive(Clone, Copy)]
pub struct UnlockUserPayload<'a> {
    pub target: &'a User,
}

pub struct UnlockUserPolicy<'a>(UnlockUserPayload<'a>);

impl<'a> UnlockUserPolicy<'a> {
    pub fn new(payload: UnlockUserPayload<'a>) -> Self {
        Self(payload)
    }
}

impl<'a> Policy<'a> for UnlockUserPolicy<'a> {
    // 管理者は自分以外のユーザーのロックを解除できる
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        let target = self.0.target;

        // 自分自身を利用再開にすることはできない
        if ctx.actor_id == target.id() {
            return Err(AuthorizationError::CannotUnlockSelf);
        }

        // ここでさらに詳細な権限チェックを行うことができます
        match ctx.actor_role {
            UserRole::Admin => Ok(()),               // 管理者は利用再開可能
            _ => Err(AuthorizationError::Forbidden), // その他の役割は拒否
        }
    }
}
