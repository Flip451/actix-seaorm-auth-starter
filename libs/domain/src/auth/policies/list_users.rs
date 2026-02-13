use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::UserRole,
};

#[derive(Clone, Copy)]
pub struct ListUsersPayload;

pub struct ListUsersPolicy(ListUsersPayload);

impl ListUsersPolicy {
    pub fn new(payload: ListUsersPayload) -> Self {
        Self(payload)
    }
}

impl<'a> Policy<'a> for ListUsersPolicy {
    // 管理者のみがユーザー一覧を取得できる
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        match ctx.actor_role {
            UserRole::Admin => Ok(()), // 管理者はユーザー一覧を取得可能
            UserRole::User => Err(AuthorizationError::Forbidden), // その他のケースは拒否
        }
    }
}
