use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Display,
    derive_more::From,
    derive_more::Into,
)]
pub struct OutboxEventId(Uuid);
