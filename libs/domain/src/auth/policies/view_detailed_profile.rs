use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{User, UserRole},
};

#[derive(Clone, Copy)]
pub struct ViewDetailedProfilePayload<'a> {
    pub target: &'a User,
}

pub struct ViewDetailedProfilePolicy<'a>(ViewDetailedProfilePayload<'a>);

impl<'a> ViewDetailedProfilePolicy<'a> {
    pub fn new(payload: ViewDetailedProfilePayload<'a>) -> Self {
        Self(payload)
    }
}

impl<'a> Policy<'a> for ViewDetailedProfilePolicy<'a> {
    // 管理者は任意のユーザーの詳細プロフィールを閲覧できる
    // 任意のログイン済みユーザーは自分自身の詳細プロフィールを閲覧できる
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        let target = self.0.target;

        match ctx.actor_role {
            UserRole::Admin => Ok(()), // 管理者は全ての詳細プロフィールを閲覧可能
            UserRole::User => {
                if ctx.actor_id == target.id() {
                    Ok(()) // 自分自身の詳細プロフィールは閲覧可能
                } else {
                    Err(AuthorizationError::Forbidden) // その他のケースは拒否
                }
            }
        }
    }
}
