use actix_web::web;
use std::sync::Arc;

use crate::{
    config::cors::configure_cors,
    handlers::auth_handler::{jwt_login_handler, logout_user_handler, register_user_handler},
    services::user_service::UserService,
};

pub fn configure_auth_routes(
    cfg: &mut web::ServiceConfig,
    user_service_data: web::Data<Arc<UserService>>,
) {
    cfg.service(
        web::scope("/auth")
            .wrap(configure_cors())
            .app_data(user_service_data)
            .route("/login", web::post().to(jwt_login_handler))
            .route("/logout", web::delete().to(logout_user_handler))
            .route("/register", web::post().to(register_user_handler)),
    );
}
