use uuid::Uuid;

use crate::{
    auth::policies::{
        activate_user::ActivateUserPolicy, change_email::ChangeEmailPolicy,
        deactivate_user::DeactivateUserPolicy, list_users::ListUsersPolicy,
        promote_to_admin::PromoteToAdminPolicy, suspend_user::SuspendUserPolicy,
        unlock_user::UnlockUserPolicy, update_profile::UpdateProfilePolicy,
        view_profile::ViewProfilePolicy,
    },
    user::{User, UserRole},
};

// 操作（アクション）を定義 [4]
#[derive(Clone, Copy)]
pub enum UserAction<'a> {
    SuspendUser { target: &'a User },    // 利用停止
    UnlockUser { target: &'a User },     // ロック解除
    DeactivateUser { target: &'a User }, // 退会
    ActivateUser { target: &'a User },   // 利用再開
    PromoteToAdmin { target: &'a User }, // 管理者への昇格
    ListUsers,                           // ユーザー一覧の取得
    ViewProfile { target: &'a User },    // プロフィール閲覧
    UpdateProfile { target: &'a User },  // プロフィール更新
    ChangeEmail { target: &'a User },    // メールアドレス変更
}

pub struct AuthorizationContext<'a> {
    pub actor_id: Uuid,
    pub actor_role: UserRole,
    pub action: UserAction<'a>,
}

// 認可エラーの定義
#[derive(Debug, thiserror::Error)]
pub enum AuthorizationError {
    #[error("権限がありません")]
    Forbidden,
    #[error("自分自身を利用停止にすることはできません")]
    CannotSuspendSelf,
    #[error("自分自身のロック解除はできません")]
    CannotUnlockSelf,
    #[error("管理者を管理者が停止することはできません")]
    CannotSuspendAdmin,
}

pub trait Policy<'a> {
    fn check(&self, ctx: &AuthorizationContext<'a>) -> Result<(), AuthorizationError>;
}

// 認可サービス（ポリシーの管理） [5]
pub struct AuthorizationService;

impl AuthorizationService {
    pub fn can(
        actor_id: Uuid,
        actor_role: UserRole,
        action: UserAction,
    ) -> Result<(), AuthorizationError> {
        let ctx = AuthorizationContext {
            actor_id,
            actor_role,
            action,
        };

        let policy: Box<dyn Policy> = match action {
            UserAction::SuspendUser { target } => Box::new(SuspendUserPolicy { target }),
            UserAction::UnlockUser { target } => Box::new(UnlockUserPolicy { target }),
            UserAction::DeactivateUser { target } => Box::new(DeactivateUserPolicy { target }),
            UserAction::ActivateUser { target } => Box::new(ActivateUserPolicy { target }),
            UserAction::PromoteToAdmin { target } => Box::new(PromoteToAdminPolicy { target }),
            UserAction::ListUsers => Box::new(ListUsersPolicy),
            UserAction::ViewProfile { target } => Box::new(ViewProfilePolicy { target }),
            UserAction::UpdateProfile { target } => Box::new(UpdateProfilePolicy { target }),
            UserAction::ChangeEmail { target } => Box::new(ChangeEmailPolicy { target }),
        };

        policy.check(&ctx)
    }
}
