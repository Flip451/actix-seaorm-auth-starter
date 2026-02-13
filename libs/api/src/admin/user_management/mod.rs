pub mod list_users;
pub mod routes;
pub mod suspend_user;

pub use self::routes::user_management_config;

#[cfg(feature = "api-docs")]
pub use self::routes::UserManagementApi;
