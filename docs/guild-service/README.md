# Guild Service - Chaos World

## 📋 Overview

Guild Service chịu trách nhiệm quản lý tất cả các guild (hội) trong game, bao gồm guild creation, member management, guild activities, và guild wars.

## 🎯 Responsibilities

### Core Functions
- **Guild Management**: Tạo, cập nhật, xóa guild
- **Member Management**: Thêm, xóa, quản lý thành viên
- **Guild Activities**: Events, raids, guild quests
- **Guild Wars**: PvP giữa các guild
- **Guild Resources**: Quản lý tài nguyên guild
- **Guild Storage**: Kho chung của guild

### Performance Requirements
- **Latency**: < 100ms cho guild operations
- **Throughput**: 5,000+ operations/second
- **Availability**: 99.9% uptime
- **Concurrency**: Handle 100,000+ concurrent guild operations

## 🏗️ Architecture

### Technology Stack
```yaml
Language: Rust
Framework: Axum
Database: PostgreSQL (ACID compliance)
Caching: Redis
Message Queue: Apache Kafka
Search: Elasticsearch (guild search)
```

### Core Components
```rust
pub struct GuildService {
    // Database
    guild_repository: Arc<GuildRepository>,
    member_repository: Arc<MemberRepository>,
    activity_repository: Arc<ActivityRepository>,
    war_repository: Arc<WarRepository>,
    
    // Guild Management
    guild_manager: Arc<GuildManager>,
    member_manager: Arc<MemberManager>,
    permission_manager: Arc<PermissionManager>,
    
    // Guild Activities
    activity_manager: Arc<ActivityManager>,
    quest_manager: Arc<QuestManager>,
    raid_manager: Arc<RaidManager>,
    
    // Guild Wars
    war_manager: Arc<WarManager>,
    battle_manager: Arc<BattleManager>,
    
    // Guild Resources
    resource_manager: Arc<ResourceManager>,
    storage_manager: Arc<StorageManager>,
    
    // External Services
    chaos_backend_client: Arc<ChaosBackendClient>,
    inventory_service_client: Arc<InventoryServiceClient>,
    chat_service_client: Arc<ChatServiceClient>,
    
    // Configuration
    config: GuildConfig,
}
```

## 👥 Guild System

### Guild Structure
```yaml
Guild Hierarchy:
  - Guild Master (Owner)
  - Vice Masters (Deputy Leaders)
  - Officers (Senior Members)
  - Members (Regular Members)
  - Recruits (New Members)

Guild Levels:
  - Level 1-10: Basic guild
  - Level 11-20: Intermediate guild
  - Level 21-30: Advanced guild
  - Level 31-40: Elite guild
  - Level 41-50: Legendary guild
```

### Guild Features
```yaml
Basic Features:
  - Guild name and description
  - Guild emblem and banner
  - Guild level and experience
  - Member limit (based on level)
  - Guild treasury

Advanced Features:
  - Guild storage
  - Guild buildings
  - Guild skills
  - Guild buffs
  - Guild events
```

### Guild Activities
```yaml
Daily Activities:
  - Guild quests
  - Guild dungeons
  - Guild raids
  - Guild farming
  - Guild trading

Weekly Activities:
  - Guild wars
  - Guild tournaments
  - Guild events
  - Guild challenges
  - Guild rankings

Special Activities:
  - Seasonal events
  - World boss raids
  - Cross-server wars
  - Guild alliances
  - Guild competitions
```

## 🗄️ Database Schema

