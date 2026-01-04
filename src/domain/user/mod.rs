mod entity;
mod error;
mod repository;
mod service;
mod value_objects;

pub use entity::{User, UserState};
pub use error::{UserDomainError, UserUniqueConstraint, UserStateTransitionError};
pub use repository::{UserRepository, UserRepositoryError};
pub use service::{EmailVerificationError, EmailVerifier, PasswordHasher, PasswordHashingError};
pub use value_objects::{
    email::{Email, UnverifiedEmail, VerifiedEmail, EmailTrait},
    password::{HashedPassword, RawPassword},
    role::UserRole,
};
