use crate::models::{User, UserSession, UserPreferences};
use crate::config::UserServiceConfig;
use mongodb::{Client, Database, Collection};
use bson::doc;
use uuid::Uuid;
use chrono::Utc;

/// User repository for MongoDB operations
#[allow(dead_code)]
pub struct UserRepository {
    collection: Collection<User>,
}

#[allow(dead_code)]
impl UserRepository {
    /// Create a new user repository
    pub fn new(database: &Database) -> Self {
        Self {
            collection: database.collection::<User>("users"),
        }
    }

    /// Create a new user
    pub async fn create_user(&self, user: &User) -> Result<User, mongodb::error::Error> {
        tracing::info!("Inserting user into MongoDB: {}", user.username);
        let result = self.collection.insert_one(user, None).await?;
        tracing::info!("User inserted with ID: {:?}", result.inserted_id);
        Ok(user.clone())
    }

    /// Find user by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, mongodb::error::Error> {
        use bson::{Binary, Bson};
        let uuid_bytes = id.as_bytes();
        let binary = Binary {
            subtype: bson::spec::BinarySubtype::UuidOld,
            bytes: uuid_bytes.to_vec(),
        };
        let filter = doc! { "id": Bson::Binary(binary) };
        tracing::info!("Searching for user with ID: {}", id);
        tracing::info!("Filter: {:?}", filter);
        let result = self.collection.find_one(filter, None).await?;
        tracing::info!("User found: {:?}", result.is_some());
        Ok(result)
    }

    /// Find user by username
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, mongodb::error::Error> {
        let filter = doc! { "username": username };
        let result = self.collection.find_one(filter, None).await?;
        Ok(result)
    }

    /// Find user by email
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, mongodb::error::Error> {
        let filter = doc! { "email": email };
        let result = self.collection.find_one(filter, None).await?;
        Ok(result)
    }

    /// Find user by username or email
    pub async fn find_by_username_or_email(&self, username_or_email: &str) -> Result<Option<User>, mongodb::error::Error> {
        let filter = doc! { 
            "$or": [
                { "username": username_or_email },
                { "email": username_or_email }
            ]
        };
        let result = self.collection.find_one(filter, None).await?;
        Ok(result)
    }

    /// Update user
    pub async fn update_user(&self, user: &User) -> Result<User, mongodb::error::Error> {
        use bson::{Binary, Bson};
        let uuid_bytes = user.id.as_bytes();
        let binary = Binary {
            subtype: bson::spec::BinarySubtype::UuidOld,
            bytes: uuid_bytes.to_vec(),
        };
        let filter = doc! { "id": Bson::Binary(binary) };
        let update = doc! { 
            "$set": {
                "username": &user.username,
                "email": &user.email,
                "password_hash": &user.password_hash,
                "display_name": user.display_name.as_ref(),
                "avatar_url": user.avatar_url.as_ref(),
                "status": user.status.to_string(),
                "email_verified": user.email_verified,
                "updated_at": user.updated_at.to_rfc3339(),
                "last_login": user.last_login.map(|dt| dt.to_rfc3339()),
                "login_count": user.login_count
            }
        };
        
        self.collection.update_one(filter, update, None).await?;
        Ok(user.clone())
    }

    /// Delete user
    pub async fn delete_user(&self, id: Uuid) -> Result<bool, mongodb::error::Error> {
        use bson::{Binary, Bson};
        let uuid_bytes = id.as_bytes();
        let binary = Binary {
            subtype: bson::spec::BinarySubtype::UuidOld,
            bytes: uuid_bytes.to_vec(),
        };
        let filter = doc! { "id": Bson::Binary(binary) };
        let result = self.collection.delete_one(filter, None).await?;
        Ok(result.deleted_count > 0)
    }

    /// Check if username exists
    pub async fn username_exists(&self, username: &str) -> Result<bool, mongodb::error::Error> {
        let filter = doc! { "username": username };
        let count = self.collection.count_documents(filter, None).await?;
        Ok(count > 0)
    }

    /// Check if email exists
    pub async fn email_exists(&self, email: &str) -> Result<bool, mongodb::error::Error> {
        let filter = doc! { "email": email };
        let count = self.collection.count_documents(filter, None).await?;
        Ok(count > 0)
    }

    /// Get all users with pagination
    pub async fn get_users_paginated(
        &self, 
        page: u32, 
        limit: u32
    ) -> Result<(Vec<User>, u64), mongodb::error::Error> {
        let skip = (page - 1) * limit;
        let total = self.collection.count_documents(None, None).await?;
        
        let mut cursor = self.collection
            .find(None, None)
            .await?;
        
        let mut users = Vec::new();
        let mut count = 0;
        while cursor.advance().await? {
            if count >= skip && users.len() < limit as usize {
                users.push(cursor.deserialize_current()?);
            }
            count += 1;
        }
        
        Ok((users, total))
    }
}

