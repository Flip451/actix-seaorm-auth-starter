use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{User, UserRole},
};

#[derive(Clone, Copy)]
pub struct UpdateProfilePayload<'a> {
    pub target: &'a User,
}

pub struct UpdateProfilePolicy<'a>(UpdateProfilePayload<'a>);

impl<'a> UpdateProfilePolicy<'a> {
    pub fn new(payload: UpdateProfilePayload<'a>) -> Self {
        Self(payload)
    }
}

impl<'a> Policy<'a> for UpdateProfilePolicy<'a> {
    // 管理者は任意のユーザーのプロフィールを更新できる
    // ユーザーは自分自身のプロフィールを更新できる
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        let target = self.0.target;

        match ctx.actor_role {
            UserRole::Admin => Ok(()), // 管理者は任意のユーザーのプロフィールを更新可能
            UserRole::User => {
                if ctx.actor_id == target.id() {
                    Ok(()) // ユーザーは自分自身のプロフィールを更新可能
                } else {
                    Err(AuthorizationError::Forbidden) // その他のケースは拒否
                }
            }
        }
    }
}
