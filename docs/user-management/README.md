# User Management Service - Chaos World

## 📋 Overview

User Management Service chịu trách nhiệm quản lý tất cả thông tin người dùng, authentication, authorization, và session management cho Chaos World MMORPG.

## 🎯 Responsibilities

### Core Functions
- **User Registration**: Đăng ký tài khoản mới
- **Authentication**: Xác thực người dùng (login/logout)
- **Authorization**: Phân quyền và kiểm soát truy cập
- **Profile Management**: Quản lý thông tin cá nhân
- **Session Management**: Quản lý phiên đăng nhập
- **Password Management**: Reset, change password

### Performance Requirements
- **Latency**: < 100ms cho authentication
- **Throughput**: 1,000+ TPS
- **Availability**: 99.9% uptime
- **Concurrency**: Handle 10,000+ concurrent users

## 🏗️ Architecture

### Technology Stack
```yaml
Language: Rust/Go
Framework: Axum/Gin
Database: PostgreSQL
Caching: Redis
Authentication: JWT + OAuth2
Password Hashing: Argon2id
```

### Core Components
```rust
pub struct UserManagementService {
    // Database
    user_repository: Arc<UserRepository>,
    session_repository: Arc<SessionRepository>,
    
    // Authentication
    auth_service: Arc<AuthService>,
    jwt_service: Arc<JwtService>,
    password_service: Arc<PasswordService>,
    
    // External services
    email_service: Arc<EmailService>,
    notification_service: Arc<NotificationService>,
    
    // Configuration
    config: UserServiceConfig,
}
```

## 🔐 Authentication & Authorization

### Authentication Flow
```mermaid
sequenceDiagram
    participant C as Client
    participant AG as API Gateway
    participant UMS as User Management
    participant DB as Database
    participant CB as Chaos Backend

    C->>AG: POST /auth/login
    AG->>UMS: Validate credentials
    UMS->>DB: Check user data
    UMS->>UMS: Verify password
    UMS->>UMS: Generate JWT token
    UMS->>DB: Store session
    UMS-->>AG: Return token + user info
    AG->>CB: Create game session
    AG-->>C: Login success
```

### JWT Token Structure
```json
{
  "header": {
    "alg": "HS256",
    "typ": "JWT"
  },
  "payload": {
    "user_id": "user_123",
    "username": "player_name",
    "email": "player@example.com",
    "roles": ["player"],
    "permissions": [
      "game:play",
      "shop:purchase",
      "profile:edit"
    ],
    "session_id": "session_456",
    "iat": 1640995200,
    "exp": 1640998800,
    "iss": "chaos-world"
  }
}
```

### Role-Based Access Control (RBAC)
```yaml
Roles:
  player:
    permissions:
      - game:play
      - shop:purchase
      - profile:edit
      - guild:join
  
  moderator:
    permissions:
      - game:play
      - shop:purchase
      - profile:edit
      - guild:join
      - chat:moderate
      - user:view_reports
  
  admin:
    permissions:
      - "*"  # All permissions
  
  developer:
    permissions:
      - game:play
      - shop:purchase
      - profile:edit
      - guild:join
      - admin:debug
      - admin:metrics
```

## 📊 Database Schema

### PostgreSQL Tables
```sql
-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100),
    avatar_url VARCHAR(500),
    status VARCHAR(20) DEFAULT 'active',
    email_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    last_login TIMESTAMP,
    login_count INTEGER DEFAULT 0
);

-- User sessions table
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    session_token VARCHAR(255) UNIQUE NOT NULL,
    refresh_token VARCHAR(255) UNIQUE NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    last_accessed TIMESTAMP DEFAULT NOW(),
    ip_address INET,
    user_agent TEXT,
    is_active BOOLEAN DEFAULT TRUE
);

-- User preferences table
CREATE TABLE user_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    language VARCHAR(10) DEFAULT 'en',
    timezone VARCHAR(50) DEFAULT 'UTC',
    notification_email BOOLEAN DEFAULT TRUE,
    notification_push BOOLEAN DEFAULT TRUE,
    privacy_level VARCHAR(20) DEFAULT 'public',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- User roles table
CREATE TABLE user_roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL,
    granted_by UUID REFERENCES users(id),
    granted_at TIMESTAMP DEFAULT NOW(),
    expires_at TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);
```

