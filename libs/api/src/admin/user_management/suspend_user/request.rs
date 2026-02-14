use serde::Deserialize;
use usecase::user::dto::SuspendUserInput;
use uuid::Uuid;
use validator::Validate;

#[cfg(feature = "api-docs")]
use utoipa::ToSchema;

#[derive(derive_more::Debug, Deserialize, Validate)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub struct SuspendUserRequest {
    #[validate(length(min = 1, message = "理由を入力してください"))]
    #[cfg_attr(feature = "api-docs", schema(examples("規約違反")))]
    pub reason: String,
}

impl SuspendUserRequest {
    pub(super) fn into_input(self, target_id: Uuid) -> SuspendUserInput {
        SuspendUserInput {
            target_id,
            reason: self.reason,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SuspendUserRequest;
    use validator::Validate;

    #[test]
    fn test_validate_empty_reason() {
        let request = SuspendUserRequest {
            reason: "".to_string(),
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("reason"));
    }

    #[test]
    fn test_validate_valid_reason() {
        let request = SuspendUserRequest {
            reason: "規約違反".to_string(),
        };

        let result = request.validate();
        assert!(result.is_ok());
    }
}
