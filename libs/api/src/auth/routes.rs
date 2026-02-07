use actix_web::web;

use super::{login, signup};

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .service(signup::signup_handler)
            .service(login::login_handler),
    );
}
