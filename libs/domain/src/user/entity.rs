use chrono::{DateTime, Utc};
use derive_entity::Entity;
use strum::EnumString;

use crate::{
    shared::{
        outbox_event::{EntityWithEvents, OutboxEvent},
        service::clock::Clock,
    },
    user::{
        Email, EmailTrait, UserEvent, UserId, UserReconstructionError, UserStateTransitionError,
        error::ModificationWithInvalidStateError,
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

#[derive(Entity)]
pub struct User {
    #[entity_id]
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
        now: DateTime<Utc>,
    ) -> Result<Self, UserDomainError> {
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
        role: &str,
        state_source: UserStateRaw,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, UserReconstructionError> {
        let state = state_source.try_into()?;
        let role = role.try_into()?;

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
            .map(|e| {
                let created_at = e.created_at();
                OutboxEvent::new(e.into(), created_at)
            })
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
        clock: &dyn Clock,
    ) -> Result<(), ModificationWithInvalidStateError> {
        let old_username = self.username.clone();
        self.username = new_username;

        let now = clock.now();
        self.updated_at = now;

        self.record_event(UserEvent::UsernameChanged(UsernameChangedEvent {
            old_username,
            new_username: self.username.clone(),
            email: self.email(),
            changed_at: now,
        }));
        Ok(())
    }
}

// ユーザーの状態遷移に関するメソッド群
impl User {
    pub fn verify_email<V: EmailVerifier>(
        &mut self,
        verifier: &V,
        clock: &dyn Clock,
    ) -> Result<(), UserDomainError> {
        let email = match &self.state {
            UserState::Active { .. } => return Ok(()), // すでに検証済みなので何もしない
            UserState::SuspendedByAdmin { .. } => {
                Err(UserStateTransitionError::AlreadySuspended {
                    to: UserStateKind::Active,
                })?
            }
            UserState::DeactivatedByUser { .. } => {
                Err(UserStateTransitionError::AlreadyDeactivated {
                    to: UserStateKind::Active,
                })?
            }
            UserState::PendingVerification { email } => {
                let email = verifier.verify(email)?;
                self.state = UserState::Active {
                    email: email.clone(),
                };
                email
            }
            UserState::ActiveWithUnverifiedEmail { email } => {
                let email = verifier.verify(email)?;
                self.state = UserState::Active {
                    email: email.clone(),
                };
                email
            }
        };

        let now = clock.now();
        self.updated_at = now;

        self.record_event(UserEvent::EmailVerified(UserEmailVerifiedEvent {
            email,
            username: self.username.clone(),
            verified_at: now,
        }));

        Ok(())
    }

    pub fn change_email(
        &mut self,
        UniqueEmail(new_email): UniqueEmail,
        clock: &dyn Clock,
    ) -> Result<(), ModificationWithInvalidStateError> {
        if new_email.as_str() == self.email().as_str() {
            // 新しいメールアドレスが現在のものと同じ場合、何もしない
            return Ok(());
        }

        match &self.state {
            UserState::Active { .. } => {
                self.state = UserState::ActiveWithUnverifiedEmail {
                    email: new_email.clone(),
                };
            }
            UserState::SuspendedByAdmin { .. } => {
                Err(ModificationWithInvalidStateError::EmailModification {
                    state: self.state.kind_raw(),
                })?
            }
            UserState::DeactivatedByUser { .. } => {
                Err(ModificationWithInvalidStateError::EmailModification {
                    state: self.state.kind_raw(),
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

        let now = clock.now();
        self.updated_at = now;

        self.record_event(UserEvent::EmailChanged(UserEmailChangedEvent {
            new_email,
            username: self.username.clone(),
            changed_at: now,
        }));
        Ok(())
    }

    pub fn suspend(&mut self, reason: String, clock: &dyn Clock) -> Result<(), UserDomainError> {
        let email = match &self.state {
            UserState::Active { email } => {
                let email = email.unverify();
                self.state = UserState::SuspendedByAdmin {
                    email: email.clone(),
                };
                email
            }
            UserState::SuspendedByAdmin { .. } => return Ok(()), // すでに停止中なので何もしない
            UserState::DeactivatedByUser { email } => {
                let email = email.clone();
                self.state = UserState::SuspendedByAdmin {
                    email: email.clone(),
                };
                email
            }
            UserState::PendingVerification { email } => {
                let email = email.clone();
                self.state = UserState::SuspendedByAdmin {
                    email: email.clone(),
                };
                email
            }
            UserState::ActiveWithUnverifiedEmail { email } => {
                let email = email.clone();
                self.state = UserState::SuspendedByAdmin {
                    email: email.clone(),
                };
                email
            }
        };

        let now = clock.now();
        self.updated_at = now;

        self.record_event(UserEvent::Suspended(UserSuspendedEvent {
            username: self.username.clone(),
            email,
            reason,
            suspended_at: now,
        }));

        Ok(())
    }

    pub fn deactivate(&mut self, clock: &dyn Clock) -> Result<(), UserDomainError> {
        let email = match &self.state {
            UserState::Active { email } => {
                let email = email.unverify();
                self.state = UserState::DeactivatedByUser {
                    email: email.clone(),
                };
                email
            }
            UserState::SuspendedByAdmin { .. } => {
                Err(UserStateTransitionError::AlreadySuspended {
                    to: UserStateKind::DeactivatedByUser,
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

        let now = clock.now();
        self.updated_at = now;

        self.record_event(UserEvent::Deactivated(UserDeactivatedEvent {
            username: self.username.clone(),
            email,
            deactivated_at: now,
        }));

        Ok(())
    }

    pub fn activate<V: EmailVerifier>(&mut self, clock: &dyn Clock) -> Result<(), UserDomainError> {
        let email = match &self.state {
            UserState::Active { .. } => return Ok(()), // すでにアクティブなので何もしない
            UserState::SuspendedByAdmin { .. } => {
                Err(UserStateTransitionError::AlreadySuspended {
                    to: UserStateKind::Active,
                })?
            }
            UserState::DeactivatedByUser { email } => {
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

        let now = clock.now();
        self.updated_at = now;

        self.record_event(UserEvent::Reactivated(UserReactivatedEvent {
            username: self.username.clone(),
            email,
            reactivated_at: now,
        }));

        Ok(())
    }

    pub fn unlock_suspension(&mut self, clock: &dyn Clock) -> Result<(), UserDomainError> {
        let email = match &self.state {
            UserState::Active { .. }
            | UserState::DeactivatedByUser { .. }
            | UserState::PendingVerification { .. }
            | UserState::ActiveWithUnverifiedEmail { .. } => {
                Err(UserStateTransitionError::NotSuspended {
                    from: self.state.clone(),
                })?
            }
            UserState::SuspendedByAdmin { email } => {
                let email = email.clone();
                self.state = UserState::ActiveWithUnverifiedEmail {
                    email: email.clone(),
                };
                email
            }
        };

        let now = clock.now();
        self.updated_at = now;

        self.record_event(UserEvent::Unlocked(UserUnlockedEvent {
            username: self.username.clone(),
            email,
            unlocked_at: now,
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
        let UserStateRaw { status, email } = raw;

        let kind = status.parse::<UserStateKind>().map_err(|_| {
            UserReconstructionError::InvalidStatus {
                invalid_status: status,
            }
        })?;

        match kind {
            UserStateKind::Active => Ok(UserState::Active {
                email: VerifiedEmail::new(&email)?,
            }),
            UserStateKind::SuspendedByAdmin => Ok(UserState::SuspendedByAdmin {
                email: UnverifiedEmail::new(&email)?,
            }),
            UserStateKind::DeactivatedByUser => Ok(UserState::DeactivatedByUser {
                email: UnverifiedEmail::new(&email)?,
            }),
            UserStateKind::PendingVerification => Ok(UserState::PendingVerification {
                email: UnverifiedEmail::new(&email)?,
            }),
            UserStateKind::ActiveWithUnverifiedEmail => Ok(UserState::ActiveWithUnverifiedEmail {
                email: UnverifiedEmail::new(&email)?,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq, strum::Display, EnumString, strum::IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum UserStateKind {
    Active,
    SuspendedByAdmin,
    DeactivatedByUser,
    PendingVerification,
    ActiveWithUnverifiedEmail,
}

impl UserState {
    pub fn kind(&self) -> &'static str {
        self.kind_raw().into()
    }

    fn kind_raw(&self) -> UserStateKind {
        match self {
            UserState::Active { .. } => UserStateKind::Active,
            UserState::SuspendedByAdmin { .. } => UserStateKind::SuspendedByAdmin,
            UserState::DeactivatedByUser { .. } => UserStateKind::DeactivatedByUser,
            UserState::PendingVerification { .. } => UserStateKind::PendingVerification,
            UserState::ActiveWithUnverifiedEmail { .. } => UserStateKind::ActiveWithUnverifiedEmail,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::user::EmailFormatError;

    use super::*;

    #[rstest]
    #[case("active", "valid@email.com", UserState::Active { email: VerifiedEmail::new("valid@email.com").unwrap() })]
    #[case("suspended_by_admin", "valid@email.com", UserState::SuspendedByAdmin { email: UnverifiedEmail::new("valid@email.com").unwrap() })]
    #[case("deactivated_by_user", "valid@email.com", UserState::DeactivatedByUser { email: UnverifiedEmail::new("valid@email.com").unwrap() })]
    #[case("pending_verification", "valid@email.com", UserState::PendingVerification { email: UnverifiedEmail::new("valid@email.com").unwrap() })]
    #[case("active_with_unverified_email", "valid@email.com", UserState::ActiveWithUnverifiedEmail { email: UnverifiedEmail::new("valid@email.com").unwrap() })]
    fn test_try_from_user_state_raw_to_user_state(
        #[case] status: &'static str,
        #[case] email: &'static str,
        #[case] expected: UserState,
    ) {
        let raw = UserStateRaw {
            status: status.to_string(),
            email: email.to_string(),
        };

        let state: UserState = raw.try_into().unwrap();
        assert_eq!(state, expected);
    }

    #[rstest]
    #[case("invalid_status", "valid@email.com", UserReconstructionError::InvalidStatus{ invalid_status: "invalid_status".to_string() })]
    #[case("", "valid@email.com", UserReconstructionError::InvalidStatus{ invalid_status: "".to_string() })]
    fn test_try_from_invalid_user_state(
        #[case] status: &'static str,
        #[case] email: &'static str,
        #[case] expected_error: UserReconstructionError,
    ) {
        let raw = UserStateRaw {
            status: status.to_string(),
            email: email.to_string(),
        };

        let result: Result<UserState, UserReconstructionError> = raw.try_into();
        assert_eq!(result.unwrap_err(), expected_error);
    }

    #[rstest]
    #[case("active", "invalid_email", "invalid_email")]
    #[case("suspended_by_admin", "invalid_email", "invalid_email")]
    #[case("deactivated_by_user", "invalid_email", "invalid_email")]
    #[case("pending_verification", "invalid_email", "invalid_email")]
    #[case("active_with_unverified_email", "invalid_email", "invalid_email")]
    fn test_try_from_invalid_user_email(
        #[case] status: &'static str,
        #[case] email: &'static str,
        #[case] expected_email_in_error: &'static str,
    ) {
        let raw = UserStateRaw {
            status: status.to_string(),
            email: email.to_string(),
        };

        let result: Result<UserState, UserReconstructionError> = raw.try_into();

        if let Err(UserReconstructionError::InvalidEmail(EmailFormatError::InvalidFormat {
            invalid_email,
            error: _,
        })) = result
        {
            assert_eq!(invalid_email, expected_email_in_error)
        } else {
            panic!()
        }
    }
}
