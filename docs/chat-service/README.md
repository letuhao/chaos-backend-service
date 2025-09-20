# Chat Service - Chaos World

## 📋 Overview

Chat Service chịu trách nhiệm quản lý tất cả giao tiếp real-time trong game, bao gồm text chat, voice chat, và moderation system.

## 🎯 Responsibilities

### Core Functions
- **Real-time Messaging**: Text chat với WebSocket
- **Channel Management**: Public, private, guild channels
- **Voice Chat**: Voice communication
- **Moderation**: Content filtering và spam prevention
- **Message History**: Lưu trữ và tìm kiếm tin nhắn
- **Emoji & Reactions**: Emoji support và message reactions

### Performance Requirements
- **Latency**: < 10ms cho message delivery
- **Throughput**: 100,000+ messages/second
- **Availability**: 99.9% uptime
- **Concurrency**: Handle 1,000,000+ concurrent connections

## 🏗️ Architecture

### Technology Stack
```yaml
Language: Rust/Go
Framework: Axum/Gin
WebSocket: Socket.IO, WebSocket
Database: PostgreSQL (messages), Redis (real-time)
Message Queue: Apache Kafka
Voice: WebRTC, Janus Gateway
Search: Elasticsearch (message search)
```

### Core Components
```rust
pub struct ChatService {
    // WebSocket Management
    websocket_manager: Arc<WebSocketManager>,
    connection_pool: Arc<ConnectionPool>,
    
    // Message Processing
    message_processor: Arc<MessageProcessor>,
    moderation_engine: Arc<ModerationEngine>,
    spam_filter: Arc<SpamFilter>,
    
    // Channel Management
    channel_manager: Arc<ChannelManager>,
    room_manager: Arc<RoomManager>,
    
    // Voice Chat
    voice_manager: Arc<VoiceManager>,
    webrtc_handler: Arc<WebRTCHandler>,
    
    // Database
    message_repository: Arc<MessageRepository>,
    channel_repository: Arc<ChannelRepository>,
    user_repository: Arc<UserRepository>,
    
    // External Services
    user_management_client: Arc<UserManagementClient>,
    anti_cheat_client: Arc<AntiCheatClient>,
    
    // Configuration
    config: ChatConfig,
}
```

## 💬 Chat Types

### Text Chat
```yaml
Public Channels:
  - General chat
  - Trade chat
  - Looking for group (LFG)
  - Guild recruitment
  - World announcements

Private Channels:
  - Direct messages
  - Group conversations
  - Guild chat
  - Party chat
  - Whisper messages

System Channels:
  - System announcements
  - Error messages
  - Status updates
  - Maintenance notifications
```

### Voice Chat
```yaml
Voice Channels:
  - Guild voice rooms
  - Party voice chat
  - Private voice calls
  - Public voice channels
  - Conference calls

Voice Features:
  - Push-to-talk
  - Voice activation
  - Noise suppression
  - Echo cancellation
  - Volume control
```

### Message Types
```yaml
Text Messages:
  - Plain text
  - Formatted text (bold, italic, color)
  - Links and URLs
  - Mentions (@username)
  - Hashtags (#topic)

Rich Messages:
  - Emoji reactions
  - File attachments
  - Image sharing
  - Voice messages
  - Location sharing

System Messages:
  - User joins/leaves
  - Channel updates
  - Moderation actions
  - System notifications
```

## 🗄️ Database Schema

