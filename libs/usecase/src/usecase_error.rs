use domain::{auth::policy::AuthorizationError, transaction::IntoTxError};
use serde::Serialize;
use thiserror::Error;
use validator::{ValidationErrors, ValidationErrorsKind};

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

fn convert_validation_error(validation_errors: &ValidationErrors) -> ValidationErrorList {
    let mut errors = Vec::new();

    // 再帰的にすべてのエラーを収集
    flatten_validation_errors(validation_errors, &mut errors, None);

    // フォールバック: バリデーション自体は失敗しているのにエラーが一つも収集できなかった場合
    // (通常は発生しませんが、__all__ のマッピング漏れなどの安全策として)
    if errors.is_empty() {
        errors.push(ValidationError::new(
            "schema",
            "入力内容に誤りがあります（詳細なエラーを特定できませんでした）",
        ));
    }

    ValidationErrorList(errors)
}

/// 再帰的に ValidationErrors をフラットなリストに展開するヘルパー関数
fn flatten_validation_errors(
    validation_errors: &ValidationErrors,
    acc: &mut Vec<ValidationError>,
    parent_path: Option<String>,
) {
    for (field, kind) in validation_errors.errors() {
        // フィールドパスの構築 (例: "user.profile.age" や "items.name")
        // "__all__" (スキーマエラー) の場合は特別なキー ("schema" 等) にマッピングするか、親の名前を使います
        let field_path = if *field == "__all__" {
            // スキーマ全体のエラーの場合
            parent_path.clone().unwrap_or_else(|| "schema".to_string())
        } else {
            match &parent_path {
                Some(parent) => format!("{}.{}", parent, field),
                None => field.to_string(),
            }
        };

        match kind {
            // 単一フィールドのエラー (スキーマエラーもここに含まれることが多い)
            ValidationErrorsKind::Field(field_errors) => {
                for err in field_errors {
                    let message = err
                        .message
                        .as_ref()
                        .map(|cow_str| cow_str.to_string())
                        .unwrap_or_else(|| "不正な値です".into());

                    acc.push(ValidationError::new(field_path.clone(), message));
                }
            }
            // ネストされた構造体のエラー (これを field_errors() は落としてしまう)
            ValidationErrorsKind::Struct(nested_errors) => {
                flatten_validation_errors(nested_errors, acc, Some(field_path));
            }
            // リスト内のエラー
            ValidationErrorsKind::List(list_errors) => {
                for (index, nested_errors) in list_errors {
                    let list_path = format!("{}[{}]", field_path, index);
                    flatten_validation_errors(nested_errors, acc, Some(list_path));
                }
            }
        }
    }
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
    use std::collections::HashMap;

    use validator::Validate;

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

    #[test]
    fn test_flatten_validation_errors() {
        #[derive(Validate)]
        struct Nested {
            #[validate(length(min = 3, message = "must be at least 3 characters"))]
            name: String,
        }

        #[derive(Validate)]
        struct TestStruct {
            #[validate(email(message = "invalid email"))]
            email: String,
            #[validate(nested)]
            nested: Nested,
            #[validate(nested)]
            items: Vec<Nested>,
        }

        let test_instance = TestStruct {
            email: "invalid-email".to_string(),
            nested: Nested {
                name: "ab".to_string(),
            },
            items: vec![
                Nested {
                    name: "a".to_string(),
                },
                Nested {
                    name: "validname".to_string(),
                },
            ],
        };

        let result = test_instance.validate();
        assert!(result.is_err());
        let validation_errors = result.unwrap_err();
        let error_list = ValidationErrorList::from(&validation_errors);
        let errors = error_list.0;
        let mut error_map = HashMap::new();
        for error in errors {
            error_map.insert(error.field.clone(), error.message.clone());
        }

        println!("{:?}", error_map);

        assert_eq!(error_map.get("email").unwrap(), "invalid email");
        assert_eq!(
            error_map.get("nested.name").unwrap(),
            "must be at least 3 characters"
        );
        assert_eq!(
            error_map.get("items[0].name").unwrap(),
            "must be at least 3 characters"
        );
        assert!(error_map.get("items[1].name").is_none());
    }
}
