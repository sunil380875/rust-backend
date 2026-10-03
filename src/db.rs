use mongodb::{Client, Database, options::ClientOptions};
use tracing::info;

/// Initialize the MongoDB client and return a handle to the named database.
pub async fn init_db(uri: &str, db_name: &str) -> anyhow::Result<Database> {
    let options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(options)?;

    // Ping to verify connection
    client
        .database("admin")
        .run_command(bson::doc! { "ping": 1 })
        .await?;

    info!("Connected to MongoDB (db: {})", db_name);
    Ok(client.database(db_name))
}
