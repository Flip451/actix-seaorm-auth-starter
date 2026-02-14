use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{UserId, UserRole},
};

#[derive(Clone, Copy)]
pub struct FindUserByIdForSuspendPayload {
    pub target_id: UserId,
}

pub struct FindUserByIdForSuspendPolicy(FindUserByIdForSuspendPayload);

impl FindUserByIdForSuspendPolicy {
    pub fn new(payload: FindUserByIdForSuspendPayload) -> Self {
        Self(payload)
    }
}

impl Policy for FindUserByIdForSuspendPolicy {
    // 管理者は任意のユーザーをIDで検索できる
    // 一般ユーザーは自分自身のみ検索可能
    fn check(&self, ctx: &AuthorizationContext) -> Result<(), AuthorizationError> {
        let target_id = self.0.target_id;

        match ctx.actor_role {
            UserRole::Admin => Ok(()), // 管理者は全てのユーザーを検索可能
            UserRole::User => {
                if ctx.actor_id == target_id {
                    Ok(()) // 自分自身は検索可能
                } else {
                    Err(AuthorizationError::Forbidden) // その他のケースは拒否
                }
            }
        }
    }
}