### Redis Cache Structure
```yaml
User Sessions:
  key: "session:{session_id}"
  value: { user_id, username, roles, permissions, expires_at }
  ttl: 3600 seconds

User Data:
  key: "user:{user_id}"
  value: { username, email, display_name, avatar_url, status }
  ttl: 1800 seconds

Rate Limiting:
  key: "rate_limit:{ip_address}:{endpoint}"
  value: { count, window_start }
  ttl: 3600 seconds
```

## 🔌 API Endpoints

### Authentication Endpoints
```yaml
POST /auth/register:
  Description: Register new user
  Request: 
    Content-Type: application/json
    Body: {
      "username": "string (3-50 chars)",
      "email": "string (valid email)",
      "password": "string (8-128 chars)",
      "display_name": "string (optional, 1-100 chars)",
      "agree_to_terms": "boolean (required)"
    }
  Response: 
    Success (201): {
      "success": true,
      "user": {
        "id": "uuid",
        "username": "string",
        "email": "string",
        "display_name": "string",
        "created_at": "timestamp"
      },
      "tokens": {
        "access_token": "jwt_token",
        "refresh_token": "jwt_token",
        "expires_at": "timestamp"
      }
    }
    Error (400/409): {
      "success": false,
      "error": "string",
      "details": "string (optional)"
    }
  Rate Limit: 5/minute per IP

POST /auth/login:
  Description: User login
  Request:
    Content-Type: application/json
    Body: {
      "username_or_email": "string",
      "password": "string",
      "remember_me": "boolean (optional, default: false)"
    }
  Response:
    Success (200): {
      "success": true,
      "user": {
        "id": "uuid",
        "username": "string",
        "email": "string",
        "display_name": "string",
        "avatar_url": "string (optional)",
        "last_login": "timestamp"
      },
      "tokens": {
        "access_token": "jwt_token",
        "refresh_token": "jwt_token",
        "expires_at": "timestamp"
      }
    }
    Error (401/400): {
      "success": false,
      "error": "string",
      "details": "string (optional)"
    }
  Rate Limit: 10/minute per IP

GET /auth/me:
  Description: Get current user profile
  Request:
    Headers: {
      "Authorization": "Bearer <access_token>"
    }
  Response:
    Success (200): {
      "success": true,
      "user": {
        "id": "uuid",
        "username": "string",
        "email": "string",
        "display_name": "string",
        "avatar_url": "string (optional)",
        "status": "string",
        "email_verified": "boolean",
        "created_at": "timestamp",
        "updated_at": "timestamp",
        "last_login": "timestamp",
        "login_count": "integer"
      }
    }
    Error (401): {
      "success": false,
      "error": "Unauthorized",
      "details": "Invalid or expired token"
    }
  Rate Limit: 1000/hour per user

POST /auth/refresh:
  Description: Refresh JWT token
  Request: 
    Content-Type: application/json
    Body: {
      "refresh_token": "string"
    }
  Response: 
    Success (200): {
      "success": true,
      "tokens": {
        "access_token": "jwt_token",
        "refresh_token": "jwt_token",
        "expires_at": "timestamp"
      }
    }
    Error (401): {
      "success": false,
      "error": "Invalid refresh token"
    }
  Rate Limit: 20/minute per user

POST /auth/logout:
  Description: User logout
  Request: 
    Headers: {
      "Authorization": "Bearer <access_token>"
    }
  Response: 
    Success (200): {
      "success": true,
      "message": "Logged out successfully"
    }
    Error (401): {
      "success": false,
      "error": "Unauthorized"
    }
  Rate Limit: 100/minute per user

POST /auth/logout-all:
  Description: Logout from all devices
  Request: 
    Headers: {
      "Authorization": "Bearer <access_token>"
    }
  Response: 
    Success (200): {
      "success": true,
      "message": "Logged out from all devices"
    }
    Error (401): {
      "success": false,
      "error": "Unauthorized"
    }
  Rate Limit: 10/minute per user
```

### User Management Endpoints
```yaml
GET /users/profile:
  Description: Get user profile
  Request: { token }
  Response: { user_id, username, email, display_name, avatar_url, created_at }
  Rate Limit: 1000/hour per user

PUT /users/profile:
  Description: Update user profile
  Request: { token, display_name, avatar_url }
  Response: { success: true, updated_fields }
  Rate Limit: 100/hour per user

GET /users/preferences:
  Description: Get user preferences
  Request: { token }
  Response: { language, timezone, notifications, privacy }
  Rate Limit: 1000/hour per user

PUT /users/preferences:
  Description: Update user preferences
  Request: { token, preferences }
  Response: { success: true }
  Rate Limit: 100/hour per user
```

