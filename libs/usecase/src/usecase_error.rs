use std::ops::Deref;

use domain::{auth::policy::AuthorizationError, transaction::IntoTxError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("不正な入力を受け取りました: {0:?}")]
    InvalidInput(ValidationErrorList),
    #[error("認証が必要です")]
    Unauthorized,
    #[error("許可されていない操作です: {message}")]
    Forbidden { message: String },
    #[error("リソースが見つかりませんでした")]
    NotFound,
    #[error("リソースの競合が検知されました: {message}")]
    Conflict { message: String },
    #[error("サーバー内部でエラーが発生しました: {0}")]
    Internal(#[source] anyhow::Error),
}

#[derive(derive_more::Debug, Error, Serialize)]
#[debug("{:?}", _0)]
#[error("{0:?}")]
pub struct ValidationErrorList(Vec<ValidationError>);

impl Deref for ValidationErrorList {
    type Target = Vec<ValidationError>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<ValidationError>> for ValidationErrorList {
    fn from(errors: Vec<ValidationError>) -> Self {
        ValidationErrorList(errors)
    }
}

#[derive(Debug, Serialize)]
pub struct ValidationError {
    field: String,
    message: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

impl IntoTxError for UseCaseError {
    fn into_tx_error(error: impl Into<anyhow::Error>) -> Self {
        UseCaseError::Internal(error.into())
    }
}

impl From<AuthorizationError> for UseCaseError {
    fn from(authz_error: AuthorizationError) -> Self {
        UseCaseError::Forbidden {
            message: authz_error.message_for_client().to_string(),
        }
    }
}

impl From<validator::ValidationErrors> for ValidationErrorList {
    fn from(validation_errors: validator::ValidationErrors) -> Self {
        convert_validation_error(&validation_errors)
    }
}

impl From<&validator::ValidationErrors> for ValidationErrorList {
    fn from(validation_errors: &validator::ValidationErrors) -> Self {
        convert_validation_error(validation_errors)
    }
}

fn convert_validation_error(
    validation_errors: &validator::ValidationErrors,
) -> ValidationErrorList {
    let errors = validation_errors
        .field_errors()
        .iter()
        .flat_map(|(field, field_errors)| {
            field_errors.iter().map(move |err| {
                // #[validate(message = "...")] で指定されたメッセージを取得
                // 指定がない場合は "不正な値です" を返す
                let message = err
                    .message
                    .as_ref()
                    .map(|cow_str| cow_str.to_string())
                    .unwrap_or_else(|| "不正な値です".into());
                ValidationError::new(field.clone(), message)
            })
        })
        .collect();

    ValidationErrorList(errors)
}

impl From<ValidationErrorList> for UseCaseError {
    fn from(errors: ValidationErrorList) -> Self {
        UseCaseError::InvalidInput(errors)
    }
}

impl From<validator::ValidationErrors> for UseCaseError {
    fn from(validation_errors: validator::ValidationErrors) -> Self {
        UseCaseError::InvalidInput(ValidationErrorList::from(validation_errors))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_validation_error_list() {
        let errors = vec![
            ValidationError::new("email", "不正な形式です"),
            ValidationError::new("password", "必須項目です"),
        ];
        let validation_error_list = ValidationErrorList::from(errors);

        let debug_output = format!("{:?}", validation_error_list);
        assert!(debug_output.contains("email"));
        assert!(debug_output.contains("不正な形式です"));
        assert!(debug_output.contains("password"));
        assert!(debug_output.contains("必須項目です"));
    }
}
