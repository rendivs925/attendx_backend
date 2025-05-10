use crate::{
    constants::{DB_NAME, MONGODB_URI, USER_COL_NAME},
    models::user_model::User,
};
use mongodb::{
    Client, Collection, IndexModel,
    bson::doc,
    error::Error as MongoError,
    options::{ClientOptions, IndexOptions},
};

pub async fn connect_to_database() -> Result<Client, MongoError> {
    let client_uri = (*MONGODB_URI).as_str();

    let client_options = ClientOptions::parse(client_uri).await?;
    Client::with_options(client_options)
}

pub async fn get_collection<T>(
    client: &Client,
    collection_name: &str,
) -> Result<Collection<T>, MongoError>
where
    T: serde::de::DeserializeOwned + serde::Serialize + Unpin + Send + Sync,
{
    Ok(client.database(&DB_NAME).collection::<T>(collection_name))
}

async fn create_partial_unique_index(
    collection: &Collection<User>,
    field: &str,
) -> Result<(), MongoError> {
    let index = IndexModel::builder()
        .keys(doc! { field: 1 })
        .options(
            IndexOptions::builder()
                .unique(true)
                .partial_filter_expression(doc! { field: { "$exists": true, "$type": "string"  } })
                .build(),
        )
        .build();

    collection.create_index(index).await?;
    Ok(())
}

pub async fn create_unique_indexes(client: &Client) -> Result<(), MongoError> {
    let collection = get_collection::<User>(client, &USER_COL_NAME).await?;

    create_partial_unique_index(&collection, "email").await?;
    create_partial_unique_index(&collection, "username").await?;
    create_partial_unique_index(&collection, "nim").await?;
    create_partial_unique_index(&collection, "nidn").await?;

    Ok(())
}
