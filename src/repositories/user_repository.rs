use crate::constants::USER_COL_NAME;
use crate::{
    config::database::get_collection, models::user_model::User,
    types::requests::user::update_user_request::UpdateUserRequest,
};
use bson::Document;
use futures_util::stream::TryStreamExt;
use mongodb::bson::{doc, to_document};
use mongodb::{Client, Collection, error::Result};

pub struct UserRepository {
    pub collection: Collection<User>,
}

impl UserRepository {
    pub async fn new(client: &Client) -> Result<Self> {
        let collection = get_collection(client, (*USER_COL_NAME).as_str()).await?;
        Ok(Self { collection })
    }

    pub async fn create_user(&self, user: &User) -> Result<User> {
        self.collection.insert_one(user).await?;
        Ok(User { ..user.clone() })
    }

    pub async fn find_user(&self, field: &str, value: &str) -> Result<Option<User>> {
        let mut filter = Document::new();
        filter.insert(field, value);
        self.collection.find_one(filter).await
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let cursor = self.collection.find(doc! {}).await?;
        let users: Vec<User> = cursor.try_collect().await?;
        Ok(users)
    }

    pub async fn update_user(
        &self,
        email: &str,
        user: UpdateUserRequest,
    ) -> Result<UpdateUserRequest> {
        let filter = doc! { "email": email };
        let update_doc = to_document(&user)?;

        self.collection
            .update_one(filter, doc! { "$set": update_doc })
            .await?;

        Ok(user)
    }

    pub async fn delete_user(&self, email: &str) -> Result<()> {
        let filter = doc! { "email": email };
        self.collection.delete_one(filter).await?;
        Ok(())
    }
}
