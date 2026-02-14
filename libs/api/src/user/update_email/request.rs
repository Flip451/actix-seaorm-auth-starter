use serde::Deserialize;
use usecase::user::dto::UpdateUserEmailInput;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(derive_more::Debug, Deserialize, Validate)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub struct UpdateEmailRequest {
    #[debug(skip)]
    #[validate(email(message = "有効なメールアドレスを入力してください"))]
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

#[cfg(test)]
mod tests {
    use super::UpdateEmailRequest;
    use validator::Validate;

    #[test]
    fn test_validate_empty_email() {
        let request = UpdateEmailRequest {
            email: "".to_string(),
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("email"));
    }

    #[test]
    fn test_validate_invalid_email() {
        let request = UpdateEmailRequest {
            email: "invalid-email".to_string(),
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("email"));
    }

    #[test]
    fn test_validate_valid_email() {
        let request = UpdateEmailRequest {
            email: "user@example.com".to_string(),
        };

        let result = request.validate();
        assert!(result.is_ok());
    }
}