### PostgreSQL Tables
```sql
-- Guilds table
CREATE TABLE guilds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guild_id VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    emblem_url VARCHAR(500),
    banner_url VARCHAR(500),
    level INTEGER DEFAULT 1,
    experience BIGINT DEFAULT 0,
    max_members INTEGER DEFAULT 50,
    treasury BIGINT DEFAULT 0,
    status VARCHAR(20) DEFAULT 'active', -- 'active', 'inactive', 'disbanded'
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Guild members table
CREATE TABLE guild_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guild_id VARCHAR(50) NOT NULL,
    user_id UUID NOT NULL,
    role VARCHAR(20) NOT NULL, -- 'master', 'vice_master', 'officer', 'member', 'recruit'
    permissions JSONB,
    joined_at TIMESTAMP DEFAULT NOW(),
    last_active_at TIMESTAMP DEFAULT NOW(),
    contribution_points INTEGER DEFAULT 0,
    is_online BOOLEAN DEFAULT FALSE,
    UNIQUE(guild_id, user_id)
);

-- Guild activities table
CREATE TABLE guild_activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    activity_id VARCHAR(50) UNIQUE NOT NULL,
    guild_id VARCHAR(50) NOT NULL,
    activity_type VARCHAR(50) NOT NULL, -- 'quest', 'raid', 'war', 'event', 'dungeon'
    title VARCHAR(255) NOT NULL,
    description TEXT,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP,
    max_participants INTEGER,
    current_participants INTEGER DEFAULT 0,
    status VARCHAR(20) DEFAULT 'scheduled', -- 'scheduled', 'active', 'completed', 'cancelled'
    rewards JSONB,
    requirements JSONB,
    created_by UUID NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Guild wars table
CREATE TABLE guild_wars (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    war_id VARCHAR(50) UNIQUE NOT NULL,
    attacker_guild_id VARCHAR(50) NOT NULL,
    defender_guild_id VARCHAR(50) NOT NULL,
    war_type VARCHAR(20) NOT NULL, -- 'territory', 'honor', 'resource', 'alliance'
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP,
    status VARCHAR(20) DEFAULT 'pending', -- 'pending', 'active', 'completed', 'cancelled'
    winner_guild_id VARCHAR(50),
    war_score JSONB,
    rewards JSONB,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Guild resources table
CREATE TABLE guild_resources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guild_id VARCHAR(50) NOT NULL,
    resource_type VARCHAR(50) NOT NULL, -- 'gold', 'materials', 'items', 'points'
    resource_id VARCHAR(50),
    quantity BIGINT NOT NULL DEFAULT 0,
    max_quantity BIGINT,
    last_updated TIMESTAMP DEFAULT NOW(),
    UNIQUE(guild_id, resource_type, resource_id)
);

-- Guild storage table
CREATE TABLE guild_storage (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guild_id VARCHAR(50) NOT NULL,
    slot_position INTEGER NOT NULL,
    item_id VARCHAR(50) NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    durability INTEGER,
    enhancement_level INTEGER DEFAULT 0,
    properties JSONB,
    deposited_by UUID NOT NULL,
    deposited_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(guild_id, slot_position)
);

-- Guild permissions table
CREATE TABLE guild_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guild_id VARCHAR(50) NOT NULL,
    role VARCHAR(20) NOT NULL,
    permission VARCHAR(50) NOT NULL,
    is_granted BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(guild_id, role, permission)
);
```

### Redis Cache Structure
```yaml
Guild Info:
  key: "guild:{guild_id}"
  value: { name, level, members_count, treasury, status }
  ttl: 3600 seconds

Guild Members:
  key: "guild_members:{guild_id}"
  value: { members: [...], total: 150, online: 25 }
  ttl: 300 seconds

Guild Activities:
  key: "guild_activities:{guild_id}"
  value: { activities: [...], active_count: 5 }
  ttl: 600 seconds

Guild Wars:
  key: "guild_wars:{guild_id}"
  value: { wars: [...], active_wars: 2 }
  ttl: 300 seconds
```

## 🔌 API Endpoints

### Guild Management Endpoints
```yaml
POST /guilds/create:
  Description: Create new guild
  Request: { 
    name, 
    description, 
    emblem_url, 
    banner_url 
  }
  Response: { 
    guild_id, 
    status: 'created' 
  }
  Rate Limit: 1/hour per user

GET /guilds/{guild_id}:
  Description: Get guild information
  Request: { guild_id }
  Response: { 
    guild_info, 
    member_count, 
    level, 
    treasury 
  }
  Rate Limit: 1000/hour per user

PUT /guilds/{guild_id}:
  Description: Update guild information
  Request: { 
    guild_id, 
    updates 
  }
  Response: { 
    success: true, 
    updated_guild 
  }
  Rate Limit: 10/hour per guild master

DELETE /guilds/{guild_id}:
  Description: Disband guild
  Request: { guild_id }
  Response: { 
    success: true 
  }
  Rate Limit: 1/day per guild master
```

### Member Management Endpoints
```yaml
POST /guilds/{guild_id}/members/invite:
  Description: Invite user to guild
  Request: { 
    guild_id, 
    user_id, 
    role 
  }
  Response: { 
    invitation_id, 
    status: 'sent' 
  }
  Rate Limit: 50/hour per officer

POST /guilds/{guild_id}/members/join:
  Description: Join guild
  Request: { 
    guild_id, 
    invitation_id 
  }
  Response: { 
    success: true, 
    member_info 
  }
  Rate Limit: 10/hour per user

POST /guilds/{guild_id}/members/leave:
  Description: Leave guild
  Request: { guild_id }
  Response: { 
    success: true 
  }
  Rate Limit: 1/hour per member

PUT /guilds/{guild_id}/members/{user_id}/role:
  Description: Change member role
  Request: { 
    guild_id, 
    user_id, 
    new_role 
  }
  Response: { 
    success: true, 
    updated_role 
  }
  Rate Limit: 100/hour per officer
```

