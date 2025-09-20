# Notification Service - Chaos World

## 📋 Overview

Notification Service chịu trách nhiệm gửi tất cả các loại thông báo đến người chơi, bao gồm push notifications, email, SMS, và in-game notifications.

## 🎯 Responsibilities

### Core Functions
- **Push Notifications**: Mobile và web push notifications
- **Email Notifications**: Transactional và marketing emails
- **SMS Notifications**: Critical alerts và 2FA
- **In-Game Notifications**: Real-time game notifications
- **Webhook Notifications**: Third-party integrations
- **Notification Management**: Preferences và delivery tracking

### Performance Requirements
- **Latency**: < 100ms cho in-game notifications
- **Throughput**: 50,000+ notifications/minute
- **Availability**: 99.9% uptime
- **Concurrency**: Handle 100,000+ concurrent notifications

## 🏗️ Architecture

### Technology Stack
```yaml
Language: Rust/Go
Framework: Axum/Gin
Message Queue: Apache Kafka, Redis
Push Services: Firebase FCM, Apple APNs
Email: SendGrid, AWS SES
SMS: Twilio, AWS SNS
WebSocket: Socket.IO, WebSocket
```

### Core Components
```rust
pub struct NotificationService {
    // Message Queue
    kafka_producer: Arc<KafkaProducer>,
    redis_client: Arc<RedisClient>,
    
    // Push Services
    firebase_client: Arc<FirebaseClient>,
    apns_client: Arc<APNSClient>,
    
    // Email Services
    sendgrid_client: Arc<SendGridClient>,
    ses_client: Arc<SESClient>,
    
    // SMS Services
    twilio_client: Arc<TwilioClient>,
    sns_client: Arc<SNSClient>,
    
    // WebSocket
    websocket_manager: Arc<WebSocketManager>,
    
    // Database
    notification_repository: Arc<NotificationRepository>,
    preference_repository: Arc<PreferenceRepository>,
    
    // Configuration
    config: NotificationConfig,
}
```

## 📱 Notification Types

### Push Notifications
```yaml
Mobile Push:
  - Game updates
  - Friend requests
  - Guild invitations
  - Event notifications
  - Marketing campaigns

Web Push:
  - Browser notifications
  - Real-time alerts
  - System notifications
  - Social updates
```

### Email Notifications
```yaml
Transactional Emails:
  - Account verification
  - Password reset
  - Purchase confirmations
  - Security alerts
  - Account changes

Marketing Emails:
  - Newsletter
  - Promotional offers
  - Event announcements
  - Product updates
  - Community news
```

### SMS Notifications
```yaml
Critical Alerts:
  - Security breaches
  - Account lockouts
  - Payment failures
  - System outages

2FA:
  - Login verification
  - Transaction verification
  - Password change verification
  - Account recovery
```

### In-Game Notifications
```yaml
Real-time Notifications:
  - Combat alerts
  - Quest updates
  - Guild messages
  - System announcements
  - Friend online status

Persistent Notifications:
  - Achievement unlocks
  - Level up notifications
  - Item rewards
  - Event participation
  - Social interactions
```

## 🗄️ Database Schema

