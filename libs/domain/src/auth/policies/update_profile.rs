use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{UserId, UserRole},
};

#[derive(Clone, Copy)]
pub struct UpdateProfilePayload {
    pub target_id: UserId,
}

pub struct UpdateProfilePolicy(UpdateProfilePayload);

impl UpdateProfilePolicy {
    pub fn new(payload: UpdateProfilePayload) -> Self {
        Self(payload)
    }
}

impl Policy for UpdateProfilePolicy {
    // 管理者は任意のユーザーのプロフィールを更新できる
    // ユーザーは自分自身のプロフィールを更新できる
    fn check(&self, ctx: &AuthorizationContext) -> Result<(), AuthorizationError> {
        let target_id = self.0.target_id;

        match ctx.actor_role {
            UserRole::Admin => Ok(()), // 管理者は任意のユーザーのプロフィールを更新可能
            UserRole::User => {
                if ctx.actor_id == target_id {
                    Ok(()) // ユーザーは自分自身のプロフィールを更新可能
                } else {
                    Err(AuthorizationError::Forbidden) // その他のケースは拒否
                }
            }
        }
    }
}
