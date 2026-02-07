use serde::Serialize;
use usecase::user::dto::{ListUsersOutput, UserData};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ListUsersResponse {
    #[schema(example =
        json!([
            {
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "username": "exampleuser",
                "email": "example@example.com",
            },
        ])
    )]
    pub users: Vec<UserInfo>,
}

impl From<ListUsersOutput> for ListUsersResponse {
    fn from(output: ListUsersOutput) -> Self {
        ListUsersResponse {
            users: output.users.into_iter().map(|user| user.into()).collect(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}

impl From<UserData> for UserInfo {
    fn from(user: UserData) -> Self {
        UserInfo {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            role: user.role.to_string(),
        }
    }
}

crate::impl_responder_for!(ListUsersResponse);