### Password Management Endpoints
```yaml
POST /auth/forgot-password:
  Description: Request password reset
  Request: { email }
  Response: { success: true, message }
  Rate Limit: 3/hour per email

POST /auth/reset-password:
  Description: Reset password with token
  Request: { token, new_password }
  Response: { success: true }
  Rate Limit: 5/hour per token

POST /auth/change-password:
  Description: Change password (authenticated)
  Request: { token, current_password, new_password }
  Response: { success: true }
  Rate Limit: 10/hour per user
```

### Admin Endpoints
```yaml
GET /admin/users:
  Description: List all users (admin only)
  Request: { token, page, limit, filter }
  Response: { users: [...], total, page, limit }
  Rate Limit: 1000/hour per admin

PUT /admin/users/{user_id}/status:
  Description: Update user status (admin only)
  Request: { token, status }
  Response: { success: true }
  Rate Limit: 100/hour per admin

POST /admin/users/{user_id}/roles:
  Description: Assign role to user (admin only)
  Request: { token, role, expires_at }
  Response: { success: true }
  Rate Limit: 100/hour per admin
```

## 🔒 Security Features

### Password Security
```yaml
Hashing Algorithm: Argon2id
Parameters:
  - memory: 64 MB
  - iterations: 3
  - parallelism: 4
  - salt_length: 16 bytes

Password Requirements:
  - Minimum length: 8 characters
  - Must contain: uppercase, lowercase, number
  - Must not contain: common passwords
  - Must not be: username or email
```

### Session Security
```yaml
JWT Security:
  - Algorithm: HS256
  - Secret: 256-bit random key
  - Expiration: 1 hour (access), 7 days (refresh)
  - Issuer: chaos-world
  - Audience: chaos-world-api

Session Management:
  - One session per device
  - Automatic cleanup of expired sessions
  - Session invalidation on password change
  - IP address validation
```

### Rate Limiting
```yaml
Authentication Endpoints:
  - Login: 10/minute per IP
  - Register: 5/minute per IP
  - Password reset: 3/hour per email
  - Refresh token: 20/minute per user

User Management Endpoints:
  - Profile updates: 100/hour per user
  - Password change: 10/hour per user
  - Logout: 100/minute per user

Admin Endpoints:
  - User listing: 1000/hour per admin
  - User management: 100/hour per admin
```

### Input Validation
```yaml
Username:
  - Length: 3-50 characters
  - Allowed: letters, numbers, underscore, hyphen
  - Not allowed: spaces, special characters
  - Case insensitive

Email:
  - Valid email format
  - Maximum length: 255 characters
  - Domain validation

Password:
  - Length: 8-128 characters
  - Complexity requirements
  - Common password check
```

## 📊 Monitoring & Analytics

### Metrics
```yaml
Authentication Metrics:
  - Login success rate
  - Login failure rate
  - Registration rate
  - Password reset rate
  - Session creation rate

User Metrics:
  - Active users (daily/monthly)
  - New user registrations
  - User retention rate
  - Profile update frequency

Security Metrics:
  - Failed login attempts
  - Password reset requests
  - Suspicious activity
  - Rate limit violations
```

### Logging
```yaml
Security Events:
  - Login attempts (success/failure)
  - Password changes
  - Profile updates
  - Role changes
  - Session creation/destruction

Audit Trail:
  - User ID
  - Action performed
  - Timestamp
  - IP address
  - User agent
  - Result (success/failure)
```

### Alerting
```yaml
Critical Alerts:
  - High failed login rate
  - Multiple password reset attempts
  - Unusual login patterns
  - Database connection failures

Warning Alerts:
  - High registration rate
  - Profile update spikes
  - Session creation spikes
  - Rate limit violations
```

