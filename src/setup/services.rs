use crate::{
    repositories::{
        organization_repository::OrganizationRepository, user_repository::UserRepository,
    },
    services::{organization_service::OrganizationService, user_service::UserService},
};
use std::sync::Arc;

pub async fn setup_services(
    client: &Arc<mongodb::Client>,
) -> (Arc<UserService>, Arc<OrganizationService>) {
    let user_repository = UserRepository::new(client)
        .await
        .expect("❌ Failed to initialize UserRepository");
    let organization_repository = OrganizationRepository::new(client)
        .await
        .expect("❌ Failed to initialize OrganizationRepository");

    let user_service = Arc::new(UserService::new(Arc::new(user_repository)));
    let organization_service =
        Arc::new(OrganizationService::new(Arc::new(organization_repository)));

    (user_service, organization_service)
}
