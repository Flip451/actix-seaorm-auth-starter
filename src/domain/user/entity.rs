use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

use crate::domain::user::{Email, value_objects::email::EmailTrait};

use super::{
    EmailVerifier, HashedPassword, UnverifiedEmail, UserDomainError, UserRole, VerifiedEmail,
};

pub struct User {
    id: Uuid,
    username: String,
    password: HashedPassword,
    role: UserRole,
    state: UserState,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>,
}

impl User {
    pub fn new(
        id: Uuid,
        username: String,
        email: String,
        password: HashedPassword,
    ) -> Result<Self, UserDomainError> {
        Ok(Self {
            id,
            username,
            password,
            role: UserRole::default(),
            state: UserState::PendingVerification {
                email: UnverifiedEmail::new(&email)?,
            },
            created_at: DateTime::<FixedOffset>::from(chrono::offset::Utc::now()),
            updated_at: DateTime::<FixedOffset>::from(chrono::offset::Utc::now()),
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn role(&self) -> &UserRole {
        &self.role
    }

    pub fn email(&self) -> Email {
        match &self.state {
            UserState::Active { email } => Email::Verified(email.clone()),
            UserState::SuspendedByAdmin { email } => Email::Unverified(email.clone()),
            UserState::DeactivatedByUser { email } => Email::Unverified(email.clone()),
            UserState::PendingVerification { email } => Email::Unverified(email.clone()),
            UserState::ActiveWithUnverifiedEmail { email } => Email::Unverified(email.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserState {
    Active { email: VerifiedEmail },                      // 通常
    SuspendedByAdmin { email: UnverifiedEmail },          // 管理者による停止(メール未認証にする)
    DeactivatedByUser { email: UnverifiedEmail },         // ユーザーによる退会
    PendingVerification { email: UnverifiedEmail },       // メール未認証
    ActiveWithUnverifiedEmail { email: UnverifiedEmail }, // メール更新後の認証待ち
}

// TODO: 状態遷移のルールを実装する
impl User {
    pub fn verify_email<V: EmailVerifier>(&mut self, verifier: &V) -> Result<(), UserDomainError> {
        match self.state {
            UserState::Active { .. } => Ok(()),
            UserState::SuspendedByAdmin { .. } => Err(UserDomainError::AlreadySuspended {
                from: self.state.clone(),
            }),
            UserState::DeactivatedByUser { .. } => Err(UserDomainError::AlreadyDeactivated {
                from: self.state.clone(),
            }),
            UserState::PendingVerification { ref email } => {
                self.state = UserState::Active {
                    email: verifier.verify(email)?,
                };
                Ok(())
            }
            UserState::ActiveWithUnverifiedEmail { ref email } => {
                self.state = UserState::Active {
                    email: verifier.verify(email)?,
                };
                Ok(())
            }
        }
    }

    pub fn change_email(&mut self, new_email: UnverifiedEmail) -> Result<(), UserDomainError> {
        match self.state {
            UserState::Active { .. } => {
                self.state = UserState::ActiveWithUnverifiedEmail { email: new_email };
                Ok(())
            }
            UserState::SuspendedByAdmin { .. } => Err(UserDomainError::AlreadySuspended {
                from: self.state.clone(),
            }),
            UserState::DeactivatedByUser { .. } => Err(UserDomainError::AlreadyDeactivated {
                from: self.state.clone(),
            }),
            UserState::PendingVerification { .. } => {
                self.state = UserState::PendingVerification { email: new_email };
                Ok(())
            }
            UserState::ActiveWithUnverifiedEmail { .. } => {
                self.state = UserState::ActiveWithUnverifiedEmail { email: new_email };
                Ok(())
            }
        }
    }

    pub fn suspend(&mut self) -> Result<(), UserDomainError> {
        match self.state {
            UserState::Active { ref email } => {
                self.state = UserState::SuspendedByAdmin {
                    email: email.unverify(),
                };
                Ok(())
            }
            UserState::SuspendedByAdmin { .. } => Ok(()), // すでに停止中なので何もしない
            UserState::DeactivatedByUser { ref email } => {
                self.state = UserState::SuspendedByAdmin {
                    email: email.clone(),
                };
                Ok(())
            }
            UserState::PendingVerification { ref email } => {
                self.state = UserState::SuspendedByAdmin {
                    email: email.clone(),
                };
                Ok(())
            }
            UserState::ActiveWithUnverifiedEmail { ref email } => {
                self.state = UserState::SuspendedByAdmin {
                    email: email.clone(),
                };
                Ok(())
            }
        }
    }

    pub fn deactivate(&mut self) -> Result<(), UserDomainError> {
        match self.state {
            UserState::Active { ref email } => {
                self.state = UserState::DeactivatedByUser {
                    email: email.unverify(),
                };
                Ok(())
            }
            UserState::SuspendedByAdmin { .. } => Err(UserDomainError::AlreadySuspended {
                from: self.state.clone(),
            }),
            UserState::DeactivatedByUser { .. } => Ok(()), // すでに退会済みなので何もしない
            UserState::PendingVerification { .. } => Err(UserDomainError::NotVerified {
                from: self.state.clone(),
            }),
            UserState::ActiveWithUnverifiedEmail { .. } => Err(UserDomainError::NotVerified {
                from: self.state.clone(),
            }),
        }
    }

    pub fn activate<V: EmailVerifier>(&mut self) -> Result<(), UserDomainError> {
        match self.state {
            UserState::Active { .. } => Ok(()), // すでにアクティブなので何もしない
            UserState::SuspendedByAdmin { .. } => Err(UserDomainError::AlreadySuspended {
                from: self.state.clone(),
            }),
            UserState::DeactivatedByUser { ref email } => {
                self.state = UserState::ActiveWithUnverifiedEmail {
                    email: email.clone(),
                };
                Ok(())
            }
            UserState::PendingVerification { .. } => Err(UserDomainError::NotVerified {
                from: self.state.clone(),
            }),
            UserState::ActiveWithUnverifiedEmail { .. } => Err(UserDomainError::NotVerified {
                from: self.state.clone(),
            }),
        }
    }

    pub fn unlock_suspension(&mut self) -> Result<(), UserDomainError> {
        match self.state {
            UserState::Active { .. }
            | UserState::DeactivatedByUser { .. }
            | UserState::PendingVerification { .. }
            | UserState::ActiveWithUnverifiedEmail { .. } => Err(UserDomainError::NotSuspended {
                from: self.state.clone(),
            }),
            UserState::SuspendedByAdmin { ref email } => {
                self.state = UserState::ActiveWithUnverifiedEmail {
                    email: email.clone(),
                };
                Ok(())
            }
        }
    }
}