### PostgreSQL Tables
```sql
-- Channels table
CREATE TABLE channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    channel_id VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    channel_type VARCHAR(20) NOT NULL, -- 'public', 'private', 'guild', 'party', 'direct'
    owner_id UUID,
    guild_id UUID,
    is_active BOOLEAN DEFAULT TRUE,
    max_members INTEGER DEFAULT 1000,
    permissions JSONB,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Messages table
CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    message_id VARCHAR(50) UNIQUE NOT NULL,
    channel_id VARCHAR(50) NOT NULL,
    user_id UUID NOT NULL,
    content TEXT NOT NULL,
    message_type VARCHAR(20) DEFAULT 'text', -- 'text', 'voice', 'image', 'file', 'system'
    reply_to VARCHAR(50),
    metadata JSONB,
    is_edited BOOLEAN DEFAULT FALSE,
    edited_at TIMESTAMP,
    is_deleted BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Channel members table
CREATE TABLE channel_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    channel_id VARCHAR(50) NOT NULL,
    user_id UUID NOT NULL,
    role VARCHAR(20) DEFAULT 'member', -- 'owner', 'admin', 'moderator', 'member'
    permissions JSONB,
    joined_at TIMESTAMP DEFAULT NOW(),
    last_read_at TIMESTAMP,
    is_muted BOOLEAN DEFAULT FALSE,
    is_banned BOOLEAN DEFAULT FALSE,
    UNIQUE(channel_id, user_id)
);

-- Message reactions table
CREATE TABLE message_reactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    message_id VARCHAR(50) NOT NULL,
    user_id UUID NOT NULL,
    emoji VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(message_id, user_id, emoji)
);

-- Moderation actions table
CREATE TABLE moderation_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    action_id VARCHAR(50) UNIQUE NOT NULL,
    moderator_id UUID NOT NULL,
    target_user_id UUID NOT NULL,
    action_type VARCHAR(20) NOT NULL, -- 'warn', 'mute', 'kick', 'ban', 'timeout'
    reason TEXT,
    duration INTEGER, -- seconds, NULL for permanent
    channel_id VARCHAR(50),
    message_id VARCHAR(50),
    created_at TIMESTAMP DEFAULT NOW(),
    expires_at TIMESTAMP
);

-- User preferences table
CREATE TABLE chat_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID UNIQUE NOT NULL,
    language VARCHAR(10) DEFAULT 'en',
    timezone VARCHAR(50) DEFAULT 'UTC',
    show_timestamps BOOLEAN DEFAULT TRUE,
    show_join_leave_messages BOOLEAN DEFAULT TRUE,
    auto_translate BOOLEAN DEFAULT FALSE,
    voice_enabled BOOLEAN DEFAULT TRUE,
    push_notifications BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

### Redis Cache Structure
```yaml
Active Connections:
  key: "connection:{user_id}"
  value: { socket_id, channel_id, last_seen, status }
  ttl: 3600 seconds

Channel Members:
  key: "channel_members:{channel_id}"
  value: { user_ids: [...], count: 150 }
  ttl: 300 seconds

Message Cache:
  key: "messages:{channel_id}:recent"
  value: { messages: [...], last_message_id }
  ttl: 3600 seconds

User Status:
  key: "user_status:{user_id}"
  value: { online, channel_id, last_seen, status }
  ttl: 300 seconds
```

## 🔌 API Endpoints

### WebSocket Endpoints
```yaml
WebSocket /chat/connect:
  Description: Connect to chat service
  Events:
    - join_channel: Join a channel
    - leave_channel: Leave a channel
    - send_message: Send a message
    - edit_message: Edit a message
    - delete_message: Delete a message
    - add_reaction: Add emoji reaction
    - remove_reaction: Remove emoji reaction
    - typing_start: Start typing indicator
    - typing_stop: Stop typing indicator
    - voice_join: Join voice channel
    - voice_leave: Leave voice channel
```

### REST API Endpoints
```yaml
GET /channels:
  Description: Get available channels
  Request: { user_id, channel_type }
  Response: { 
    channels: [...], 
    total 
  }
  Rate Limit: 1000/hour per user

POST /channels:
  Description: Create new channel
  Request: { 
    name, 
    description, 
    channel_type, 
    guild_id 
  }
  Response: { 
    channel_id, 
    status: 'created' 
  }
  Rate Limit: 10/hour per user

