use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{UserId, UserRole},
};

#[derive(Clone, Copy)]
pub struct DeactivateUserPayload {
    pub target_id: UserId,
}

pub struct DeactivateUserPolicy(DeactivateUserPayload);

impl DeactivateUserPolicy {
    pub fn new(payload: DeactivateUserPayload) -> Self {
        Self(payload)
    }
}

impl Policy for DeactivateUserPolicy {
    // ユーザーは自分自身を退会できる。管理者は任意ののユーザーを退会させることができる。
    fn check(&self, ctx: &AuthorizationContext) -> Result<(), AuthorizationError> {
        let target_id = self.0.target_id;

        match ctx.actor_role {
            UserRole::Admin => Ok(()), // 管理者は他のユーザーを退会可能
            UserRole::User => {
                if ctx.actor_id == target_id {
                    Ok(()) // ユーザーは自分自身を退会可能
                } else {
                    Err(AuthorizationError::Forbidden) // その他のケースは拒否
                }
            }
        }
    }
}
