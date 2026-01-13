use domain::user::UserRole;
use uuid::Uuid;

pub trait Identity: Send {
    fn actor_id(&self) -> Uuid;
    fn actor_role(&self) -> UserRole;
}
