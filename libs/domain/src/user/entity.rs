use chrono::{DateTime, Utc};
use strum::EnumString;

use crate::{
    shared::outbox_event::{EntityWithEvents, OutboxEvent},
    user::{
        Email, EmailTrait, UserEvent, UserId, UserReconstructionError, UserStateTransitionError,
        events::{
            UserCreatedEvent, UserDeactivatedEvent, UserEmailChangedEvent, UserEmailVerifiedEvent,
            UserReactivatedEvent, UserSuspendedEvent, UserUnlockedEvent, UsernameChangedEvent,
        },
        service::{UniqueEmail, UniqueUserInfo, UniqueUsername},
    },
};

use super::{
    EmailVerifier, HashedPassword, UnverifiedEmail, UserDomainError, UserRole, VerifiedEmail,
};

pub struct User {
    id: UserId,
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
    pub(crate) fn new(
        id: UserId,
        UniqueUserInfo { email, username }: UniqueUserInfo,
        password: HashedPassword,
    ) -> Result<Self, UserDomainError> {
        let now = Utc::now();
        Ok(Self {
            id,
            username: username.clone(),
            password,
            role: UserRole::User,
            state: UserState::PendingVerification {
                email: email.clone(),
            },
            created_at: now,
            updated_at: now,
            events: vec![UserEvent::Created(UserCreatedEvent {
                email,
                username,
                registered_at: now,
            })],
        })
    }