## 🧪 Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_user_registration() {
        // Test user registration flow
    }
    
    #[tokio::test]
    async fn test_authentication() {
        // Test login/logout flow
    }
    
    #[tokio::test]
    async fn test_password_hashing() {
        // Test password hashing and verification
    }
    
    #[tokio::test]
    async fn test_jwt_token_generation() {
        // Test JWT token creation and validation
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_complete_auth_flow() {
    // Test complete authentication flow
    let client = reqwest::Client::new();
    
    // Register user
    let register_response = client
        .post("http://localhost:8082/auth/register")
        .json(&RegisterRequest {
            username: "test_user",
            email: "test@example.com",
            password: "TestPassword123",
            display_name: "Test User",
        })
        .send()
        .await
        .unwrap();
    
    assert_eq!(register_response.status(), 200);
    
    // Login user
    let login_response = client
        .post("http://localhost:8082/auth/login")
        .json(&LoginRequest {
            username: "test_user",
            password: "TestPassword123",
        })
        .send()
        .await
        .unwrap();
    
    assert_eq!(login_response.status(), 200);
}
```

### Load Tests
```yaml
# k6 load test
import http from 'k6/http';
import { check } from 'k6';

export let options = {
  stages: [
    { duration: '2m', target: 100 },
    { duration: '5m', target: 100 },
    { duration: '2m', target: 200 },
    { duration: '5m', target: 200 },
    { duration: '2m', target: 0 },
  ],
};

export default function() {
  let response = http.post('http://localhost:8082/auth/login', {
    username: 'test_user',
    password: 'test_password'
  });
  
  check(response, {
    'status is 200': (r) => r.status === 200,
    'response time < 100ms': (r) => r.timings.duration < 100,
  });
}
```

## 🚀 Deployment

### Docker
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/user-management /usr/local/bin/
EXPOSE 8082
CMD ["user-management"]
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: user-management
spec:
  replicas: 3
  selector:
    matchLabels:
      app: user-management
  template:
    metadata:
      labels:
        app: user-management
    spec:
      containers:
      - name: user-management
        image: user-management:latest
        ports:
        - containerPort: 8082
        env:
        - name: DATABASE_URL
          value: "postgresql://user:pass@postgres:5432/user_db"
        - name: REDIS_URL
          value: "redis://redis:6379"
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: jwt-secret
              key: secret
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
```

### Database Migration
```sql
-- Migration script
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create tables
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100),
    avatar_url VARCHAR(500),
    status VARCHAR(20) DEFAULT 'active',
    email_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    last_login TIMESTAMP,
    login_count INTEGER DEFAULT 0
);

-- Create indexes
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_status ON users(status);
CREATE INDEX idx_users_created_at ON users(created_at);

-- Create triggers for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at 
    BEFORE UPDATE ON users 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();
```

## 🔧 Configuration

### Environment Variables
```yaml
# Database Configuration
DATABASE_URL=postgresql://user:pass@localhost:5432/user_db
DATABASE_POOL_SIZE=10
DATABASE_TIMEOUT=30s

# Redis Configuration
REDIS_URL=redis://localhost:6379
REDIS_PASSWORD=secret
REDIS_DB=0

# JWT Configuration
JWT_SECRET=your-secret-key
JWT_ACCESS_EXPIRY=3600
JWT_REFRESH_EXPIRY=604800

# Server Configuration
SERVER_PORT=8082
SERVER_HOST=0.0.0.0
SERVER_WORKERS=4

# Email Configuration
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password

# Rate Limiting
RATE_LIMIT_ENABLED=true
RATE_LIMIT_REDIS_URL=redis://localhost:6379
RATE_LIMIT_WINDOW=3600
```

### Configuration File
```yaml
# user-management-config.yaml
server:
  port: 8082
  host: "0.0.0.0"
  workers: 4
  max_connections: 10000

database:
  url: "postgresql://user:pass@localhost:5432/user_db"
  pool_size: 10
  timeout: "30s"
  ssl_mode: "prefer"

redis:
  url: "redis://localhost:6379"
  password: "secret"
  db: 0
  pool_size: 100

jwt:
  secret: "your-secret-key"
  access_expiry: 3600
  refresh_expiry: 604800
  issuer: "chaos-world"
  audience: "chaos-world-api"

password:
  min_length: 8
  require_uppercase: true
  require_lowercase: true
  require_numbers: true
  require_special: false
  max_length: 128

rate_limiting:
  enabled: true
  window: 3600
  max_requests: 1000
  login_attempts: 10
  password_reset: 3
```

## 🔗 Related Services

- [API Gateway](./api-gateway/README.md) - Entry point for all requests
- [Chaos Backend](./chaos-backend/README.md) - Game Logic Core
- [Payment Service](./payment-service/README.md) - Transaction Service
- [Microservices Architecture](./microservices-architecture/README.md) - Overall Architecture
