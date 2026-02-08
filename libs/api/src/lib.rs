pub mod admin;
pub mod auth;
pub mod error;
pub mod middleware;
pub mod routes;
pub mod shared;
pub mod user;

pub use routes::routes_config;

#[cfg(feature = "api-docs")]
pub mod openapi;
#[cfg(feature = "api-docs")]
pub use openapi::generate_api_doc;