### PostgreSQL Tables
```sql
-- Notifications table
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    notification_type VARCHAR(50) NOT NULL,
    channel VARCHAR(20) NOT NULL, -- 'push', 'email', 'sms', 'in_game'
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    data JSONB,
    status VARCHAR(20) DEFAULT 'pending', -- 'pending', 'sent', 'delivered', 'failed'
    priority VARCHAR(10) DEFAULT 'normal', -- 'low', 'normal', 'high', 'urgent'
    scheduled_at TIMESTAMP,
    sent_at TIMESTAMP,
    delivered_at TIMESTAMP,
    failure_reason TEXT,
    retry_count INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Notification preferences table
CREATE TABLE notification_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID UNIQUE NOT NULL,
    push_enabled BOOLEAN DEFAULT TRUE,
    email_enabled BOOLEAN DEFAULT TRUE,
    sms_enabled BOOLEAN DEFAULT FALSE,
    in_game_enabled BOOLEAN DEFAULT TRUE,
    marketing_emails BOOLEAN DEFAULT TRUE,
    friend_requests BOOLEAN DEFAULT TRUE,
    guild_invitations BOOLEAN DEFAULT TRUE,
    system_alerts BOOLEAN DEFAULT TRUE,
    quiet_hours_start TIME,
    quiet_hours_end TIME,
    timezone VARCHAR(50) DEFAULT 'UTC',
    language VARCHAR(10) DEFAULT 'en',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Notification templates table
CREATE TABLE notification_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    template_name VARCHAR(100) UNIQUE NOT NULL,
    notification_type VARCHAR(50) NOT NULL,
    channel VARCHAR(20) NOT NULL,
    subject_template TEXT,
    body_template TEXT NOT NULL,
    variables JSONB,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Notification delivery logs table
CREATE TABLE notification_delivery_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    notification_id UUID REFERENCES notifications(id) ON DELETE CASCADE,
    delivery_attempt INTEGER NOT NULL,
    status VARCHAR(20) NOT NULL,
    provider_response JSONB,
    error_message TEXT,
    delivered_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);
```

### Redis Cache Structure
```yaml
User Preferences:
  key: "notification_prefs:{user_id}"
  value: { push_enabled, email_enabled, sms_enabled, ... }
  ttl: 3600 seconds

Notification Queue:
  key: "notification_queue:{priority}"
  value: { notification_id, user_id, type, data }
  ttl: 86400 seconds

Rate Limiting:
  key: "rate_limit:notification:{user_id}:{type}"
  value: { count, window_start }
  ttl: 3600 seconds
```

## 🔌 API Endpoints

### Notification Endpoints
```yaml
POST /notifications/send:
  Description: Send notification to user
  Request: { 
    user_id, 
    type, 
    channel, 
    title, 
    message, 
    data, 
    priority 
  }
  Response: { 
    notification_id, 
    status 
  }
  Rate Limit: 100/minute per user

POST /notifications/broadcast:
  Description: Send notification to multiple users
  Request: { 
    user_ids, 
    type, 
    channel, 
    title, 
    message, 
    data 
  }
  Response: { 
    notification_ids: [...], 
    status 
  }
  Rate Limit: 10/minute per admin

GET /notifications/history:
  Description: Get notification history
  Request: { 
    user_id, 
    page, 
    limit, 
    type, 
    status 
  }
  Response: { 
    notifications: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user
```

### Preference Endpoints
```yaml
GET /preferences:
  Description: Get notification preferences
  Request: { user_id }
  Response: { 
    push_enabled, 
    email_enabled, 
    sms_enabled, 
    in_game_enabled,
    marketing_emails,
    quiet_hours 
  }
  Rate Limit: 1000/hour per user

PUT /preferences:
  Description: Update notification preferences
  Request: { 
    user_id, 
    preferences 
  }
  Response: { 
    success: true, 
    updated_preferences 
  }
  Rate Limit: 100/hour per user

POST /preferences/unsubscribe:
  Description: Unsubscribe from notifications
  Request: { 
    user_id, 
    token, 
    channel 
  }
  Response: { 
    success: true 
  }
  Rate Limit: 10/minute per user
```

### Template Endpoints
```yaml
GET /templates:
  Description: Get notification templates
  Request: { type, channel, active }
  Response: { 
    templates: [...], 
    total 
  }
  Rate Limit: 1000/hour per admin

POST /templates:
  Description: Create notification template
  Request: { 
    template_name, 
    type, 
    channel, 
    subject_template, 
    body_template, 
    variables 
  }
  Response: { 
    template_id, 
    status: 'created' 
  }
  Rate Limit: 100/hour per admin

PUT /templates/{template_id}:
  Description: Update notification template
  Request: { 
    template_id, 
    updates 
  }
  Response: { 
    success: true 
  }
  Rate Limit: 100/hour per admin
```

## 📊 Notification Flow

