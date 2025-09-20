# Event Service - Chaos World

## 📋 Overview

Event Service chịu trách nhiệm quản lý tất cả các sự kiện trong game, bao gồm event scheduling, event rewards, event participation, và event analytics.

## 🎯 Responsibilities

### Core Functions
- **Event Management**: Tạo, cập nhật, xóa events
- **Event Scheduling**: Lên lịch và quản lý thời gian events
- **Event Rewards**: Quản lý phần thưởng cho events
- **Event Participation**: Theo dõi sự tham gia của người chơi
- **Event Analytics**: Phân tích hiệu suất events
- **Event Notifications**: Thông báo về events

### Performance Requirements
- **Latency**: < 50ms cho event operations
- **Throughput**: 20,000+ operations/second
- **Availability**: 99.9% uptime
- **Concurrency**: Handle 1,000,000+ concurrent event interactions

## 🏗️ Architecture

### Technology Stack
```yaml
Language: Rust
Framework: Axum
Database: PostgreSQL (event data), Redis (real-time)
Message Queue: Apache Kafka
Scheduler: Cron, Quartz
Cache: Redis Cluster
```

### Core Components
```rust
pub struct EventService {
    // Database
    event_repository: Arc<EventRepository>,
    participation_repository: Arc<ParticipationRepository>,
    reward_repository: Arc<RewardRepository>,
    schedule_repository: Arc<ScheduleRepository>,
    
    // Event Management
    event_manager: Arc<EventManager>,
    schedule_manager: Arc<ScheduleManager>,
    reward_manager: Arc<RewardManager>,
    participation_manager: Arc<ParticipationManager>,
    
    // Event Processing
    event_processor: Arc<EventProcessor>,
    event_scheduler: Arc<EventScheduler>,
    event_notifier: Arc<EventNotifier>,
    
    // Analytics
    event_analytics: Arc<EventAnalytics>,
    participation_analytics: Arc<ParticipationAnalytics>,
    reward_analytics: Arc<RewardAnalytics>,
    
    // External Services
    chaos_backend_client: Arc<ChaosBackendClient>,
    notification_service_client: Arc<NotificationServiceClient>,
    analytics_service_client: Arc<AnalyticsServiceClient>,
    
    // Configuration
    config: EventConfig,
}
```

## 🎉 Event Types

### Scheduled Events
```yaml
Daily Events:
  - Login rewards
  - Daily quests
  - Daily dungeons
  - Daily PvP matches
  - Daily crafting

Weekly Events:
  - Weekly raids
  - Weekly tournaments
  - Weekly guild wars
  - Weekly challenges
  - Weekly rewards

Monthly Events:
  - Monthly rankings
  - Monthly competitions
  - Monthly rewards
  - Monthly special dungeons
  - Monthly guild events
```

### Special Events
```yaml
Holiday Events:
  - New Year celebration
  - Valentine's Day
  - Halloween
  - Christmas
  - Game anniversary

Seasonal Events:
  - Spring festival
  - Summer adventure
  - Autumn harvest
  - Winter wonderland
  - Seasonal rewards

Limited Events:
  - Beta testing rewards
  - Pre-registration bonuses
  - Launch celebrations
  - Update celebrations
  - Community events
```

### Dynamic Events
```yaml
World Events:
  - World boss spawns
  - Server-wide challenges
  - Global competitions
  - Cross-server events
  - Community goals

Random Events:
  - Lucky hour bonuses
  - Double experience
  - Double gold drops
  - Special monster spawns
  - Treasure hunts

Emergency Events:
  - Server maintenance rewards
  - Bug compensation
  - Apology gifts
  - Emergency bonuses
  - Crisis events
```

## 🗄️ Database Schema

