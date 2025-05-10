use crate::config::cors::configure_cors;
use crate::handlers::user_handler::{jwt_login_handler, logout_user_handler};
use crate::services::user_service::UserService;
use actix_web::web;
use std::sync::Arc;

pub fn configure_auth_routes(
    cfg: &mut web::ServiceConfig,
    user_service_data: web::Data<Arc<UserService>>,
) {
    cfg.service(
        web::scope("/login")
            .app_data(user_service_data.clone())
            .route("", web::post().to(jwt_login_handler))
            .wrap(configure_cors()),
    );

    cfg.service(
        web::scope("/logout")
            .app_data(user_service_data)
            .route("", web::delete().to(logout_user_handler))
            .wrap(configure_cors()),
    );
}
