use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{UserId, UserRole},
};

#[derive(Clone, Copy)]
pub struct ActivateUserPayload {
    pub target_id: UserId,
}

pub struct ActivateUserPolicy(ActivateUserPayload);

impl ActivateUserPolicy {
    pub fn new(payload: ActivateUserPayload) -> Self {
        Self(payload)
    }
}

impl Policy for ActivateUserPolicy {
    // 管理者は任意のユーザーを利用再開できる
    // ユーザーは自分自身を利用再開できる
    fn check(&self, ctx: &AuthorizationContext) -> Result<(), AuthorizationError> {
        let target_id = self.0.target_id;

        match ctx.actor_role {
            UserRole::Admin => Ok(()), // 管理者は利用再開可能
            UserRole::User => {
                if ctx.actor_id == target_id {
                    Ok(()) // ユーザーは自分自身を利用再開可能
                } else {
                    Err(AuthorizationError::Forbidden) // その他のケースは拒否
                }
            }
        }
    }
}