### PostgreSQL Tables
```sql
-- Events table
CREATE TABLE events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    event_type VARCHAR(50) NOT NULL, -- 'scheduled', 'special', 'dynamic', 'emergency'
    category VARCHAR(50) NOT NULL, -- 'daily', 'weekly', 'monthly', 'holiday', 'seasonal'
    status VARCHAR(20) DEFAULT 'draft', -- 'draft', 'scheduled', 'active', 'completed', 'cancelled'
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP,
    duration INTEGER, -- seconds
    max_participants INTEGER,
    current_participants INTEGER DEFAULT 0,
    requirements JSONB,
    rewards JSONB,
    settings JSONB,
    created_by UUID NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Event schedules table
CREATE TABLE event_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    schedule_id VARCHAR(50) UNIQUE NOT NULL,
    event_id VARCHAR(50) NOT NULL,
    schedule_type VARCHAR(20) NOT NULL, -- 'once', 'daily', 'weekly', 'monthly', 'cron'
    schedule_config JSONB NOT NULL, -- cron expression, interval, etc.
    timezone VARCHAR(50) DEFAULT 'UTC',
    is_active BOOLEAN DEFAULT TRUE,
    next_run TIMESTAMP,
    last_run TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Event participations table
CREATE TABLE event_participations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    participation_id VARCHAR(50) UNIQUE NOT NULL,
    event_id VARCHAR(50) NOT NULL,
    user_id UUID NOT NULL,
    participation_type VARCHAR(20) NOT NULL, -- 'automatic', 'manual', 'invited'
    status VARCHAR(20) DEFAULT 'active', -- 'active', 'completed', 'cancelled', 'expired'
    progress JSONB,
    rewards_claimed JSONB,
    participation_time TIMESTAMP DEFAULT NOW(),
    completion_time TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Event rewards table
CREATE TABLE event_rewards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reward_id VARCHAR(50) UNIQUE NOT NULL,
    event_id VARCHAR(50) NOT NULL,
    reward_type VARCHAR(50) NOT NULL, -- 'experience', 'gold', 'item', 'currency', 'title'
    reward_value JSONB NOT NULL,
    reward_condition JSONB,
    max_claims INTEGER,
    current_claims INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Event analytics table
CREATE TABLE event_analytics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id VARCHAR(50) NOT NULL,
    metric_name VARCHAR(100) NOT NULL,
    metric_value DECIMAL(15,2) NOT NULL,
    metric_type VARCHAR(20) NOT NULL, -- 'counter', 'gauge', 'histogram', 'summary'
    tags JSONB,
    timestamp TIMESTAMP DEFAULT NOW()
);

-- Event notifications table
CREATE TABLE event_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    notification_id VARCHAR(50) UNIQUE NOT NULL,
    event_id VARCHAR(50) NOT NULL,
    user_id UUID,
    notification_type VARCHAR(20) NOT NULL, -- 'event_start', 'event_end', 'reward_available', 'reminder'
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    status VARCHAR(20) DEFAULT 'pending', -- 'pending', 'sent', 'delivered', 'failed'
    scheduled_at TIMESTAMP,
    sent_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);
```

### Redis Cache Structure
```yaml
Active Events:
  key: "active_events"
  value: { events: [...], count: 25 }
  ttl: 60 seconds

Event Participants:
  key: "event_participants:{event_id}"
  value: { participants: [...], count: 1500 }
  ttl: 300 seconds

Event Rewards:
  key: "event_rewards:{event_id}"
  value: { rewards: [...], total_value: 10000 }
  ttl: 3600 seconds

Event Schedules:
  key: "event_schedules"
  value: { schedules: [...], next_run: "2023-01-01T00:00:00Z" }
  ttl: 60 seconds
```

## 🔌 API Endpoints

### Event Management Endpoints
```yaml
GET /events:
  Description: Get events
  Request: { 
    event_type, 
    category, 
    status, 
    page, 
    limit 
  }
  Response: { 
    events: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user

GET /events/{event_id}:
  Description: Get specific event
  Request: { event_id }
  Response: { 
    event_info, 
    participants, 
    rewards, 
    analytics 
  }
  Rate Limit: 1000/hour per user

POST /events:
  Description: Create event (admin only)
  Request: { 
    event_id, 
    name, 
    description, 
    event_type, 
    start_time, 
    end_time, 
    rewards 
  }
  Response: { 
    event_id, 
    status: 'created' 
  }
  Rate Limit: 100/hour per admin

PUT /events/{event_id}:
  Description: Update event (admin only)
  Request: { 
    event_id, 
    updates 
  }
  Response: { 
    success: true, 
    updated_event 
  }
  Rate Limit: 100/hour per admin
```

### Participation Endpoints
```yaml
POST /events/{event_id}/join:
  Description: Join event
  Request: { 
    event_id, 
    user_id 
  }
  Response: { 
    participation_id, 
    status: 'joined' 
  }
  Rate Limit: 10/minute per user

POST /events/{event_id}/leave:
  Description: Leave event
  Request: { 
    event_id, 
    user_id 
  }
  Response: { 
    success: true 
  }
  Rate Limit: 10/minute per user

GET /events/{event_id}/participants:
  Description: Get event participants
  Request: { 
    event_id, 
    page, 
    limit 
  }
  Response: { 
    participants: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user

GET /users/{user_id}/events:
  Description: Get user's events
  Request: { 
    user_id, 
    status, 
    page, 
    limit 
  }
  Response: { 
    events: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user
```

### Reward Endpoints
```yaml
GET /events/{event_id}/rewards:
  Description: Get event rewards
  Request: { event_id }
  Response: { 
    rewards: [...], 
    total_value 
  }
  Rate Limit: 1000/hour per user

POST /events/{event_id}/rewards/claim:
  Description: Claim event reward
  Request: { 
    event_id, 
    user_id, 
    reward_id 
  }
  Response: { 
    success: true, 
    claimed_reward 
  }
  Rate Limit: 10/minute per user

GET /users/{user_id}/rewards:
  Description: Get user's rewards
  Request: { 
    user_id, 
    event_id, 
    status, 
    page, 
    limit 
  }
  Response: { 
    rewards: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user
```

