use crate::{
    auth::policy::{AuthorizationContext, AuthorizationError, Policy},
    user::{UserId, UserRole},
};

#[derive(Clone, Copy)]
pub struct FindUserByIdPayload {
    pub target_id: UserId,
}

pub struct FindUserByIdPolicy(FindUserByIdPayload);

impl FindUserByIdPolicy {
    pub fn new(payload: FindUserByIdPayload) -> Self {
        Self(payload)
    }
}

impl<'a> Policy<'a> for FindUserByIdPolicy {
    // 管理者は任意のユーザーをIDで検索できる
    // 一般ユーザーは自分自身のみ検索可能
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError> {
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
