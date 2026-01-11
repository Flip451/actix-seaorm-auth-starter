use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{User, UserRole},
};

#[derive(Clone, Copy)]
pub struct ActivateUserPayload<'a> {
    pub target: &'a User,
}

pub struct ActivateUserPolicy<'a>(ActivateUserPayload<'a>);

impl<'a> ActivateUserPolicy<'a> {
    pub fn new(payload: ActivateUserPayload<'a>) -> Self {
        Self(payload)
    }
}

impl<'a> Policy<'a> for ActivateUserPolicy<'a> {
    // 管理者は任意のユーザーを利用再開できる
    // ユーザーは自分自身を利用再開できる
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        let target = self.0.target;

        match ctx.actor_role {
            UserRole::Admin => Ok(()), // 管理者は利用再開可能
            UserRole::User => {
                if ctx.actor_id == target.id() {
                    Ok(()) // ユーザーは自分自身を利用再開可能
                } else {
                    Err(AuthorizationError::Forbidden) // その他のケースは拒否
                }
            }
        }
    }
}
