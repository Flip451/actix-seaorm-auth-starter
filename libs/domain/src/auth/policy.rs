use uuid::Uuid;

use crate::{
    auth::policies::{
        activate_user::{ActivateUserPayload, ActivateUserPolicy},
        change_email::{ChangeEmailPayload, ChangeEmailPolicy},
        deactivate_user::{DeactivateUserPayload, DeactivateUserPolicy},
        list_users::{ListUsersPayload, ListUsersPolicy},
        promote_to_admin::{PromoteToAdminPayload, PromoteToAdminPolicy},
        suspend_user::{SuspendUserPayload, SuspendUserPolicy},
        unlock_user::{UnlockUserPayload, UnlockUserPolicy},
        update_profile::{UpdateProfilePayload, UpdateProfilePolicy},
        view_profile::{ViewProfilePayload, ViewProfilePolicy},
    },
    user::UserRole,
};

// 操作（アクション）を定義 [4]
#[derive(Clone, Copy)]
pub enum UserAction<'a> {
    SuspendUser(SuspendUserPayload<'a>),       // 利用停止
    UnlockUser(UnlockUserPayload<'a>),         // ロック解除
    DeactivateUser(DeactivateUserPayload<'a>), // 退会
    ActivateUser(ActivateUserPayload<'a>),     // 利用再開
    PromoteToAdmin(PromoteToAdminPayload<'a>), // 管理者への昇格
    ListUsers(ListUsersPayload),               // ユーザー一覧の取得
    ViewProfile(ViewProfilePayload<'a>),       // プロフィール閲覧
    UpdateProfile(UpdateProfilePayload<'a>),   // プロフィール更新
    ChangeEmail(ChangeEmailPayload<'a>),       // メールアドレス変更
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
            UserAction::SuspendUser(payload) => Box::new(SuspendUserPolicy::new(payload)),
            UserAction::UnlockUser(payload) => Box::new(UnlockUserPolicy::new(payload)),
            UserAction::DeactivateUser(payload) => Box::new(DeactivateUserPolicy::new(payload)),
            UserAction::ActivateUser(payload) => Box::new(ActivateUserPolicy::new(payload)),
            UserAction::PromoteToAdmin(payload) => Box::new(PromoteToAdminPolicy::new(payload)),
            UserAction::ListUsers(payload) => Box::new(ListUsersPolicy::new(payload)),
            UserAction::ViewProfile(payload) => Box::new(ViewProfilePolicy::new(payload)),
            UserAction::UpdateProfile(payload) => Box::new(UpdateProfilePolicy::new(payload)),
            UserAction::ChangeEmail(payload) => Box::new(ChangeEmailPolicy::new(payload)),
        };

        policy.check(&ctx)
    }
}
