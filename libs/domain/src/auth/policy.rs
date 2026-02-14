use crate::{
    auth::policies::{
        activate_user::{ActivateUserPayload, ActivateUserPolicy},
        change_email::{ChangeEmailPayload, ChangeEmailPolicy},
        deactivate_user::{DeactivateUserPayload, DeactivateUserPolicy},
        find_user_by_id_for_suspend::{
            FindUserByIdForSuspendPayload, FindUserByIdForSuspendPolicy,
        },
        list_users::{ListUsersPayload, ListUsersPolicy},
        promote_to_admin::{PromoteToAdminPayload, PromoteToAdminPolicy},
        suspend_user::{SuspendUserPayload, SuspendUserPolicy},
        unlock_user::{UnlockUserPayload, UnlockUserPolicy},
        update_profile::{UpdateProfilePayload, UpdateProfilePolicy},
        view_detailed_profile::{ViewDetailedProfilePayload, ViewDetailedProfilePolicy},
        view_public_profile::{ViewPublicProfilePayload, ViewPublicProfilePolicy},
    },
    user::{UserId, UserRole},
};

// 操作（アクション）を定義 [4]
#[derive(Clone, Copy)]
pub enum UserAction {
    SuspendUser(SuspendUserPayload),                       // 利用停止
    UnlockUser(UnlockUserPayload),                         // ロック解除
    DeactivateUser(DeactivateUserPayload),                 // 退会
    ActivateUser(ActivateUserPayload),                     // 利用再開
    PromoteToAdmin(PromoteToAdminPayload),                 // 管理者への昇格
    ListUsers(ListUsersPayload),                           // ユーザー一覧の取得
    ViewPublicProfile(ViewPublicProfilePayload),           // プロフィール閲覧
    ViewDetailedProfile(ViewDetailedProfilePayload),       // 詳細プロフィール閲覧
    FindUserByIdForSuspend(FindUserByIdForSuspendPayload), // ユーザーIDによるユーザー検索
    UpdateProfile(UpdateProfilePayload),                   // プロフィール更新
    ChangeEmail(ChangeEmailPayload),                       // メールアドレス変更
}

pub struct AuthorizationContext {
    pub actor_id: UserId,
    pub actor_role: UserRole,
    pub action: UserAction,
}

// 認可エラーの定義
// TODO: #38 で整理
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

impl AuthorizationError {
    pub fn message_for_client(&self) -> &'static str {
        match self {
            AuthorizationError::Forbidden => "権限がありません",
            AuthorizationError::CannotSuspendSelf => "自分自身を利用停止にすることはできません",
            AuthorizationError::CannotUnlockSelf => "自分自身のロック解除はできません",
            AuthorizationError::CannotSuspendAdmin => "管理者を管理者が停止することはできません",
        }
    }
}

pub trait Policy {
    fn check(&self, ctx: &AuthorizationContext) -> Result<(), AuthorizationError>;
}

pub trait Actor {
    fn actor_id(&self) -> UserId;
    fn actor_role(&self) -> UserRole;
}

// 認可サービス（ポリシーの管理）
pub struct AuthorizationService;

impl AuthorizationService {
    pub fn can(actor: &impl Actor, action: UserAction) -> Result<(), AuthorizationError> {
        let actor_id = actor.actor_id();
        let actor_role = actor.actor_role();

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
            UserAction::ViewDetailedProfile(payload) => {
                Box::new(ViewDetailedProfilePolicy::new(payload))
            }
            UserAction::ViewPublicProfile(payload) => {
                Box::new(ViewPublicProfilePolicy::new(payload))
            }
            UserAction::FindUserByIdForSuspend(find_user_by_id_for_update_payload) => Box::new(
                FindUserByIdForSuspendPolicy::new(find_user_by_id_for_update_payload),
            ),
            UserAction::UpdateProfile(payload) => Box::new(UpdateProfilePolicy::new(payload)),
            UserAction::ChangeEmail(payload) => Box::new(ChangeEmailPolicy::new(payload)),
        };

        policy.check(&ctx)
    }
}
