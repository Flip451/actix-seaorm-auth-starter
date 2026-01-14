use domain::user::{UserId, UserRole};

pub trait Identity: Send {
    fn actor_id(&self) -> UserId;
    fn actor_role(&self) -> UserRole;
}
