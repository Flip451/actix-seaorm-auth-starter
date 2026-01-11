use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{User, UserRole},
};

#[derive(Clone, Copy)]
pub struct DeactivateUserPayload<'a> {
    pub target: &'a User,
}

pub struct DeactivateUserPolicy<'a>(DeactivateUserPayload<'a>);

impl<'a> DeactivateUserPolicy<'a> {
    pub fn new(payload: DeactivateUserPayload<'a>) -> Self {
        Self(payload)
    }
}

impl<'a> Policy<'a> for DeactivateUserPolicy<'a> {
    // ユーザーは自分自身を退会できる。管理者は任意ののユーザーを退会させることができる。
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        let target = self.0.target;

        match ctx.actor_role {
            UserRole::Admin => Ok(()), // 管理者は他のユーザーを退会可能
            UserRole::User => {
                if ctx.actor_id == target.id() {
                    Ok(()) // ユーザーは自分自身を退会可能
                } else {
                    Err(AuthorizationError::Forbidden) // その他のケースは拒否
                }
            }
        }
    }
}