### Schedule Endpoints
```yaml
GET /schedules:
  Description: Get event schedules
  Request: { 
    schedule_type, 
    is_active, 
    page, 
    limit 
  }
  Response: { 
    schedules: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user

POST /schedules:
  Description: Create event schedule (admin only)
  Request: { 
    event_id, 
    schedule_type, 
    schedule_config, 
    timezone 
  }
  Response: { 
    schedule_id, 
    status: 'created' 
  }
  Rate Limit: 100/hour per admin

PUT /schedules/{schedule_id}:
  Description: Update event schedule (admin only)
  Request: { 
    schedule_id, 
    updates 
  }
  Response: { 
    success: true, 
    updated_schedule 
  }
  Rate Limit: 100/hour per admin
```

## ⏰ Event Scheduling

### Schedule Types
```yaml
Once:
  - Single occurrence
  - Specific date/time
  - No repetition
  - Manual trigger

Daily:
  - Every day
  - Specific time
  - Timezone aware
  - Automatic trigger

Weekly:
  - Specific day of week
  - Specific time
  - Timezone aware
  - Automatic trigger

Monthly:
  - Specific day of month
  - Specific time
  - Timezone aware
  - Automatic trigger

Cron:
  - Custom schedule
  - Cron expression
  - Flexible timing
  - Automatic trigger
```

### Schedule Examples
```yaml
Daily Login Reward:
  - Schedule: "0 0 * * *" (midnight)
  - Timezone: "UTC"
  - Event: "daily_login_reward"
  - Auto-start: true

Weekly Raid:
  - Schedule: "0 20 * * 5" (Friday 8 PM)
  - Timezone: "UTC"
  - Event: "weekly_raid"
  - Auto-start: true

Monthly Tournament:
  - Schedule: "0 18 1 * *" (1st of month 6 PM)
  - Timezone: "UTC"
  - Event: "monthly_tournament"
  - Auto-start: true

Holiday Event:
  - Schedule: "0 0 25 12 *" (Christmas Day)
  - Timezone: "UTC"
  - Event: "christmas_event"
  - Auto-start: true
```

## 🎁 Reward System

### Reward Types
```yaml
Experience Rewards:
  - Base experience
  - Bonus experience
  - Experience multipliers
  - Level-based scaling

Currency Rewards:
  - Gold
  - Gems
  - Tokens
  - Special currency

Item Rewards:
  - Equipment
  - Consumables
  - Materials
  - Special items

Title Rewards:
  - Achievement titles
  - Event titles
  - Seasonal titles
  - Exclusive titles
```

### Reward Conditions
```yaml
Participation Rewards:
  - Join event
  - Complete event
  - Reach milestone
  - Win competition

Performance Rewards:
  - Top performer
  - Best score
  - Fastest completion
  - Most creative

Loyalty Rewards:
  - Daily participation
  - Weekly participation
  - Monthly participation
  - Yearly participation
```

## 📊 Monitoring & Analytics

### Event Metrics
```yaml
Participation Metrics:
  - Total participants
  - Active participants
  - Completion rate
  - Dropout rate
  - Engagement time

Reward Metrics:
  - Rewards distributed
  - Reward value
  - Claim rate
  - Satisfaction score
  - Redemption rate

Performance Metrics:
  - Event success rate
  - System latency
  - Error rate
  - Availability
  - Throughput
```

### Real-time Dashboards
```yaml
Live Monitoring:
  - Active events
  - Current participants
  - Reward claims
  - System health
  - Error rates

Historical Analysis:
  - Event trends
  - Participation patterns
  - Reward effectiveness
  - User behavior
  - Performance trends
```

