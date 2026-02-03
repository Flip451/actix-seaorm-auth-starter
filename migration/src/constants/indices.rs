#[derive(strum::IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
#[strum(prefix = "idx_")]
pub enum Indices {
    OutboxProcessQueue,
}
