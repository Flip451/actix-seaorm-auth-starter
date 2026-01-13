mod entity;
mod error;
mod events;
mod repository;
mod service;
mod value_objects;

pub use entity::{User, UserState};
pub use error::{UserDomainError, UserStateTransitionError, UserUniqueConstraint};
pub use events::*;
pub use repository::{UserRepository, UserRepositoryError};
pub use service::{EmailVerificationError, EmailVerifier, PasswordHasher, PasswordHashingError};
pub use value_objects::{
    email::{Email, EmailTrait, UnverifiedEmail, VerifiedEmail},
    password::{HashedPassword, RawPassword},
    role::UserRole,
};
