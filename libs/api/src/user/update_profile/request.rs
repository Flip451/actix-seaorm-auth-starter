use serde::Deserialize;
use usecase::user::dto::UpdateUserProfileInput;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(derive_more::Debug, Deserialize, Validate)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, message = "ユーザー名は空にできません"))]
    #[cfg_attr(feature = "api-docs", schema(examples("exampleuser")))]
    pub username: Option<String>,
    // 他に更新可能なプロフィール情報があればここに追加
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
    fn test_validate_no_fields() {
        // すべてのフィールドがNoneの場合でもバリデーションは成功する
        // 実際のビジネスロジックでのチェックは別途行う想定
        let request = UpdateProfileRequest { username: None };

        let result = request.validate();
        assert!(result.is_ok());
    }
}