/// Session repository for MongoDB operations
#[allow(dead_code)]
pub struct SessionRepository {
    collection: Collection<UserSession>,
}

#[allow(dead_code)]
impl SessionRepository {
    /// Create a new session repository
    pub fn new(database: &Database) -> Self {
        Self {
            collection: database.collection::<UserSession>("user_sessions"),
        }
    }

    /// Create a new session
    pub async fn create_session(&self, session: &UserSession) -> Result<UserSession, mongodb::error::Error> {
        tracing::info!("Inserting session into MongoDB: {:?}", session);
        let result = self.collection.insert_one(session, None).await?;
        tracing::info!("Session inserted with ID: {:?}", result.inserted_id);
        Ok(session.clone())
    }

    /// Find session by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<UserSession>, mongodb::error::Error> {
        let filter = doc! { "id": id.to_string() };
        let result = self.collection.find_one(filter, None).await?;
        Ok(result)
    }

    /// Find session by refresh token
    pub async fn find_by_refresh_token(&self, refresh_token: &str) -> Result<Option<UserSession>, mongodb::error::Error> {
        let filter = doc! { 
            "refresh_token": refresh_token,
            "is_active": true
        };
        let result = self.collection.find_one(filter, None).await?;
        Ok(result)
    }

    /// Update session
    pub async fn update_session(&self, session: &UserSession) -> Result<UserSession, mongodb::error::Error> {
        let filter = doc! { "id": session.id.to_string() };
        let update = doc! { 
            "$set": {
                "session_token": &session.session_token,
                "refresh_token": &session.refresh_token,
                "expires_at": bson::DateTime::from_system_time(session.expires_at.into()),
                "last_accessed": bson::DateTime::from_system_time(session.last_accessed.into()),
                "ip_address": session.ip_address.as_ref(),
                "user_agent": session.user_agent.as_ref(),
                "user_fingerprint": session.user_fingerprint.as_ref(),
                "is_active": session.is_active
            }
        };
        
        self.collection.update_one(filter, update, None).await?;
        Ok(session.clone())
    }

    /// Deactivate session
    pub async fn deactivate_session(&self, id: Uuid) -> Result<bool, mongodb::error::Error> {
        let filter = doc! { "id": id.to_string() };
        let update = doc! { "$set": { "is_active": false } };
        let result = self.collection.update_one(filter, update, None).await?;
        Ok(result.modified_count > 0)
    }

    /// Deactivate all sessions for user
    pub async fn deactivate_all_user_sessions(&self, user_id: Uuid) -> Result<u64, mongodb::error::Error> {
        let filter = doc! { "user_id": user_id.to_string() };
        let update = doc! { "$set": { "is_active": false } };
        let result = self.collection.update_many(filter, update, None).await?;
        Ok(result.modified_count)
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) -> Result<u64, mongodb::error::Error> {
        let filter = doc! { 
            "expires_at": { "$lt": bson::DateTime::from_system_time(Utc::now().into()) }
        };
        let result = self.collection.delete_many(filter, None).await?;
        Ok(result.deleted_count)
    }
}