### Guild Activities Endpoints
```yaml
POST /guilds/{guild_id}/activities:
  Description: Create guild activity
  Request: { 
    guild_id, 
    activity_type, 
    title, 
    description, 
    start_time, 
    max_participants, 
    rewards 
  }
  Response: { 
    activity_id, 
    status: 'created' 
  }
  Rate Limit: 10/hour per officer

GET /guilds/{guild_id}/activities:
  Description: Get guild activities
  Request: { 
    guild_id, 
    activity_type, 
    status, 
    page, 
    limit 
  }
  Response: { 
    activities: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user

POST /guilds/{guild_id}/activities/{activity_id}/join:
  Description: Join guild activity
  Request: { 
    guild_id, 
    activity_id, 
    user_id 
  }
  Response: { 
    success: true, 
    participant_info 
  }
  Rate Limit: 10/hour per user
```

### Guild War Endpoints
```yaml
POST /guilds/{guild_id}/wars/declare:
  Description: Declare war on another guild
  Request: { 
    guild_id, 
    target_guild_id, 
    war_type, 
    start_time 
  }
  Response: { 
    war_id, 
    status: 'declared' 
  }
  Rate Limit: 1/day per guild master

GET /guilds/{guild_id}/wars:
  Description: Get guild wars
  Request: { 
    guild_id, 
    war_type, 
    status, 
    page, 
    limit 
  }
  Response: { 
    wars: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user

POST /guilds/{guild_id}/wars/{war_id}/accept:
  Description: Accept war declaration
  Request: { 
    guild_id, 
    war_id 
  }
  Response: { 
    success: true, 
    status: 'accepted' 
  }
  Rate Limit: 1/hour per guild master
```

### Guild Storage Endpoints
```yaml
GET /guilds/{guild_id}/storage:
  Description: Get guild storage
  Request: { guild_id }
  Response: { 
    storage: [...], 
    total_slots, 
    used_slots 
  }
  Rate Limit: 1000/hour per member

POST /guilds/{guild_id}/storage/deposit:
  Description: Deposit item to guild storage
  Request: { 
    guild_id, 
    item_id, 
    quantity, 
    slot_position 
  }
  Response: { 
    success: true, 
    slot_position 
  }
  Rate Limit: 100/hour per member

POST /guilds/{guild_id}/storage/withdraw:
  Description: Withdraw item from guild storage
  Request: { 
    guild_id, 
    slot_position, 
    quantity 
  }
  Response: { 
    success: true, 
    withdrawn_item 
  }
  Rate Limit: 100/hour per member
```

## ⚔️ Guild Wars

### War Types
```yaml
Territory Wars:
  - Control of specific areas
  - Resource bonuses
  - Strategic advantages
  - Long-term benefits

Honor Wars:
  - Prestige and ranking
  - Temporary bonuses
  - Bragging rights
  - Short-term benefits

Resource Wars:
  - Direct resource competition
  - Immediate rewards
  - High stakes
  - Quick resolution

Alliance Wars:
  - Multi-guild conflicts
  - Complex strategies
  - Large-scale battles
  - Diplomatic elements
```

### War Mechanics
```yaml
War Phases:
  - Declaration phase
  - Preparation phase
  - Active battle phase
  - Resolution phase
  - Reward distribution

War Scoring:
  - Battle victories
  - Territory control
  - Resource capture
  - Member participation
  - Strategic objectives

War Rewards:
  - Experience points
  - Guild resources
  - Special items
  - Territory bonuses
  - Prestige points
```

## 📊 Monitoring & Analytics

### Guild Metrics
```yaml
Guild Performance:
  - Member count growth
  - Activity participation
  - War win rate
  - Resource accumulation
  - Level progression

Member Engagement:
  - Daily active members
  - Activity participation
  - Contribution points
  - Online time
  - Retention rate

War Analytics:
  - War frequency
  - Win/loss ratio
  - Battle duration
  - Member participation
  - Reward distribution
```

### Real-time Dashboards
```yaml
Live Monitoring:
  - Active guilds
  - Online members
  - Active wars
  - Guild activities
  - System health

Historical Analysis:
  - Guild growth trends
  - War patterns
  - Member behavior
  - Resource flow
  - Performance metrics
```

