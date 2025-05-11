use actix_web::web;
use attendx_backend::{
    routes::{
        auth_routes::configure_auth_routes, organization_routes::configure_organization_routes,
        user_routes::configure_user_routes,
    },
    setup::{database::setup_database, services::setup_services},
};
use dotenv::dotenv;
use log::info;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    dotenv().ok();

    if env_logger::try_init().is_err() {
        eprintln!("Logger already initialized");
    }

    info!("🚀 Starting Smart Attendance Backend...");

    let client = setup_database().await;
    let (user_service, organization_service) = setup_services(&client).await;

    let user_service_data = web::Data::new(user_service.clone());
    let organization_service_data = web::Data::new(organization_service.clone());

    let config = move |cfg: &mut web::ServiceConfig| {
        configure_user_routes(cfg, user_service_data.clone());
        configure_auth_routes(cfg, user_service_data.clone());
        configure_organization_routes(cfg, organization_service_data.clone());
    };

    info!("✅ Application started successfully");
    Ok(config.into())
}
