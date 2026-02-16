mod entity;
mod error;
mod events;
mod factory;
mod repository;
mod service;
mod value_objects;

pub use entity::{User, UserState, UserStateKind, UserStateRaw};
pub use error::{
    ModificationWithInvalidStateError, UserDomainError, UserReconstructionError,
    UserStateTransitionError, UserUniqueConstraintViolation,
};
pub use events::*;
pub use factory::UserFactory;
pub use repository::{UserRepository, UserRepositoryError};
pub use service::{
    EmailVerificationError, EmailVerifier, IdGenerator, IdGeneratorFactory, PasswordHasher,
    PasswordHashingError, UserUniquenessService,
};
pub use value_objects::{
    email::{Email, EmailFormatError, EmailTrait, UnverifiedEmail, VerifiedEmail},
    password::{HashedPassword, PasswordPolicyViolation, RawPassword},
    role::UserRole,
    user_id::UserId,
};
