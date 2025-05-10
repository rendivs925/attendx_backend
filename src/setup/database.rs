use crate::config::database::{connect_to_database, create_unique_indexes};
use std::sync::Arc;

pub async fn setup_database() -> Arc<mongodb::Client> {
    let client = connect_to_database()
        .await
        .expect("❌ Failed to connect to MongoDB");
    println!("✅ Connected to MongoDB");

    create_unique_indexes(&client)
        .await
        .expect("❌ Failed to create indexes");
    println!("✅ Unique indexes created successfully");

    Arc::new(client)
}