    // 永続化処理されたユーザーを再構築するためのコンストラクタ
    pub fn reconstruct(
        id: UserId,
        username: String,
        password: HashedPassword,
        role: UserRole,
        state_source: UserStateRaw,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, UserReconstructionError> {
        let state = state_source.try_into()?;

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

    fn pull_outbox_events(&mut self) -> Vec<OutboxEvent> {
        std::mem::take(&mut self.events)
            .into_iter()
            .map(|e| OutboxEvent::new(e.into()))
            .collect()
    }

    pub fn id(&self) -> UserId {
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
    // TODO: #70 で username を変更可能な State を定義する
    pub fn change_username(
        &mut self,
        UniqueUsername(new_username): UniqueUsername,
    ) -> Result<(), UserDomainError> {
        let old_username = self.username.clone();
        self.username = new_username;

        self.record_event(UserEvent::UsernameChanged(UsernameChangedEvent {
            old_username,
            new_username: self.username.clone(),
            email: self.email(),
            changed_at: Utc::now(),
        }));
        Ok(())
    }
}

// ユーザーの状態遷移に関するメソッド群
impl User {
    pub fn verify_email<V: EmailVerifier>(&mut self, verifier: &V) -> Result<(), UserDomainError> {
        let email = match self.state {
            UserState::Active { .. } => return Ok(()), // すでに検証済みなので何もしない
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
                let email = verifier.verify(email)?;
                self.state = UserState::Active {
                    email: email.clone(),
                };
                email
            }
            UserState::ActiveWithUnverifiedEmail { ref email } => {
                let email = verifier.verify(email)?;
                self.state = UserState::Active {
                    email: email.clone(),
                };
                email
            }
        };

        self.record_event(UserEvent::EmailVerified(UserEmailVerifiedEvent {
            email,
            username: self.username.clone(),
            verified_at: Utc::now(),
        }));

        Ok(())
    }

    pub fn change_email(
        &mut self,
        UniqueEmail(new_email): UniqueEmail,
    ) -> Result<(), UserDomainError> {
        if new_email.as_str() == self.email().as_str() {
            // 新しいメールアドレスが現在のものと同じ場合、何もしない
            return Ok(());
        }

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

        self.record_event(UserEvent::EmailChanged(UserEmailChangedEvent {
            new_email,
            username: self.username.clone(),
            changed_at: Utc::now(),
        }));
        Ok(())
    }

    pub fn suspend(&mut self, reason: String) -> Result<(), UserDomainError> {
        let email = match self.state {
            UserState::Active { ref email } => {
                let email = email.unverify();
                self.state = UserState::SuspendedByAdmin {
                    email: email.clone(),
                };
                email
            }
            UserState::SuspendedByAdmin { .. } => return Ok(()), // すでに停止中なので何もしない
            UserState::DeactivatedByUser { ref email } => {
                let email = email.clone();
                self.state = UserState::SuspendedByAdmin {
                    email: email.clone(),
                };
                email
            }
            UserState::PendingVerification { ref email } => {
                let email = email.clone();
                self.state = UserState::SuspendedByAdmin {
                    email: email.clone(),
                };
                email
            }
            UserState::ActiveWithUnverifiedEmail { ref email } => {
                let email = email.clone();
                self.state = UserState::SuspendedByAdmin {
                    email: email.clone(),
                };
                email
            }
        };

        self.record_event(UserEvent::Suspended(UserSuspendedEvent {
            username: self.username.clone(),
            email,
            reason,
            suspended_at: Utc::now(),
        }));

        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<(), UserDomainError> {
        let email = match self.state {
            UserState::Active { ref email } => {
                let email = email.unverify();
                self.state = UserState::DeactivatedByUser {
                    email: email.clone(),
                };
                email
            }
            UserState::SuspendedByAdmin { .. } => {
                Err(UserStateTransitionError::AlreadySuspended {
                    from: self.state.clone(),
                })?
            }
            UserState::DeactivatedByUser { .. } => return Ok(()), // すでに退会済みなので何もしない
            UserState::PendingVerification { .. } => Err(UserStateTransitionError::NotVerified {
                from: self.state.clone(),
            })?,
            UserState::ActiveWithUnverifiedEmail { .. } => {
                Err(UserStateTransitionError::NotVerified {
                    from: self.state.clone(),
                })?
            }
        };

        self.record_event(UserEvent::Deactivated(UserDeactivatedEvent {
            username: self.username.clone(),
            email,
            deactivated_at: Utc::now(),
        }));

        Ok(())
    }

    pub fn activate<V: EmailVerifier>(&mut self) -> Result<(), UserDomainError> {
        let email = match self.state {
            UserState::Active { .. } => return Ok(()), // すでにアクティブなので何もしない
            UserState::SuspendedByAdmin { .. } => {
                Err(UserStateTransitionError::AlreadySuspended {
                    from: self.state.clone(),
                })?
            }
            UserState::DeactivatedByUser { ref email } => {
                let email = email.clone();
                self.state = UserState::ActiveWithUnverifiedEmail {
                    email: email.clone(),
                };
                email
            }
            UserState::PendingVerification { .. } => Err(UserStateTransitionError::NotVerified {
                from: self.state.clone(),
            })?,
            UserState::ActiveWithUnverifiedEmail { .. } => {
                Err(UserStateTransitionError::NotVerified {
                    from: self.state.clone(),
                })?
            }
        };

        self.record_event(UserEvent::Reactivated(UserReactivatedEvent {
            username: self.username.clone(),
            email,
            reactivated_at: Utc::now(),
        }));

        Ok(())
    }

    pub fn unlock_suspension(&mut self) -> Result<(), UserDomainError> {
        let email = match self.state {
            UserState::Active { .. }
            | UserState::DeactivatedByUser { .. }
            | UserState::PendingVerification { .. }
            | UserState::ActiveWithUnverifiedEmail { .. } => {
                Err(UserStateTransitionError::NotSuspended {
                    from: self.state.clone(),
                })?
            }
            UserState::SuspendedByAdmin { ref email } => {
                let email = email.clone();
                self.state = UserState::ActiveWithUnverifiedEmail {
                    email: email.clone(),
                };
                email
            }
        };

        self.record_event(UserEvent::Unlocked(UserUnlockedEvent {
            username: self.username.clone(),
            email,
            unlocked_at: Utc::now(),
        }));

        Ok(())
    }
}

impl EntityWithEvents for User {
    fn pull_events(&mut self) -> Vec<OutboxEvent> {
        self.pull_outbox_events()
    }
}

pub struct UserStateRaw {
    pub status: String,
    pub email: String,
}

impl TryFrom<UserStateRaw> for UserState {
    type Error = UserReconstructionError;

    fn try_from(raw: UserStateRaw) -> Result<Self, Self::Error> {
        let kind = raw
            .status
            .parse::<UserStatusKind>()
            .map_err(|_| UserReconstructionError::InvalidStatus(raw.status.clone()))?;

        match kind {
            UserStatusKind::Active => Ok(UserState::Active {
                // ドメイン層なので VerifiedEmail::new を呼ぶのは責務の範囲内
                email: VerifiedEmail::new(&raw.email)
                    .map_err(|_| UserReconstructionError::InvalidEmail(raw.email))?,
            }),
            UserStatusKind::SuspendedByAdmin => Ok(UserState::SuspendedByAdmin {
                email: UnverifiedEmail::new(&raw.email)
                    .map_err(|_| UserReconstructionError::InvalidEmail(raw.email))?,
            }),
            UserStatusKind::DeactivatedByUser => Ok(UserState::DeactivatedByUser {
                email: UnverifiedEmail::new(&raw.email)
                    .map_err(|_| UserReconstructionError::InvalidEmail(raw.email))?,
            }),
            UserStatusKind::PendingVerification => Ok(UserState::PendingVerification {
                email: UnverifiedEmail::new(&raw.email)
                    .map_err(|_| UserReconstructionError::InvalidEmail(raw.email))?,
            }),
            UserStatusKind::ActiveWithUnverifiedEmail => Ok(UserState::ActiveWithUnverifiedEmail {
                email: UnverifiedEmail::new(&raw.email)
                    .map_err(|_| UserReconstructionError::InvalidEmail(raw.email))?,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq, strum::Display, EnumString, strum::IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
enum UserStatusKind {
    Active,
    SuspendedByAdmin,
    DeactivatedByUser,
    PendingVerification,
    ActiveWithUnverifiedEmail,
}

impl UserState {
    pub fn kind(&self) -> &'static str {
        match self {
            UserState::Active { .. } => UserStatusKind::Active,
            UserState::SuspendedByAdmin { .. } => UserStatusKind::SuspendedByAdmin,
            UserState::DeactivatedByUser { .. } => UserStatusKind::DeactivatedByUser,
            UserState::PendingVerification { .. } => UserStatusKind::PendingVerification,
            UserState::ActiveWithUnverifiedEmail { .. } => {
                UserStatusKind::ActiveWithUnverifiedEmail
            }
        }
        .into()
    }
}
