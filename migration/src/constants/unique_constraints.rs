#[derive(strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum UniqueConstraints {
    UserEmailKey,
    UserUsernameKey,
}
