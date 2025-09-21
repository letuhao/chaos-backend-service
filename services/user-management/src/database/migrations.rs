use mongodb::Database;
use bson::doc;

/// Initialize MongoDB database with collections and indexes
pub async fn initialize_database(database: &Database) -> Result<(), mongodb::error::Error> {
    tracing::info!("Initializing MongoDB database...");
    
    // Create collections if they don't exist
    let collections = ["users", "user_sessions", "user_preferences", "user_roles"];
    for collection_name in &collections {
        database.create_collection(collection_name, None).await?;
        tracing::info!("Created collection: {}", collection_name);
    }
    
    // Create indexes for better performance
    create_indexes(database).await?;
    
    tracing::info!("MongoDB database initialization completed successfully");
    Ok(())
}

/// Create database indexes
async fn create_indexes(database: &Database) -> Result<(), mongodb::error::Error> {
    // Users collection indexes
    let users_collection = database.collection::<crate::models::User>("users");
    
    // Username unique index
    users_collection.create_index(
        mongodb::IndexModel::builder()
            .keys(doc! { "username": 1 })
            .options(mongodb::options::IndexOptions::builder().unique(true).build())
            .build(),
        None,
    ).await?;
    
    // Email unique index
    users_collection.create_index(
        mongodb::IndexModel::builder()
            .keys(doc! { "email": 1 })
            .options(mongodb::options::IndexOptions::builder().unique(true).build())
            .build(),
        None,
    ).await?;
    
    // Status index
    users_collection.create_index(
        mongodb::IndexModel::builder()
            .keys(doc! { "status": 1 })
            .build(),
        None,
    ).await?;
    
    // Created at index
    users_collection.create_index(
        mongodb::IndexModel::builder()
            .keys(doc! { "created_at": 1 })
            .build(),
        None,
    ).await?;
    
    // Sessions collection indexes
    let sessions_collection = database.collection::<crate::models::UserSession>("user_sessions");
    
    // User ID index
    sessions_collection.create_index(
        mongodb::IndexModel::builder()
            .keys(doc! { "user_id": 1 })
            .build(),
        None,
    ).await?;
    
    // Refresh token unique index
    sessions_collection.create_index(
        mongodb::IndexModel::builder()
            .keys(doc! { "refresh_token": 1 })
            .options(mongodb::options::IndexOptions::builder().unique(true).build())
            .build(),
        None,
    ).await?;
    
    // Expires at index (for TTL)
    sessions_collection.create_index(
        mongodb::IndexModel::builder()
            .keys(doc! { "expires_at": 1 })
            .options(mongodb::options::IndexOptions::builder().expire_after(Some(std::time::Duration::from_secs(0))).build())
            .build(),
        None,
    ).await?;
    
    // Is active index
    sessions_collection.create_index(
        mongodb::IndexModel::builder()
            .keys(doc! { "is_active": 1 })
            .build(),
        None,
    ).await?;
    
    // Preferences collection indexes
    let prefs_collection = database.collection::<crate::models::UserPreferences>("user_preferences");
    
    // User ID unique index
    prefs_collection.create_index(
        mongodb::IndexModel::builder()
            .keys(doc! { "user_id": 1 })
            .options(mongodb::options::IndexOptions::builder().unique(true).build())
            .build(),
        None,
    ).await?;
    
    // Roles collection indexes
    let roles_collection = database.collection::<crate::models::UserRole>("user_roles");
    
    // User ID index
    roles_collection.create_index(
        mongodb::IndexModel::builder()
            .keys(doc! { "user_id": 1 })
            .build(),
        None,
    ).await?;
    
    // Role index
    roles_collection.create_index(
        mongodb::IndexModel::builder()
            .keys(doc! { "role": 1 })
            .build(),
        None,
    ).await?;
    
    // Is active index
    roles_collection.create_index(
        mongodb::IndexModel::builder()
            .keys(doc! { "is_active": 1 })
            .build(),
        None,
    ).await?;
    
    tracing::info!("Database indexes created successfully");
    Ok(())
}

// Unused utility functions removed for cleaner code
