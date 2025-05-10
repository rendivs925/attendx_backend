use crate::constants::ORGANIZATIONS_COL_NAME;
use crate::{config::database::get_collection, models::organization_model::Organization};
use futures_util::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, to_document};
use mongodb::{Client, Collection, error::Result};

pub struct OrganizationRepository {
    collection: Collection<Organization>,
}

impl OrganizationRepository {
    pub async fn new(client: &Client) -> Result<Self> {
        let collection = get_collection(client, (*ORGANIZATIONS_COL_NAME).as_str()).await?;
        Ok(Self { collection })
    }

    pub async fn create_organization(
        &self,
        mut organization: Organization,
    ) -> Result<Organization> {
        let insert_result = self.collection.insert_one(&organization).await?;
        organization._id = Some(insert_result.inserted_id.as_object_id().unwrap());
        Ok(organization)
    }

    pub async fn find_organization_by_id(&self, org_id: &str) -> Result<Option<Organization>> {
        let object_id = ObjectId::parse_str(org_id).unwrap();
        self.collection.find_one(doc! { "_id": object_id }).await
    }

    pub async fn get_all_organizations(&self) -> Result<Vec<Organization>> {
        let cursor = self.collection.find(doc! {}).await?;
        let organizations: Vec<Organization> = cursor.try_collect().await?;
        Ok(organizations)
    }

    pub async fn update_organization(
        &self,
        org_id: &str,
        organization: &Organization,
    ) -> Result<Organization> {
        let object_id = ObjectId::parse_str(org_id).unwrap();
        let update_doc = to_document(organization)?;

        self.collection
            .update_one(doc! { "_id": object_id }, doc! { "$set": update_doc })
            .await?;

        Ok(organization.clone())
    }

    pub async fn delete_organization(&self, org_id: &str) -> Result<()> {
        let object_id = ObjectId::parse_str(org_id).unwrap();
        self.collection
            .delete_one(doc! { "_id": object_id })
            .await?;
        Ok(())
    }
}