GET /channels/{channel_id}/messages:
  Description: Get channel messages
  Request: { 
    channel_id, 
    page, 
    limit, 
    before_message_id 
  }
  Response: { 
    messages: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user

POST /channels/{channel_id}/messages:
  Description: Send message to channel
  Request: { 
    channel_id, 
    content, 
    message_type, 
    reply_to 
  }
  Response: { 
    message_id, 
    status: 'sent' 
  }
  Rate Limit: 100/minute per user
```

### Moderation Endpoints
```yaml
POST /moderation/warn:
  Description: Warn a user
  Request: { 
    target_user_id, 
    reason, 
    channel_id 
  }
  Response: { 
    action_id, 
    status: 'warned' 
  }
  Rate Limit: 100/hour per moderator

POST /moderation/mute:
  Description: Mute a user
  Request: { 
    target_user_id, 
    duration, 
    reason, 
    channel_id 
  }
  Response: { 
    action_id, 
    status: 'muted' 
  }
  Rate Limit: 50/hour per moderator

POST /moderation/ban:
  Description: Ban a user
  Request: { 
    target_user_id, 
    duration, 
    reason, 
    channel_id 
  }
  Response: { 
    action_id, 
    status: 'banned' 
  }
  Rate Limit: 20/hour per moderator

GET /moderation/actions:
  Description: Get moderation history
  Request: { 
    user_id, 
    action_type, 
    page, 
    limit 
  }
  Response: { 
    actions: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per moderator
```

## 🔍 Moderation System

### Content Filtering
```yaml
Spam Detection:
  - Message frequency limits
  - Duplicate message detection
  - Character repetition
  - Link spam detection
  - Emoji spam detection

Profanity Filter:
  - Word blacklist
  - Context-aware filtering
  - Leet speak detection
  - Bypass attempt detection
  - Custom word lists

Inappropriate Content:
  - Image content analysis
  - Link safety checking
  - Phishing detection
  - Malware link detection
  - NSFW content detection
```

### Automated Moderation
```yaml
Auto-Actions:
  - Auto-delete spam messages
  - Auto-mute repeat offenders
  - Auto-ban severe violations
  - Auto-warn minor violations
  - Auto-flag suspicious content

Machine Learning:
  - Toxicity detection
  - Sentiment analysis
  - Context understanding
  - Pattern recognition
  - Behavior analysis
```

### Manual Moderation
```yaml
Moderator Tools:
  - Message review queue
  - User history view
  - Bulk actions
  - Appeal system
  - Audit logging

Moderator Permissions:
  - Channel moderation
  - User management
  - Content review
  - Ban management
  - Report handling
```

## 📊 Monitoring & Analytics

### Chat Metrics
```yaml
Usage Metrics:
  - Messages per second
  - Active users
  - Channel activity
  - Voice usage
  - Moderation actions

Performance Metrics:
  - Message delivery time
  - WebSocket latency
  - Connection stability
  - Error rates
  - System availability

Quality Metrics:
  - Spam detection rate
  - False positive rate
  - User satisfaction
  - Moderation effectiveness
  - Content quality
```

### Real-time Dashboards
```yaml
Live Monitoring:
  - Active connections
  - Message throughput
  - Channel activity
  - Moderation queue
  - System health

Historical Analysis:
  - Usage trends
  - Peak hours
  - Popular channels
  - Moderation patterns
  - Performance trends
```

## 🧪 Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_message_sending() {
        // Test message sending flow
    }
    
    #[tokio::test]
    async fn test_channel_management() {
        // Test channel creation, joining, leaving
    }
    
    #[tokio::test]
    async fn test_moderation_system() {
        // Test moderation actions
    }
    
    #[tokio::test]
    async fn test_voice_chat() {
        // Test voice chat functionality
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_complete_chat_flow() {
    // Test complete chat flow
    let service = ChatService::new();
    
    // Create channel
    let channel = service.create_channel(ChannelRequest {
        name: "test_channel",
        channel_type: "public",
        owner_id: "user_1",
    }).await.unwrap();
    
    // Join channel
    service.join_channel(&channel.channel_id, "user_1").await.unwrap();
    service.join_channel(&channel.channel_id, "user_2").await.unwrap();
    
    // Send message
    let message = service.send_message(MessageRequest {
        channel_id: &channel.channel_id,
        user_id: "user_1",
        content: "Hello, world!",
        message_type: "text",
    }).await.unwrap();
    
    // Verify message received
    let messages = service.get_channel_messages(&channel.channel_id, 0, 10).await.unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].content, "Hello, world!");
}
```

### Load Tests
```yaml
# k6 load test
import ws from 'k6/ws';
import { check } from 'k6';

export let options = {
  stages: [
    { duration: '2m', target: 1000 },
    { duration: '5m', target: 1000 },
    { duration: '2m', target: 2000 },
    { duration: '5m', target: 2000 },
    { duration: '2m', target: 0 },
  ],
};

export default function() {
  let url = 'ws://localhost:8087/chat/connect';
  let params = { tags: { my_tag: 'hello' } };
  
  let res = ws.connect(url, params, function (socket) {
    socket.on('open', () => {
      socket.send(JSON.stringify({
        type: 'join_channel',
        channel_id: 'test_channel'
      }));
    });
    
    socket.on('message', (data) => {
      let message = JSON.parse(data);
      check(message, {
        'message received': (m) => m.type === 'message',
      });
    });
    
    socket.setTimeout(() => {
      socket.close();
    }, 10000);
  });
  
  check(res, { 'status is 101': (r) => r && r.status === 101 });
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
COPY --from=builder /app/target/release/chat-service /usr/local/bin/
EXPOSE 8087
CMD ["chat-service"]
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: chat-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: chat-service
  template:
    metadata:
      labels:
        app: chat-service
    spec:
      containers:
      - name: chat-service
        image: chat-service:latest
        ports:
        - containerPort: 8087
        env:
        - name: DATABASE_URL
          value: "postgresql://user:pass@postgres:5432/chat_db"
        - name: REDIS_URL
          value: "redis://redis:6379"
        - name: KAFKA_BROKERS
          value: "kafka:9092"
        resources:
          requests:
            memory: "1Gi"
            cpu: "1000m"
          limits:
            memory: "2Gi"
            cpu: "2000m"
```

## 🔧 Configuration

### Environment Variables
```yaml
# Database Configuration
DATABASE_URL=postgresql://user:pass@localhost:5432/chat_db
DATABASE_POOL_SIZE=20
DATABASE_TIMEOUT=30s

# Redis Configuration
REDIS_URL=redis://localhost:6379
REDIS_PASSWORD=secret
REDIS_DB=5

# Kafka Configuration
KAFKA_BROKERS=localhost:9092
KAFKA_TOPIC_PREFIX=chat
KAFKA_GROUP_ID=chat_service

# WebSocket Configuration
WEBSOCKET_PORT=8087
WEBSOCKET_HOST=0.0.0.0
MAX_CONNECTIONS=1000000
MESSAGE_BUFFER_SIZE=1000

# Voice Chat Configuration
WEBRTC_STUN_SERVERS=stun:stun.l.google.com:19302
WEBRTC_TURN_SERVERS=turn:turn.example.com:3478
JANUS_GATEWAY_URL=http://janus:8088

# External Services
USER_MANAGEMENT_URL=grpc://user-management:9091
ANTI_CHEAT_URL=grpc://anti-cheat:9093

# Moderation Configuration
MODERATION_ENABLED=true
AUTO_MODERATION_ENABLED=true
SPAM_DETECTION_ENABLED=true
PROFANITY_FILTER_ENABLED=true
```

### Configuration File
```yaml
# chat-config.yaml
server:
  port: 8087
  host: "0.0.0.0"
  workers: 4
  max_connections: 1000000

database:
  url: "postgresql://user:pass@localhost:5432/chat_db"
  pool_size: 20
  timeout: "30s"
  ssl_mode: "prefer"

redis:
  url: "redis://localhost:6379"
  password: "secret"
  db: 5
  pool_size: 100

kafka:
  brokers: ["localhost:9092"]
  topic_prefix: "chat"
  group_id: "chat_service"
  auto_offset_reset: "latest"

websocket:
  port: 8087
  host: "0.0.0.0"
  max_connections: 1000000
  message_buffer_size: 1000
  ping_interval: 30
  pong_timeout: 60

voice_chat:
  webrtc:
    stun_servers: ["stun:stun.l.google.com:19302"]
    turn_servers: ["turn:turn.example.com:3478"]
  janus_gateway:
    url: "http://janus:8088"
    enabled: true

external_services:
  user_management:
    url: "grpc://user-management:9091"
    timeout: "3s"
    retries: 3
  
  anti_cheat:
    url: "grpc://anti-cheat:9093"
    timeout: "5s"
    retries: 3

moderation:
  enabled: true
  auto_moderation: true
  spam_detection: true
  profanity_filter: true
  content_filter: true
  ml_moderation: true
  
  limits:
    messages_per_minute: 30
    messages_per_hour: 1000
    max_message_length: 2000
    max_emoji_count: 10
  
  actions:
    auto_delete_spam: true
    auto_mute_repeat_offenders: true
    auto_ban_severe_violations: true
    auto_warn_minor_violations: true

channels:
  max_channels_per_user: 50
  max_members_per_channel: 1000
  max_private_channels: 20
  max_guild_channels: 100
  channel_name_max_length: 50
  channel_description_max_length: 500
```

## 🔗 Related Services

- [User Management](./user-management/README.md) - User authentication
- [Anti-Cheat Service](./anti-cheat-service/README.md) - Content moderation
- [Guild Service](./guild-service/README.md) - Guild channels
- [Notification Service](./notification-service/README.md) - Chat notifications
- [Microservices Architecture](./microservices-architecture/README.md) - Overall Architecture