## 🧪 Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_event_creation() {
        // Test event creation and validation
    }
    
    #[tokio::test]
    async fn test_event_scheduling() {
        // Test event scheduling system
    }
    
    #[tokio::test]
    async fn test_participation_management() {
        // Test participation join/leave
    }
    
    #[tokio::test]
    async fn test_reward_system() {
        // Test reward distribution and claiming
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_complete_event_flow() {
    // Test complete event flow
    let service = EventService::new();
    
    // Create event
    let event = service.create_event(EventRequest {
        event_id: "test_event",
        name: "Test Event",
        event_type: "scheduled",
        start_time: "2023-01-01T00:00:00Z",
        end_time: "2023-01-01T23:59:59Z",
        rewards: vec![RewardRequest {
            reward_type: "experience",
            reward_value: json!({"amount": 1000}),
        }],
    }).await.unwrap();
    
    // Create schedule
    service.create_schedule(ScheduleRequest {
        event_id: &event.event_id,
        schedule_type: "once",
        schedule_config: json!({"start_time": "2023-01-01T00:00:00Z"}),
    }).await.unwrap();
    
    // Join event
    service.join_event(&event.event_id, "user_1").await.unwrap();
    service.join_event(&event.event_id, "user_2").await.unwrap();
    
    // Complete event
    service.complete_event(&event.event_id, "user_1").await.unwrap();
    
    // Claim reward
    let reward = service.claim_reward(&event.event_id, "user_1", "reward_1").await.unwrap();
    
    // Verify event state
    let event_info = service.get_event(&event.event_id).await.unwrap();
    assert_eq!(event_info.current_participants, 2);
    assert_eq!(event_info.status, "completed");
    
    // Verify reward
    assert_eq!(reward.reward_type, "experience");
    assert_eq!(reward.reward_value["amount"], 1000);
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
  let response = http.get('http://localhost:8092/events?status=active');
  check(response, {
    'status is 200': (r) => r.status === 200,
    'response time < 50ms': (r) => r.timings.duration < 50,
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
COPY --from=builder /app/target/release/event-service /usr/local/bin/
EXPOSE 8092
CMD ["event-service"]
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: event-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: event-service
  template:
    metadata:
      labels:
        app: event-service
    spec:
      containers:
      - name: event-service
        image: event-service:latest
        ports:
        - containerPort: 8092
        env:
        - name: DATABASE_URL
          value: "postgresql://user:pass@postgres:5432/event_db"
        - name: REDIS_URL
          value: "redis://redis:6379"
        - name: KAFKA_BROKERS
          value: "kafka:9092"
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
DATABASE_URL=postgresql://user:pass@localhost:5432/event_db
DATABASE_POOL_SIZE=20
DATABASE_TIMEOUT=30s

# Redis Configuration
REDIS_URL=redis://localhost:6379
REDIS_PASSWORD=secret
REDIS_DB=10

# Kafka Configuration
KAFKA_BROKERS=localhost:9092
KAFKA_TOPIC_PREFIX=events
KAFKA_GROUP_ID=event_service

# External Services
CHAOS_BACKEND_URL=grpc://chaos-backend:9090
NOTIFICATION_SERVICE_URL=grpc://notification-service:9095
ANALYTICS_SERVICE_URL=grpc://analytics-service:9097

# Server Configuration
SERVER_PORT=8092
SERVER_HOST=0.0.0.0
SERVER_WORKERS=4

# Event Configuration
MAX_EVENTS=1000
MAX_PARTICIPANTS_PER_EVENT=10000
EVENT_CACHE_TTL=3600
SCHEDULE_CHECK_INTERVAL=60
```

### Configuration File
```yaml
# event-config.yaml
server:
  port: 8092
  host: "0.0.0.0"
  workers: 4
  max_connections: 10000

database:
  url: "postgresql://user:pass@localhost:5432/event_db"
  pool_size: 20
  timeout: "30s"
  ssl_mode: "prefer"

redis:
  url: "redis://localhost:6379"
  password: "secret"
  db: 10
  pool_size: 100

kafka:
  brokers: ["localhost:9092"]
  topic_prefix: "events"
  group_id: "event_service"
  auto_offset_reset: "latest"

external_services:
  chaos_backend:
    url: "grpc://chaos-backend:9090"
    timeout: "5s"
    retries: 3
  
  notification_service:
    url: "grpc://notification-service:9095"
    timeout: "5s"
    retries: 3
  
  analytics_service:
    url: "grpc://analytics-service:9097"
    timeout: "5s"
    retries: 3

event_settings:
  max_events: 1000
  max_participants_per_event: 10000
  event_cache_ttl: 3600
  schedule_check_interval: 60
  auto_start_events: true
  auto_end_events: true
  reward_claim_timeout: 86400

schedule_settings:
  max_schedules: 10000
  schedule_check_interval: 60
  timezone_default: "UTC"
  cron_timezone: "UTC"
  schedule_retry_attempts: 3
  schedule_retry_delay: 300

reward_settings:
  max_rewards_per_event: 100
  reward_claim_timeout: 86400
  reward_validation_enabled: true
  reward_duplicate_check: true
  reward_audit_logging: true

notification_settings:
  event_start_notification: true
  event_end_notification: true
  reward_available_notification: true
  reminder_notifications: true
  notification_channels: ["push", "email", "in_game"]
```

## 🔗 Related Services

- [Chaos Backend](./chaos-backend/README.md) - Game Logic Core
- [Notification Service](./notification-service/README.md) - Event notifications
- [Analytics Service](./analytics-service/README.md) - Event analytics
- [User Management](./user-management/README.md) - User authentication
- [Microservices Architecture](./microservices-architecture/README.md) - Overall Architecture
