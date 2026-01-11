use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{User, UserRole},
};

pub struct SuspendUserPolicy<'a> {
    pub target: &'a User,
}

impl<'a> Policy<'a> for SuspendUserPolicy<'a> {
    // 管理者は自分以外の非管理者ユーザーを停止できる
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        let target = self.target;

        // 自分自身を利用停止にすることはできない
        if ctx.actor_id == target.id() {
            return Err(AuthorizationError::CannotSuspendSelf);
        }
        // 管理者を管理者が停止することはできない
        if target.role() == UserRole::Admin {
            return Err(AuthorizationError::CannotSuspendAdmin);
        }
        // ここでさらに詳細な権限チェックを行うことができます
        match ctx.actor_role {
            UserRole::Admin => Ok(()),               // 管理者は利用停止可能
            _ => Err(AuthorizationError::Forbidden), // その他の役割は拒否
        }
    }
}
