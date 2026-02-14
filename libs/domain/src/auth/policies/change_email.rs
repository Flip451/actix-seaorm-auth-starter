use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{UserId, UserRole},
};

#[derive(Clone, Copy)]
pub struct ChangeEmailPayload {
    pub target_id: UserId,
}

pub struct ChangeEmailPolicy(ChangeEmailPayload);

impl ChangeEmailPolicy {
    pub fn new(payload: ChangeEmailPayload) -> Self {
        Self(payload)
    }
}

impl Policy for ChangeEmailPolicy {
    // 管理者は任意のユーザーのメールアドレスを変更できる
    // ユーザーは自分自身のメールアドレスを変更できる
    fn check(&self, ctx: &AuthorizationContext) -> Result<(), AuthorizationError> {
        let target_id = self.0.target_id;

        match ctx.actor_role {
            UserRole::Admin => Ok(()), // 管理者は任意のユーザーのメールアドレスを変更可能
            UserRole::User => {
                if ctx.actor_id == target_id {
                    Ok(()) // ユーザーは自分自身のメールアドレスを変更可能
                } else {
                    Err(AuthorizationError::Forbidden) // その他のケースは拒否
                }
            }
        }
    }
}
