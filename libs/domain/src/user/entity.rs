use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    shared::outbox::OutboxEvent,
    user::{Email, UserEvent, UserStateTransitionError},
};

use super::{
    EmailVerifier, HashedPassword, UnverifiedEmail, UserDomainError, UserRole, VerifiedEmail,
};

pub struct User {
    id: Uuid,
    username: String,
    password: HashedPassword,
    role: UserRole,
    state: UserState,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    events: Vec<UserEvent>,
}

impl User {
    // 新規ユーザー作成のためのコンストラクタ
    pub fn new(
        username: String,
        email: UnverifiedEmail,
        password: HashedPassword,
    ) -> Result<Self, UserDomainError> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        Ok(Self {
            id,
            username,
            password,
            role: UserRole::User,
            state: UserState::PendingVerification {
                email: email.clone(),
            },
            created_at: now,
            updated_at: now,
            events: vec![UserEvent::UserCreated {
                user_id: id,
                email,
                registered_at: now,
            }],
        })
    }

    // 永続化処理されたユーザーを再構築するためのコンストラクタ
    pub fn reconstruct(
        id: Uuid,
        username: String,
        password: HashedPassword,
        role: UserRole,
        state: UserState,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, UserDomainError> {
        Ok(Self {
            id,
            username,
            password,
            role,
            state,
            created_at,
            updated_at,
            events: vec![],
        })
    }

    fn record_event(&mut self, event: UserEvent) {
        self.events.push(event);
    }

    pub fn pull_outbox_events(&mut self) -> Vec<OutboxEvent> {
        std::mem::take(&mut self.events)
            .into_iter()
            .map(|e| OutboxEvent::new(e.into()))
            .collect()
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &HashedPassword {
        &self.password
    }

    pub fn role(&self) -> UserRole {
        self.role
    }

    pub fn state(&self) -> &UserState {
        &self.state
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

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
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

impl User {
    pub fn change_username(&mut self, new_username: String) -> Result<(), UserDomainError> {
        self.username = new_username;

        self.record_event(UserEvent::UsernameChanged {
            user_id: self.id,
            new_username: self.username.clone(),
            changed_at: Utc::now(),
        });

        Ok(())
    }
}

// ユーザーの状態遷移に関するメソッド群
impl User {
    pub fn verify_email<V: EmailVerifier>(&mut self, verifier: &V) -> Result<(), UserDomainError> {
        match self.state {
            UserState::Active { .. } => {} // すでにアクティブなので何もしない
            UserState::SuspendedByAdmin { .. } => {
                Err(UserStateTransitionError::AlreadySuspended {
                    from: self.state.clone(),
                })?
            }
            UserState::DeactivatedByUser { .. } => {
                Err(UserStateTransitionError::AlreadyDeactivated {
                    from: self.state.clone(),
                })?
            }
            UserState::PendingVerification { ref email } => {
                self.state = UserState::Active {
                    email: verifier.verify(email)?,
                };
            }
            UserState::ActiveWithUnverifiedEmail { ref email } => {
                self.state = UserState::Active {
                    email: verifier.verify(email)?,
                };
            }
        }

        self.record_event(UserEvent::EmailVerified {
            user_id: self.id,
            verified_at: Utc::now(),
        });

        Ok(())
    }

    pub fn change_email(&mut self, new_email: UnverifiedEmail) -> Result<(), UserDomainError> {
        match self.state {
            UserState::Active { .. } => {
                self.state = UserState::ActiveWithUnverifiedEmail {
                    email: new_email.clone(),
                };
            }
            UserState::SuspendedByAdmin { .. } => {
                Err(UserStateTransitionError::AlreadySuspended {
                    from: self.state.clone(),
                })?
            }
            UserState::DeactivatedByUser { .. } => {
                Err(UserStateTransitionError::AlreadyDeactivated {
                    from: self.state.clone(),
                })?
            }
            UserState::PendingVerification { .. } => {
                self.state = UserState::PendingVerification {
                    email: new_email.clone(),
                };
            }
            UserState::ActiveWithUnverifiedEmail { .. } => {
                self.state = UserState::ActiveWithUnverifiedEmail {
                    email: new_email.clone(),
                };
            }
        }

        self.record_event(UserEvent::UserEmailChanged {
            user_id: self.id,
            new_email,
            changed_at: Utc::now(),
        });

        Ok(())
    }

    pub fn suspend(&mut self, reason: String) -> Result<(), UserDomainError> {
        match self.state {
            UserState::Active { ref email } => {
                self.state = UserState::SuspendedByAdmin {
                    email: email.unverify(),
                };
            }
            UserState::SuspendedByAdmin { .. } => {} // すでに停止中なので何もしない
            UserState::DeactivatedByUser { ref email } => {
                self.state = UserState::SuspendedByAdmin {
                    email: email.clone(),
                };
            }
            UserState::PendingVerification { ref email } => {
                self.state = UserState::SuspendedByAdmin {
                    email: email.clone(),
                };
            }
            UserState::ActiveWithUnverifiedEmail { ref email } => {
                self.state = UserState::SuspendedByAdmin {
                    email: email.clone(),
                };
            }
        }

        self.record_event(UserEvent::UserSuspended {
            user_id: self.id,
            reason,
            suspended_at: Utc::now(),
        });

        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<(), UserDomainError> {
        match self.state {
            UserState::Active { ref email } => {
                self.state = UserState::DeactivatedByUser {
                    email: email.unverify(),
                };
            }
            UserState::SuspendedByAdmin { .. } => {
                Err(UserStateTransitionError::AlreadySuspended {
                    from: self.state.clone(),
                })?
            }
            UserState::DeactivatedByUser { .. } => {} // すでに退会済みなので何もしない
            UserState::PendingVerification { .. } => Err(UserStateTransitionError::NotVerified {
                from: self.state.clone(),
            })?,
            UserState::ActiveWithUnverifiedEmail { .. } => {
                Err(UserStateTransitionError::NotVerified {
                    from: self.state.clone(),
                })?
            }
        }

        self.record_event(UserEvent::UserDeactivated {
            user_id: self.id,
            deactivated_at: Utc::now(),
        });

        Ok(())
    }

    pub fn activate<V: EmailVerifier>(&mut self) -> Result<(), UserDomainError> {
        match self.state {
            UserState::Active { .. } => {} // すでにアクティブなので何もしない
            UserState::SuspendedByAdmin { .. } => {
                Err(UserStateTransitionError::AlreadySuspended {
                    from: self.state.clone(),
                })?
            }
            UserState::DeactivatedByUser { ref email } => {
                self.state = UserState::ActiveWithUnverifiedEmail {
                    email: email.clone(),
                };
            }
            UserState::PendingVerification { .. } => Err(UserStateTransitionError::NotVerified {
                from: self.state.clone(),
            })?,
            UserState::ActiveWithUnverifiedEmail { .. } => {
                Err(UserStateTransitionError::NotVerified {
                    from: self.state.clone(),
                })?
            }
        }

        self.record_event(UserEvent::UserReactivated {
            user_id: self.id,
            reactivated_at: Utc::now(),
        });

        Ok(())
    }

    pub fn unlock_suspension(&mut self) -> Result<(), UserDomainError> {
        match self.state {
            UserState::Active { .. }
            | UserState::DeactivatedByUser { .. }
            | UserState::PendingVerification { .. }
            | UserState::ActiveWithUnverifiedEmail { .. } => {
                Err(UserStateTransitionError::NotSuspended {
                    from: self.state.clone(),
                })?
            }
            UserState::SuspendedByAdmin { ref email } => {
                self.state = UserState::ActiveWithUnverifiedEmail {
                    email: email.clone(),
                };
            }
        }

        self.record_event(UserEvent::UserUnlocked {
            user_id: self.id,
            unlocked_at: Utc::now(),
        });

        Ok(())
    }
}
