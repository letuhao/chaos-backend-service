use mongodb::{Client, Collection};
use mongodb::bson::doc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🔧 Clearing MongoDB runtime_flags to use hardcoded defaults...");
    
    // Connect to MongoDB
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let db = client.database("chaos_game");
    let collection: Collection<mongodb::bson::Document> = db.collection("runtime_flags");
    
    // Delete all documents in runtime_flags collection
    let result = collection.delete_many(doc! {}, None).await?;
    println!("✅ Deleted {} documents from runtime_flags collection", result.deleted_count);
    
    println!("🎯 Chaos Backend will now use hardcoded port 8081");
    
    Ok(())
}
