use serde::Deserialize;
use validator::Validate;

#[cfg(feature = "api-docs")]
use utoipa::ToSchema;

#[derive(derive_more::Debug, Deserialize, Validate)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub struct LoginRequest {
    #[validate(email(message = "有効なメールアドレスを入力してください"))]
    #[cfg_attr(feature = "api-docs", schema(example = "user@example.com"))]
    pub email: String,

    #[validate(length(min = 8, message = "パスワードは8文字以上必要です"))]
    #[cfg_attr(feature = "api-docs", schema(example = "password123"))]
    #[debug(skip)]
    pub password: String,
}

impl From<LoginRequest> for usecase::auth::dto::LoginInput {
    fn from(req: LoginRequest) -> Self {
        Self {
            email: req.email,
            password: req.password,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LoginRequest;
    use validator::Validate;

    #[test]
    fn test_validate_invalid_email() {
        let request = LoginRequest {
            email: "invalid-email".to_string(),
            password: "validpassword123".to_string(),
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("email"));
    }

    #[test]
    fn test_validate_short_password() {
        let request = LoginRequest {
            email: "valid@example.com".to_string(),
            password: "short".to_string(),
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn test_validate_valid_request() {
        let request = LoginRequest {
            email: "valid@example.com".to_string(),
            password: "validpassword123".to_string(),
        };

        let result = request.validate();
        assert!(result.is_ok());
    }
}
