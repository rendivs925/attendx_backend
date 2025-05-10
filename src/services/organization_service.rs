use crate::{
    models::organization_model::Organization,
    repositories::organization_repository::OrganizationRepository,
};
use anyhow::Result;
use std::sync::Arc;

pub struct OrganizationService {
    organization_repository: Arc<OrganizationRepository>,
}

impl OrganizationService {
    pub fn new(organization_repository: Arc<OrganizationRepository>) -> Self {
        Self {
            organization_repository,
        }
    }

    pub async fn create_organization(&self, organization: Organization) -> Result<Organization> {
        self.organization_repository
            .create_organization(organization)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn get_organization_by_id(&self, org_id: &str) -> Result<Option<Organization>> {
        self.organization_repository
            .find_organization_by_id(org_id)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn get_all_organizations(&self) -> Result<Vec<Organization>> {
        self.organization_repository
            .get_all_organizations()
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn update_organization(
        &self,
        org_id: &str,
        organization: Organization,
    ) -> Result<Organization> {
        self.organization_repository
            .update_organization(org_id, &organization)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn delete_organization(&self, org_id: &str) -> Result<()> {
        self.organization_repository
            .delete_organization(org_id)
            .await
            .map_err(anyhow::Error::from)
    }
}
