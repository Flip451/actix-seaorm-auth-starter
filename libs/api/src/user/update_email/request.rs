use serde::Deserialize;
use usecase::user::dto::UpdateUserEmailInput;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(derive_more::Debug, Deserialize)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub struct UpdateEmailRequest {
    #[debug(skip)]
    #[cfg_attr(feature = "api-docs", schema(examples("user@example.com")))]
    pub email: String,
}

impl UpdateEmailRequest {
    pub(super) fn into_input(self, target_id: Uuid) -> UpdateUserEmailInput {
        UpdateUserEmailInput {
            target_id,
            new_email: self.email,
        }
    }
}
