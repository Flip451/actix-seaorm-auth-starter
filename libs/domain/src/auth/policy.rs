use uuid::Uuid;

use crate::user::{User, UserRole};

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

// 認可サービス（ポリシーの管理） [5]
pub struct AuthorizationService;

impl AuthorizationService {
    pub fn can(
        actor_id: Uuid,
        actor_role: &UserRole,
        action: UserAction,
    ) -> Result<(), AuthorizationError> {
        match (actor_role, action) {
            // 管理者は自分以外の非管理者ユーザーを停止できる
            (UserRole::Admin, UserAction::SuspendUser { target }) => {
                // ターゲットユーザーが自分自身でないことを確認
                if actor_id == target.id() {
                    return Err(AuthorizationError::CannotSuspendSelf);
                }
                // ターゲットユーザーが管理者でないことを確認
                if target.role() == &UserRole::Admin {
                    return Err(AuthorizationError::CannotSuspendAdmin);
                }
                Ok(())
            }

            // 管理者は自身以外のユーザーのロックを解除できる
            (UserRole::Admin, UserAction::UnlockUser { target }) => {
                if actor_id == target.id() {
                    return Err(AuthorizationError::CannotUnlockSelf);
                }
                Ok(())
            }

            // 管理者は任意のユーザーを退会させることができる
            (UserRole::Admin, UserAction::DeactivateUser { .. }) => Ok(()),

            // ユーザーは自分自身を退会させることができる
            (UserRole::User, UserAction::DeactivateUser { target }) => {
                if actor_id == target.id() {
                    Ok(())
                } else {
                    Err(AuthorizationError::Forbidden)
                }
            }

            // 管理者は任意のユーザーを利用再開できる
            (UserRole::Admin, UserAction::ActivateUser { .. }) => Ok(()),

            // ユーザーは自分自身を利用再開できる
            (UserRole::User, UserAction::ActivateUser { target }) => {
                if actor_id == target.id() {
                    Ok(())
                } else {
                    Err(AuthorizationError::Forbidden)
                }
            }

            // 管理者は任意のユーザーを管理者に昇格できる
            (UserRole::Admin, UserAction::PromoteToAdmin { .. }) => Ok(()),

            // 管理者はユーザー一覧を取得できる
            (UserRole::Admin, UserAction::ListUsers) => Ok(()),

            // 管理者は任意のユーザーのプロフィールを閲覧できる
            (UserRole::Admin, UserAction::ViewProfile { .. }) => Ok(()),

            // ユーザーは自分自身のプロフィールを閲覧できる
            (UserRole::User, UserAction::ViewProfile { target }) => {
                if actor_id == target.id() {
                    Ok(())
                } else {
                    Err(AuthorizationError::Forbidden)
                }
            }

            // ユーザーは自分自身のプロフィールを更新できる
            (UserRole::User, UserAction::UpdateProfile { target }) => {
                if actor_id == target.id() {
                    Ok(())
                } else {
                    Err(AuthorizationError::Forbidden)
                }
            }

            // ユーザーは自分自身のメールアドレスを変更できる
            (UserRole::User, UserAction::ChangeEmail { target }) => {
                if actor_id == target.id() {
                    Ok(())
                } else {
                    Err(AuthorizationError::Forbidden)
                }
            }

            // デフォルトは拒否
            _ => Err(AuthorizationError::Forbidden),
        }
    }
}
