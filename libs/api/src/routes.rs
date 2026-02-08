use actix_web::web;

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    crate::auth::auth_config(cfg);
    crate::user::user_config(cfg);
}