### Real-time Notification Flow
```mermaid
sequenceDiagram
    participant CB as Chaos Backend
    participant NS as Notification Service
    participant MQ as Message Queue
    participant WS as WebSocket
    participant C as Client

    CB->>NS: Send notification request
    NS->>NS: Process notification
    NS->>MQ: Queue notification
    NS->>WS: Send real-time notification
    WS->>C: Deliver notification
    NS->>NS: Update delivery status
    NS-->>CB: Notification sent
```

### Batch Notification Flow
```mermaid
sequenceDiagram
    participant NS as Notification Service
    participant MQ as Message Queue
    participant WP as Worker Process
    participant PS as Push Service
    participant DB as Database

    NS->>MQ: Queue batch notifications
    MQ->>WP: Process notifications
    WP->>PS: Send push notifications
    PS-->>WP: Delivery status
    WP->>DB: Update delivery logs
    WP->>NS: Batch completion status
```

## 🔧 Notification Channels

### Push Notifications
```yaml
Firebase FCM:
  - Android notifications
  - Web push notifications
  - Topic messaging
  - Data messages
  - Notification scheduling

Apple APNs:
  - iOS notifications
  - Silent notifications
  - Rich notifications
  - Badge updates
  - Sound customization
```

### Email Notifications
```yaml
SendGrid:
  - Transactional emails
  - Marketing emails
  - Template management
  - Delivery tracking
  - Bounce handling

AWS SES:
  - High-volume emails
  - Reputation management
  - Bounce and complaint handling
  - Suppression lists
  - Delivery analytics
```

### SMS Notifications
```yaml
Twilio:
  - SMS messaging
  - Voice calls
  - WhatsApp messaging
  - International support
  - Delivery receipts

AWS SNS:
  - SMS messaging
  - Push notifications
  - Email notifications
  - Topic subscriptions
  - Message filtering
```

## 📈 Monitoring & Analytics

### Delivery Metrics
```yaml
Delivery Rates:
  - Push delivery rate
  - Email delivery rate
  - SMS delivery rate
  - In-game delivery rate
  - Overall delivery rate

Performance Metrics:
  - Average delivery time
  - Queue processing time
  - Error rates by channel
  - Retry success rates
  - System availability

User Engagement:
  - Open rates
  - Click-through rates
  - Unsubscribe rates
  - Preference changes
  - Channel preferences
```

### Real-time Dashboards
```yaml
Live Monitoring:
  - Notifications per minute
  - Delivery success rate
  - Error rate by channel
  - Queue depth
  - Active connections

Historical Analysis:
  - Delivery trends
  - User engagement
  - Channel performance
  - Error patterns
  - Cost analysis
```

