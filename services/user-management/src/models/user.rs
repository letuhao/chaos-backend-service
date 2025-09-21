use serde::{Deserialize, Serialize, Serializer, Deserializer};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use bson::{Binary, Bson};

/// Serialize UUID as BSON Binary for MongoDB
fn serialize_uuid_as_binary<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let uuid_bytes = uuid.as_bytes();
    let binary = Binary {
        subtype: bson::spec::BinarySubtype::UuidOld,
        bytes: uuid_bytes.to_vec(),
    };
    Bson::Binary(binary).serialize(serializer)
}

/// Deserialize UUID from BSON Binary for MongoDB
fn deserialize_uuid_from_binary<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    let bson = Bson::deserialize(deserializer)?;
    match bson {
        Bson::Binary(binary) => {
            if binary.bytes.len() == 16 {
                let mut bytes = [0u8; 16];
                bytes.copy_from_slice(&binary.bytes);
                Ok(Uuid::from_bytes(bytes))
            } else {
                Err(serde::de::Error::custom("Invalid UUID binary length"))
            }
        }
        _ => Err(serde::de::Error::custom("Expected BSON Binary for UUID"))
    }
}

/// User status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    Banned,
    PendingVerification,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Active => write!(f, "active"),
            UserStatus::Inactive => write!(f, "inactive"),
            UserStatus::Suspended => write!(f, "suspended"),
            UserStatus::Banned => write!(f, "banned"),
            UserStatus::PendingVerification => write!(f, "pending_verification"),
        }
    }
}

impl std::str::FromStr for UserStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(UserStatus::Active),
            "inactive" => Ok(UserStatus::Inactive),
            "suspended" => Ok(UserStatus::Suspended),
            "banned" => Ok(UserStatus::Banned),
            "pending_verification" => Ok(UserStatus::PendingVerification),
            _ => Err(format!("Invalid user status: {}", s)),
        }
    }
}

/// User entity representing a user in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(serialize_with = "serialize_uuid_as_binary", deserialize_with = "deserialize_uuid_from_binary")]
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub login_count: i32,
}

/// User session entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub user_fingerprint: Option<String>,
    pub is_active: bool,
}

/// User preferences entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub id: Uuid,
    pub user_id: Uuid,
    pub language: String,
    pub timezone: String,
    pub notification_email: bool,
    pub notification_push: bool,
    pub privacy_level: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User role entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub granted_by: Option<Uuid>,
    pub granted_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

/// Public user information (without sensitive data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub login_count: i32,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            status: user.status,
            email_verified: user.email_verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
            last_login: user.last_login,
            login_count: user.login_count,
        }
    }
}

/// JWT token claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub session_id: Uuid,
    pub iat: i64,
    pub exp: i64,
    pub iss: String,
    pub aud: String,
}

/// Token response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

/// Authentication response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    pub user: PublicUser,
    pub tokens: TokenResponse,
}

/// Error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub details: Option<String>,
}

impl ErrorResponse {
    pub fn new(error: &str) -> Self {
        Self {
            success: false,
            error: error.to_string(),
            details: None,
        }
    }

    pub fn with_details(error: &str, details: &str) -> Self {
        Self {
            success: false,
            error: error.to_string(),
            details: Some(details.to_string()),
        }
    }
}

/// Success response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}

impl SuccessResponse {
    pub fn new(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
        }
    }
}