/// Preferences repository for MongoDB operations
#[allow(dead_code)]
pub struct PreferencesRepository {
    collection: Collection<UserPreferences>,
}

#[allow(dead_code)]
impl PreferencesRepository {
    /// Create a new preferences repository
    pub fn new(database: &Database) -> Self {
        Self {
            collection: database.collection::<UserPreferences>("user_preferences"),
        }
    }

    /// Create user preferences
    pub async fn create_preferences(&self, preferences: &UserPreferences) -> Result<UserPreferences, mongodb::error::Error> {
        self.collection.insert_one(preferences, None).await?;
        Ok(preferences.clone())
    }

    /// Get user preferences
    pub async fn get_preferences(&self, user_id: Uuid) -> Result<Option<UserPreferences>, mongodb::error::Error> {
        let filter = doc! { "user_id": user_id.to_string() };
        let result = self.collection.find_one(filter, None).await?;
        Ok(result)
    }

    /// Update user preferences
    pub async fn update_preferences(&self, preferences: &UserPreferences) -> Result<UserPreferences, mongodb::error::Error> {
        let filter = doc! { "user_id": preferences.user_id.to_string() };
        let update = doc! { 
            "$set": {
                "language": &preferences.language,
                "timezone": &preferences.timezone,
                "notification_email": preferences.notification_email,
                "notification_push": preferences.notification_push,
                "privacy_level": &preferences.privacy_level,
                "updated_at": bson::DateTime::from_system_time(preferences.updated_at.into())
            }
        };
        
        self.collection.update_one(filter, update, None).await?;
        Ok(preferences.clone())
    }
}

/// Database connection manager for MongoDB
#[allow(dead_code)]
pub struct DatabaseManager {
    pub user_repo: UserRepository,
    pub session_repo: SessionRepository,
    pub preferences_repo: PreferencesRepository,
    pub database: Database,
}

#[allow(dead_code)]
impl DatabaseManager {
    /// Create a new database manager
    pub async fn new(config: &UserServiceConfig) -> Result<Self, mongodb::error::Error> {
        let client = Client::with_uri_str(&config.database.url).await?;
        let database = client.database("chaos_user_management");
        
        Ok(Self {
            user_repo: UserRepository::new(&database),
            session_repo: SessionRepository::new(&database),
            preferences_repo: PreferencesRepository::new(&database),
            database,
        })
    }

    /// Create indexes for better performance
    pub async fn create_indexes(&self) -> Result<(), mongodb::error::Error> {
        // Users collection indexes
        self.database.collection::<User>("users").create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "username": 1 })
                .options(mongodb::options::IndexOptions::builder().unique(true).build())
                .build(),
            None,
        ).await?;

        self.database.collection::<User>("users").create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "email": 1 })
                .options(mongodb::options::IndexOptions::builder().unique(true).build())
                .build(),
            None,
        ).await?;

        self.database.collection::<User>("users").create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "status": 1 })
                .build(),
            None,
        ).await?;

        // Sessions collection indexes
        self.database.collection::<UserSession>("user_sessions").create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "user_id": 1 })
                .build(),
            None,
        ).await?;

        self.database.collection::<UserSession>("user_sessions").create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "refresh_token": 1 })
                .options(mongodb::options::IndexOptions::builder().unique(true).build())
                .build(),
            None,
        ).await?;

        self.database.collection::<UserSession>("user_sessions").create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "expires_at": 1 })
                .build(),
            None,
        ).await?;

        // Preferences collection indexes
        self.database.collection::<UserPreferences>("user_preferences").create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "user_id": 1 })
                .options(mongodb::options::IndexOptions::builder().unique(true).build())
                .build(),
            None,
        ).await?;

        Ok(())
    }

    /// Check if database is accessible
    pub async fn check_database_connection(&self) -> Result<(), mongodb::error::Error> {
        self.database.run_command(doc! { "ping": 1 }, None).await?;
        Ok(())
    }
}