## 🧪 Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_notification_sending() {
        // Test notification sending flow
    }
    
    #[tokio::test]
    async fn test_preference_management() {
        // Test preference management
    }
    
    #[tokio::test]
    async fn test_template_rendering() {
        // Test template rendering
    }
    
    #[tokio::test]
    async fn test_rate_limiting() {
        // Test rate limiting
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_complete_notification_flow() {
    // Test complete notification flow
    let service = NotificationService::new();
    
    // Send notification
    let notification = NotificationRequest {
        user_id: "test_user",
        type: "friend_request",
        channel: "push",
        title: "New Friend Request",
        message: "You have a new friend request",
        data: Some(json!({"from_user": "friend_123"})),
        priority: "normal",
    };
    
    let result = service.send_notification(notification).await;
    assert!(result.is_ok());
    
    // Verify delivery
    let status = service.get_notification_status(&result.unwrap().notification_id).await;
    assert_eq!(status, "delivered");
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
  let response = http.post('http://localhost:8085/notifications/send', {
    user_id: 'test_user',
    type: 'test',
    channel: 'push',
    title: 'Test Notification',
    message: 'This is a test notification'
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
COPY --from=builder /app/target/release/notification-service /usr/local/bin/
EXPOSE 8085
CMD ["notification-service"]
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: notification-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: notification-service
  template:
    metadata:
      labels:
        app: notification-service
    spec:
      containers:
      - name: notification-service
        image: notification-service:latest
        ports:
        - containerPort: 8085
        env:
        - name: DATABASE_URL
          value: "postgresql://user:pass@postgres:5432/notification_db"
        - name: REDIS_URL
          value: "redis://redis:6379"
        - name: KAFKA_BROKERS
          value: "kafka:9092"
        - name: FIREBASE_SERVER_KEY
          valueFrom:
            secretKeyRef:
              name: firebase-secret
              key: server-key
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
```

## 🔧 Configuration

### Environment Variables
```yaml
# Database Configuration
DATABASE_URL=postgresql://user:pass@localhost:5432/notification_db
DATABASE_POOL_SIZE=20
DATABASE_TIMEOUT=30s

# Redis Configuration
REDIS_URL=redis://localhost:6379
REDIS_PASSWORD=secret
REDIS_DB=3

# Kafka Configuration
KAFKA_BROKERS=localhost:9092
KAFKA_TOPIC_PREFIX=notifications
KAFKA_GROUP_ID=notification_service

# Push Services
FIREBASE_SERVER_KEY=your-firebase-server-key
APNS_KEY_ID=your-apns-key-id
APNS_TEAM_ID=your-apns-team-id
APNS_PRIVATE_KEY_PATH=/path/to/apns-key.p8

# Email Services
SENDGRID_API_KEY=your-sendgrid-api-key
SES_ACCESS_KEY=your-ses-access-key
SES_SECRET_KEY=your-ses-secret-key
SES_REGION=us-east-1

# SMS Services
TWILIO_ACCOUNT_SID=your-twilio-account-sid
TWILIO_AUTH_TOKEN=your-twilio-auth-token
SNS_ACCESS_KEY=your-sns-access-key
SNS_SECRET_KEY=your-sns-secret-key
SNS_REGION=us-east-1

# Server Configuration
SERVER_PORT=8085
SERVER_HOST=0.0.0.0
SERVER_WORKERS=4

# Rate Limiting
RATE_LIMIT_ENABLED=true
RATE_LIMIT_REDIS_URL=redis://localhost:6379
RATE_LIMIT_WINDOW=3600
RATE_LIMIT_MAX_REQUESTS=1000
```

### Configuration File
```yaml
# notification-config.yaml
server:
  port: 8085
  host: "0.0.0.0"
  workers: 4
  max_connections: 10000

database:
  url: "postgresql://user:pass@localhost:5432/notification_db"
  pool_size: 20
  timeout: "30s"
  ssl_mode: "prefer"

redis:
  url: "redis://localhost:6379"
  password: "secret"
  db: 3
  pool_size: 100

kafka:
  brokers: ["localhost:9092"]
  topic_prefix: "notifications"
  group_id: "notification_service"
  auto_offset_reset: "latest"

push_services:
  firebase:
    server_key: "your-firebase-server-key"
    enabled: true
  
  apns:
    key_id: "your-apns-key-id"
    team_id: "your-apns-team-id"
    private_key_path: "/path/to/apns-key.p8"
    enabled: true

email_services:
  sendgrid:
    api_key: "your-sendgrid-api-key"
    enabled: true
  
  ses:
    access_key: "your-ses-access-key"
    secret_key: "your-ses-secret-key"
    region: "us-east-1"
    enabled: true

sms_services:
  twilio:
    account_sid: "your-twilio-account-sid"
    auth_token: "your-twilio-auth-token"
    enabled: true
  
  sns:
    access_key: "your-sns-access-key"
    secret_key: "your-sns-secret-key"
    region: "us-east-1"
    enabled: true

rate_limiting:
  enabled: true
  window: 3600
  max_requests: 1000
  per_user_limit: 100
  per_channel_limit: 50

notification_settings:
  max_retry_attempts: 3
  retry_delay: 300
  batch_size: 1000
  processing_interval: 1000
  template_cache_ttl: 3600
```

## 🔗 Related Services

- [API Gateway](./api-gateway/README.md) - Entry point for all requests
- [User Management](./user-management/README.md) - User authentication
- [Chaos Backend](./chaos-backend/README.md) - Game Logic Core
- [Microservices Architecture](./microservices-architecture/README.md) - Overall Architecture
