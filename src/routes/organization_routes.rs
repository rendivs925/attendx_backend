use crate::config::cors::configure_cors;
use crate::handlers::organization_handler::{
    create_organization_handler, delete_organization_handler, get_all_organizations_handler,
    get_organization_handler, update_organization_handler,
};
use crate::services::organization_service::OrganizationService;
use actix_web::web;
use std::sync::Arc;

pub fn configure_organization_routes(
    cfg: &mut web::ServiceConfig,
    organization_service: web::Data<Arc<OrganizationService>>,
) {
    cfg.service(
        web::scope("/organizations")
            .app_data(organization_service)
            .route("/new", web::post().to(create_organization_handler))
            .route("/all", web::get().to(get_all_organizations_handler))
            .route("/{id}", web::get().to(get_organization_handler))
            .route("/{id}", web::put().to(update_organization_handler))
            .route("/{id}", web::delete().to(delete_organization_handler))
            .wrap(configure_cors()),
    );
}
