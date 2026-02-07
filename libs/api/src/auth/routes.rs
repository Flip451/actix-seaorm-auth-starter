use actix_web::web;

use super::{login, signup};

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signup", web::post().to(signup::signup_handler))
            .route("/login", web::post().to(login::login_handler)),
    );
}