## 🧪 Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_guild_creation() {
        // Test guild creation flow
    }
    
    #[tokio::test]
    async fn test_member_management() {
        // Test member invitation, joining, leaving
    }
    
    #[tokio::test]
    async fn test_guild_activities() {
        // Test activity creation and participation
    }
    
    #[tokio::test]
    async fn test_guild_wars() {
        // Test war declaration and management
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_complete_guild_flow() {
    // Test complete guild flow
    let service = GuildService::new();
    
    // Create guild
    let guild = service.create_guild(GuildRequest {
        name: "Test Guild",
        description: "A test guild",
        master_id: "user_1",
    }).await.unwrap();
    
    // Invite member
    service.invite_member(&guild.guild_id, "user_2", "member").await.unwrap();
    
    // Join guild
    service.join_guild(&guild.guild_id, "invitation_123", "user_2").await.unwrap();
    
    // Create activity
    let activity = service.create_activity(ActivityRequest {
        guild_id: &guild.guild_id,
        activity_type: "quest",
        title: "Test Quest",
        start_time: "2023-01-01T00:00:00Z",
    }).await.unwrap();
    
    // Join activity
    service.join_activity(&guild.guild_id, &activity.activity_id, "user_2").await.unwrap();
    
    // Verify guild state
    let guild_info = service.get_guild(&guild.guild_id).await.unwrap();
    assert_eq!(guild_info.member_count, 2);
    assert_eq!(guild_info.activities.len(), 1);
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
  let response = http.get('http://localhost:8088/guilds/test_guild');
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
COPY --from=builder /app/target/release/guild-service /usr/local/bin/
EXPOSE 8088
CMD ["guild-service"]
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: guild-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: guild-service
  template:
    metadata:
      labels:
        app: guild-service
    spec:
      containers:
      - name: guild-service
        image: guild-service:latest
        ports:
        - containerPort: 8088
        env:
        - name: DATABASE_URL
          value: "postgresql://user:pass@postgres:5432/guild_db"
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
DATABASE_URL=postgresql://user:pass@localhost:5432/guild_db
DATABASE_POOL_SIZE=20
DATABASE_TIMEOUT=30s

# Redis Configuration
REDIS_URL=redis://localhost:6379
REDIS_PASSWORD=secret
REDIS_DB=6

# Kafka Configuration
KAFKA_BROKERS=localhost:9092
KAFKA_TOPIC_PREFIX=guild
KAFKA_GROUP_ID=guild_service

# External Services
CHAOS_BACKEND_URL=grpc://chaos-backend:9090
INVENTORY_SERVICE_URL=grpc://inventory-service:9094
CHAT_SERVICE_URL=grpc://chat-service:9095

# Server Configuration
SERVER_PORT=8088
SERVER_HOST=0.0.0.0
SERVER_WORKERS=4

# Guild Configuration
MAX_GUILD_MEMBERS=1000
MAX_GUILD_LEVEL=50
GUILD_EXPERIENCE_MULTIPLIER=1.0
GUILD_WAR_COOLDOWN=86400
```

### Configuration File
```yaml
# guild-config.yaml
server:
  port: 8088
  host: "0.0.0.0"
  workers: 4
  max_connections: 10000

database:
  url: "postgresql://user:pass@localhost:5432/guild_db"
  pool_size: 20
  timeout: "30s"
  ssl_mode: "prefer"

redis:
  url: "redis://localhost:6379"
  password: "secret"
  db: 6
  pool_size: 100

kafka:
  brokers: ["localhost:9092"]
  topic_prefix: "guild"
  group_id: "guild_service"
  auto_offset_reset: "latest"

external_services:
  chaos_backend:
    url: "grpc://chaos-backend:9090"
    timeout: "5s"
    retries: 3
  
  inventory_service:
    url: "grpc://inventory-service:9094"
    timeout: "5s"
    retries: 3
  
  chat_service:
    url: "grpc://chat-service:9095"
    timeout: "5s"
    retries: 3

guild_settings:
  max_members: 1000
  max_level: 50
  experience_multiplier: 1.0
  war_cooldown: 86400
  activity_cooldown: 3600
  storage_slots: 1000
  max_activities: 10
  max_wars: 5

permissions:
  master:
    - create_activities
    - manage_members
    - declare_wars
    - manage_storage
    - disband_guild
  
  vice_master:
    - create_activities
    - manage_members
    - manage_storage
    - invite_members
  
  officer:
    - create_activities
    - invite_members
    - manage_storage
  
  member:
    - join_activities
    - use_storage
    - view_guild_info
```

## 🔗 Related Services

- [Chaos Backend](./chaos-backend/README.md) - Game Logic Core
- [Inventory Service](./inventory-service/README.md) - Guild storage
- [Chat Service](./chat-service/README.md) - Guild chat
- [User Management](./user-management/README.md) - User authentication
- [Microservices Architecture](./microservices-architecture/README.md) - Overall Architecture
