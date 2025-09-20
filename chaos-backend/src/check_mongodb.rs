use mongodb::{Client, Collection};
use serde_json::Value;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to MongoDB
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    
    // Check both databases
    let databases = ["chaos_game", "actor_core_config"];
    
    for db_name in &databases {
        let db = client.database(db_name);
        println!("\n{}", "=".repeat(60));
        println!("🔍 DATABASE: {}", db_name);
        println!("{}", "=".repeat(60));
        
        // List all collections
        println!("\n📋 COLLECTIONS IN DATABASE:");
        let collections = db.list_collection_names(None).await?;
        
        if collections.is_empty() {
            println!("  No collections found");
            continue;
        }
        
        for collection_name in &collections {
            let count = db.collection::<Value>(collection_name).count_documents(None, None).await?;
            println!("  - {}: {} documents", collection_name, count);
        }
        
        println!("\n{}", "=".repeat(40));
        
        // Check runtime_flags collection
        if collections.contains(&"runtime_flags".to_string()) {
            println!("\n🚩 RUNTIME FLAGS COLLECTION:");
            let runtime_flags: Collection<Value> = db.collection("runtime_flags");
            let mut flags_docs = runtime_flags.find(None, None).await?;
            
            while let Some(doc_result) = flags_docs.next().await {
                let doc = doc_result?;
                println!("\n📄 Document ID: {}", doc.get("_id").unwrap_or(&Value::Null));
                
                for (key, value) in doc.as_object().unwrap() {
                    if key != "_id" {
                        let value_type = match value {
                            Value::Null => "null",
                            Value::Bool(_) => "bool",
                            Value::Number(_) => "number",
                            Value::String(_) => "string",
                            Value::Array(_) => "array",
                            Value::Object(_) => "object",
                        };
                        println!("  {}: {} ({})", key, value, value_type);
                    }
                }
            }
        }
        
        // Check configurations collection
        let config_collections: Vec<_> = collections.iter()
            .filter(|name| name.to_lowercase().contains("config") || name.to_lowercase().contains("configuration"))
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
                                        let value_type = match value {
                                            Value::Null => "null",
                                            Value::Bool(_) => "bool",
                                            Value::Number(_) => "number",
                                            Value::String(_) => "string",
                                            Value::Array(_) => "array",
                                            Value::Object(_) => "object",
                                        };
                                        println!("    {}: {} ({})", key, value, value_type);
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
        
        // Summary for this database
        println!("\n📊 SUMMARY FOR {}:", db_name);
        println!("  Total collections: {}", collections.len());
        
        let mut total_config_docs = 0;
        for collection_name in &config_collections {
            let count = db.collection::<Value>(collection_name).count_documents(None, None).await?;
            total_config_docs += count;
        }
        
        println!("  Configuration documents: {}", total_config_docs);
    }
    
    println!("\n✅ MongoDB data structure analysis completed!");
    
    Ok(())
}