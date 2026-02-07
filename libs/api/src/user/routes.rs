use actix_web::web;

use crate::user::{get_own_profile, get_profile, list_users, suspend_user, update_profile};

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .service(get_own_profile::get_own_profile_handler)
            .service(get_profile::get_profile_handler)
            .service(list_users::list_users_handler)
            .service(update_profile::update_profile_handler)
            .service(suspend_user::suspend_user_handler),
    );
}
