use serde::{Deserialize, Serialize};
use validator::Validate;

/// User registration request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: String,
    
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email must be less than 255 characters"))]
    pub email: String,
    
    #[validate(length(min = 8, max = 128, message = "Password must be between 8 and 128 characters"))]
    pub password: String,
    
    #[validate(length(max = 100, message = "Display name must be less than 100 characters"))]
    #[serde(alias = "displayName")]
    pub display_name: Option<String>,
    
    #[validate(must_match(other = "agree_to_terms", message = "You must agree to the terms and conditions"))]
    #[serde(alias = "agreeToTerms")]
    pub agree_to_terms: bool,
}

/// User login request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "Username or email is required"))]
    #[serde(alias = "username")]
    pub username_or_email: String,
    
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
    
    #[serde(alias = "rememberMe")]
    pub remember_me: Option<bool>,
}

/// Refresh token request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}

/// Change password request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, message = "Current password is required"))]
    pub current_password: String,
    
    #[validate(length(min = 8, max = 128, message = "New password must be between 8 and 128 characters"))]
    pub new_password: String,
}

/// Update profile request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(max = 100, message = "Display name must be less than 100 characters"))]
    pub display_name: Option<String>,
    
    #[validate(url(message = "Invalid avatar URL format"))]
    #[validate(length(max = 500, message = "Avatar URL must be less than 500 characters"))]
    pub avatar_url: Option<String>,
}

/// Update preferences request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdatePreferencesRequest {
    #[validate(length(min = 2, max = 10, message = "Language code must be between 2 and 10 characters"))]
    pub language: Option<String>,
    
    #[validate(length(max = 50, message = "Timezone must be less than 50 characters"))]
    pub timezone: Option<String>,
    
    pub notification_email: Option<bool>,
    pub notification_push: Option<bool>,
    
    pub privacy_level: Option<String>,
}

/// User profile response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub success: bool,
    pub user: crate::models::user::PublicUser,
}

/// User preferences response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferencesResponse {
    pub success: bool,
    pub preferences: crate::models::user::UserPreferences,
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
            sort_by: Some("created_at".to_string()),
            sort_order: Some("desc".to_string()),
        }
    }
}

/// Paginated response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub success: bool,
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
}

/// Pagination information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

/// User filter parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFilterParams {
    pub status: Option<String>,
    pub email_verified: Option<bool>,
    pub created_after: Option<chrono::DateTime<chrono::Utc>>,
    pub created_before: Option<chrono::DateTime<chrono::Utc>>,
    pub search: Option<String>,
}

/// Admin user management request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AdminUpdateUserRequest {
    pub status: Option<String>,
    pub email_verified: Option<bool>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

/// Admin assign role request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AdminAssignRoleRequest {
    #[validate(length(min = 1, message = "Role is required"))]
    pub role: String,
    
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Rate limit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub limit: u32,
    pub remaining: u32,
    pub reset_time: chrono::DateTime<chrono::Utc>,
}

/// Rate limit response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitResponse {
    pub success: bool,
    pub error: String,
    pub rate_limit: RateLimitInfo,
}

// Unused functions removed for cleaner code
