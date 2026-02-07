use domain::{
    auth::policy::Actor,
    user::{UserId, UserRole},
};
use uuid::Uuid;

pub trait Identity: std::fmt::Debug + Send {
    fn actor_id(&self) -> Uuid;
    fn actor_role(&self) -> UserRoleData;
}

#[derive(derive_more::Debug)]
pub struct IdentityWrapper {
    inner: Box<dyn Identity>,
}

impl Actor for IdentityWrapper {
    fn actor_id(&self) -> UserId {
        self.inner.actor_id().into()
    }

    fn actor_role(&self) -> UserRole {
        self.inner.actor_role().into()
    }
}

impl<T: Identity + 'static> From<T> for IdentityWrapper {
    fn from(identity: T) -> Self {
        Self {
            inner: Box::new(identity),
        }
    }
}

#[derive(derive_more::Debug, Clone, Copy, derive_more::Display, PartialEq, Eq)]
pub enum UserRoleData {
    Admin,
    User,
}

impl From<UserRoleData> for UserRole {
    fn from(role: UserRoleData) -> Self {
        match role {
            UserRoleData::Admin => UserRole::Admin,
            UserRoleData::User => UserRole::User,
        }
    }
}

impl From<UserRole> for UserRoleData {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::Admin => UserRoleData::Admin,
            UserRole::User => UserRoleData::User,
        }
    }
}
