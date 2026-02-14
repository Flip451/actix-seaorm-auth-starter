use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{UserId, UserRole},
};

#[derive(Clone, Copy)]
pub struct ViewDetailedProfilePayload {
    pub target_id: UserId,
}

pub struct ViewDetailedProfilePolicy(ViewDetailedProfilePayload);

impl ViewDetailedProfilePolicy {
    pub fn new(payload: ViewDetailedProfilePayload) -> Self {
        Self(payload)
    }
}

impl Policy for ViewDetailedProfilePolicy {
    // 管理者は任意のユーザーの詳細プロフィールを閲覧できる
    // 任意のログイン済みユーザーは自分自身の詳細プロフィールを閲覧できる
    fn check(&self, ctx: &AuthorizationContext) -> Result<(), AuthorizationError> {
        let target_id = self.0.target_id;

        match ctx.actor_role {
            UserRole::Admin => Ok(()), // 管理者は全ての詳細プロフィールを閲覧可能
            UserRole::User => {
                if ctx.actor_id == target_id {
                    Ok(()) // 自分自身の詳細プロフィールは閲覧可能
                } else {
                    Err(AuthorizationError::Forbidden) // その他のケースは拒否
                }
            }
        }
    }
}
