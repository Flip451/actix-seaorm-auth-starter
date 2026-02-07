use actix_web::web;

use super::{get_profile, list_users, suspend_user, update_profile};

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users") // /user/me というパスになる
            .route("/me", web::get().to(get_profile::get_user_handler))
            .route(
                "/me",
                web::patch().to(update_profile::update_profile_handler),
            ),
    );
    cfg.service(
        web::scope("/admin") // /admin/users というパスになる
            .route(
                "/users/{user_id}/suspend",
                web::post().to(suspend_user::suspend_user_handler),
            )
            .route("/users", web::get().to(list_users::list_users_handler)),
    );
}
