use serde::Deserialize;
use usecase::user::dto::UpdateUserProfileInput;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(derive_more::Debug, Deserialize, Validate)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
#[validate(schema(function = "validate_at_least_one_field"))]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, message = "ユーザー名は空にできません"))]
    #[cfg_attr(feature = "api-docs", schema(example = "exampleuser"))]
    pub username: Option<String>,
    // 他に更新可能なプロフィール情報があればここに追加
}

impl UpdateProfileRequest {
    fn is_empty(&self) -> bool {
        [
            self.username.is_none(),
            // 他のオプションフィールドもここに追加
        ]
        .iter()
        .all(|x| *x)
    }
}

fn validate_at_least_one_field(
    input: &UpdateProfileRequest,
) -> Result<(), validator::ValidationError> {
    if input.is_empty() {
        let mut error = validator::ValidationError::new("at_least_one_field_required");
        error.message = Some("少なくとも1つの項目を変更してください".into());
        return Err(error);
    }
    Ok(())
}

impl UpdateProfileRequest {
    pub(super) fn into_input(self, target_id: Uuid) -> UpdateUserProfileInput {
        UpdateUserProfileInput {
            target_id,
            username: self.username,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UpdateProfileRequest;
    use usecase::usecase_error::ValidationErrorList;
    use validator::Validate;

    #[test]
    fn test_validate_empty_username() {
        let request = UpdateProfileRequest {
            username: Some("".to_string()),
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("username"));
    }

    #[test]
    fn test_validate_valid_username() {
        let request = UpdateProfileRequest {
            username: Some("validuser".to_string()),
        };

        let result = request.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_none_username() {
        let request = UpdateProfileRequest { username: None };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();

        let validation_error_list = ValidationErrorList::from(&errors);

        let json_string = serde_json::to_string(&validation_error_list)
            .expect("Failed to serialize validation errors");

        assert_eq!(
            json_string,
            r#"[{"field":"schema","message":"少なくとも1つの項目を変更してください"}]"#
        );
    }
}
