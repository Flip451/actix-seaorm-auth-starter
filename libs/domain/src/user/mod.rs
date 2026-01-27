mod entity;
mod error;
mod events;
mod factory;
mod repository;
mod service;
mod value_objects;

pub use entity::{User, UserState, UserStateRaw};
pub use error::{
    UserDomainError, UserReconstructionError, UserStateTransitionError, UserUniqueConstraint,
};
pub use events::*;
pub use factory::UserFactory;
pub use repository::{UserRepository, UserRepositoryError};
pub use service::{
    EmailVerificationError, EmailVerifier, IdGenerator, PasswordHasher, PasswordHashingError,
    UserUniquenessService,
};
pub use value_objects::{
    email::{Email, EmailTrait, UnverifiedEmail, VerifiedEmail},
    password::{HashedPassword, RawPassword},
    role::UserRole,
    user_id::UserId,
};
