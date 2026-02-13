use crate::auth::policy::{AuthorizationContext, AuthorizationError, Policy};

#[derive(Clone, Copy)]
pub struct ListUsersPayload;

pub struct ListUsersPolicy(ListUsersPayload);

impl ListUsersPolicy {
    pub fn new(payload: ListUsersPayload) -> Self {
        Self(payload)
    }
}

impl<'a> Policy<'a> for ListUsersPolicy {
    // 任意のログイン済みユーザーがアクセス可能とする
    fn check(&self, _ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
        Ok(())
    }
}
