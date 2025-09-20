use mongodb::{Client, Collection};
use serde_json::Value;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to MongoDB
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let db = client.database("chaos_game");
    
    println!("{}", "=".repeat(60));
    println!("🔍 MONGODB DATA STRUCTURE ANALYSIS");
    println!("{}", "=".repeat(60));
    
    // List all collections
    println!("\n📋 COLLECTIONS IN DATABASE:");
    let collections = db.list_collection_names(None).await?;
    
    for collection_name in &collections {
        let count = db.collection::<Value>(collection_name).count_documents(None, None).await?;
        println!("  - {}: {} documents", collection_name, count);
    }
    
    println!("\n{}", "=".repeat(60));
    
    // Check runtime_flags collection
    println!("\n🚩 RUNTIME FLAGS COLLECTION:");
    let runtime_flags: Collection<Value> = db.collection("runtime_flags");
    let flags_docs = runtime_flags.find(None, None).await?;
    
    for doc_result in flags_docs {
        let doc = doc_result?;
        println!("\n📄 Document ID: {}", doc.get("_id").unwrap_or(&Value::Null));
        
        for (key, value) in doc.as_object().unwrap() {
            if key != "_id" {
                println!("  {}: {} ({})", key, value, value.type_str());
            }
        }
    }
    
    println!("\n{}", "=".repeat(60));
    
    // Check configurations collection
    let config_collections: Vec<_> = collections.iter()
        .filter(|name| name.to_lowercase().contains("config"))
        .collect();
    
    if !config_collections.is_empty() {
        println!("\n⚙️  CONFIGURATION COLLECTIONS:");
        
        for collection_name in &config_collections {
            println!("\n📁 Collection: {}", collection_name);
            let collection: Collection<Value> = db.collection(collection_name);
            let count = collection.count_documents(None, None).await?;
            println!("  Total documents: {}", count);
            
            if count > 0 {
                let mut sample_docs = collection.find(None, None).await?;
                println!("  Sample documents:");
                
                let mut i = 0;
                while let Some(doc_result) = sample_docs.next().await {
                    let doc = doc_result?;
                    i += 1;
                    println!("\n  📄 Document {}:", i);
                    println!("    ID: {}", doc.get("_id").unwrap_or(&Value::Null));
                    
                    for (key, value) in doc.as_object().unwrap() {
                        if key != "_id" {
                            match value {
                                Value::Object(obj) => {
                                    println!("    {}: Object with {} keys", key, obj.len());
                                }
                                Value::Array(arr) => {
                                    println!("    {}: Array with {} items", key, arr.len());
                                }
                                _ => {
                                    println!("    {}: {} ({})", key, value, value.type_str());
                                }
                            }
                        }
                    }
                    
                    if i >= 3 { break; }
                }
            }
        }
    } else {
        println!("\n⚠️  No configuration collections found");
    }
    
    println!("\n{}", "=".repeat(60));
    
    // Summary
    println!("\n📊 SUMMARY:");
    println!("  Total collections: {}", collections.len());
    
    let mut total_config_docs = 0;
    for collection_name in &config_collections {
        let count = db.collection::<Value>(collection_name).count_documents(None, None).await?;
        total_config_docs += count;
    }
    
    println!("  Configuration documents: {}", total_config_docs);
    
    println!("\n✅ MongoDB data structure analysis completed!");
    
    Ok(())
}
