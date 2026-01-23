use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Deserialize,
    Serialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::From,
    derive_more::Into,
)]
pub struct UserId(Uuid);
