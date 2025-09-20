# Matchmaking Service - Chaos World

## 📋 Overview

Matchmaking Service chịu trách nhiệm ghép đôi người chơi cho các hoạt động PvP, dungeons, raids, và team formation trong Chaos World MMORPG.

## 🎯 Responsibilities

### Core Functions
- **PvP Matchmaking**: Ghép đôi cho PvP battles
- **Dungeon Matching**: Ghép nhóm cho dungeons
- **Raid Matching**: Ghép nhóm cho raids
- **Team Formation**: Tạo team cho các hoạt động
- **Skill-based Matching**: Ghép đôi dựa trên skill level
- **Queue Management**: Quản lý hàng đợi

### Performance Requirements
- **Latency**: < 100ms cho matchmaking
- **Throughput**: 10,000+ matches/second
- **Availability**: 99.9% uptime
- **Concurrency**: Handle 100,000+ concurrent players

## 🏗️ Architecture

### Technology Stack
```yaml
Language: Rust
Framework: Axum
Database: PostgreSQL (match data), Redis (queues)
Message Queue: Apache Kafka
Cache: Redis Cluster
Algorithm: ELO rating system
```

### Core Components
```rust
pub struct MatchmakingService {
    // Database
    match_repository: Arc<MatchRepository>,
    player_repository: Arc<PlayerRepository>,
    queue_repository: Arc<QueueRepository>,
    rating_repository: Arc<RatingRepository>,
    
    // Matchmaking
    pvp_matcher: Arc<PvPMatcher>,
    dungeon_matcher: Arc<DungeonMatcher>,
    raid_matcher: Arc<RaidMatcher>,
    team_matcher: Arc<TeamMatcher>,
    
    // Rating System
    elo_calculator: Arc<EloCalculator>,
    skill_analyzer: Arc<SkillAnalyzer>,
    rating_updater: Arc<RatingUpdater>,
    
    // Queue Management
    queue_manager: Arc<QueueManager>,
    priority_manager: Arc<PriorityManager>,
    timeout_manager: Arc<TimeoutManager>,
    
    // External Services
    chaos_backend_client: Arc<ChaosBackendClient>,
    user_management_client: Arc<UserManagementClient>,
    analytics_service_client: Arc<AnalyticsServiceClient>,
    
    // Configuration
    config: MatchmakingConfig,
}
```

## ⚔️ Matchmaking Types

### PvP Matchmaking
```yaml
PvP Modes:
  - 1v1 Duel
  - 2v2 Team Battle
  - 3v3 Team Battle
  - 5v5 Team Battle
  - 10v10 Guild War
  - Battle Royale (100 players)

PvP Criteria:
  - Skill rating (ELO)
  - Level range
  - Equipment score
  - Win/loss ratio
  - Recent performance
  - Ping/latency
```

### Dungeon Matchmaking
```yaml
Dungeon Types:
  - Solo Dungeons (1 player)
  - Party Dungeons (2-5 players)
  - Raid Dungeons (6-20 players)
  - Guild Dungeons (Guild members only)
  - Event Dungeons (Special events)

Dungeon Criteria:
  - Level requirement
  - Class composition
  - Experience level
  - Equipment requirements
  - Completion rate
  - Time availability
```

### Raid Matchmaking
```yaml
Raid Types:
  - Normal Raids (6-10 players)
  - Heroic Raids (10-15 players)
  - Mythic Raids (15-20 players)
  - World Raids (20+ players)
  - Cross-server Raids

Raid Criteria:
  - Role requirements (Tank, Healer, DPS)
  - Skill level
  - Equipment score
  - Raid experience
  - Availability
  - Communication preferences
```

### Team Formation
```yaml
Team Types:
  - PvP Teams
  - PvE Teams
  - Guild Teams
  - Tournament Teams
  - Event Teams

Team Criteria:
  - Role balance
  - Skill compatibility
  - Communication
  - Availability
  - Experience level
  - Equipment level
```

## 🗄️ Database Schema

### PostgreSQL Tables
```sql
-- Player ratings table
CREATE TABLE player_ratings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    game_mode VARCHAR(50) NOT NULL, -- 'pvp_1v1', 'pvp_2v2', 'dungeon', 'raid'
    rating INTEGER NOT NULL DEFAULT 1200,
    wins INTEGER DEFAULT 0,
    losses INTEGER DEFAULT 0,
    draws INTEGER DEFAULT 0,
    win_streak INTEGER DEFAULT 0,
    loss_streak INTEGER DEFAULT 0,
    last_played TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, game_mode)
);

-- Match queues table
CREATE TABLE match_queues (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    queue_id VARCHAR(50) UNIQUE NOT NULL,
    user_id UUID NOT NULL,
    game_mode VARCHAR(50) NOT NULL,
    queue_type VARCHAR(20) NOT NULL, -- 'pvp', 'dungeon', 'raid', 'team'
    priority INTEGER DEFAULT 0,
    preferences JSONB,
    status VARCHAR(20) DEFAULT 'waiting', -- 'waiting', 'matched', 'cancelled', 'expired'
    queued_at TIMESTAMP DEFAULT NOW(),
    matched_at TIMESTAMP,
    expires_at TIMESTAMP
);

-- Matches table
CREATE TABLE matches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    match_id VARCHAR(50) UNIQUE NOT NULL,
    game_mode VARCHAR(50) NOT NULL,
    match_type VARCHAR(20) NOT NULL, -- 'pvp', 'dungeon', 'raid', 'team'
    participants JSONB NOT NULL,
    match_data JSONB,
    status VARCHAR(20) DEFAULT 'pending', -- 'pending', 'active', 'completed', 'cancelled'
    started_at TIMESTAMP,
    ended_at TIMESTAMP,
    duration INTEGER, -- seconds
    winner_team INTEGER,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Match results table
CREATE TABLE match_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    match_id VARCHAR(50) NOT NULL,
    user_id UUID NOT NULL,
    team_id INTEGER,
    result VARCHAR(20) NOT NULL, -- 'win', 'loss', 'draw'
    rating_change INTEGER NOT NULL,
    new_rating INTEGER NOT NULL,
    performance_score DECIMAL(5,2),
    rewards JSONB,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Team formations table
CREATE TABLE team_formations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id VARCHAR(50) UNIQUE NOT NULL,
    team_name VARCHAR(100),
    team_type VARCHAR(20) NOT NULL, -- 'pvp', 'pve', 'guild', 'tournament'
    leader_id UUID NOT NULL,
    members JSONB NOT NULL,
    team_rating INTEGER DEFAULT 0,
    wins INTEGER DEFAULT 0,
    losses INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Matchmaking preferences table
CREATE TABLE matchmaking_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID UNIQUE NOT NULL,
    preferred_game_modes JSONB,
    skill_level_preference VARCHAR(20) DEFAULT 'balanced', -- 'balanced', 'challenging', 'easy'
    ping_tolerance INTEGER DEFAULT 100, -- milliseconds
    language_preference VARCHAR(10) DEFAULT 'en',
    communication_preference VARCHAR(20) DEFAULT 'any', -- 'voice', 'text', 'any', 'none'
    time_availability JSONB,
    auto_accept_matches BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

### Redis Cache Structure
```yaml
Active Queues:
  key: "queue:{game_mode}:{queue_type}"
  value: { players: [...], count: 150, avg_wait_time: 30 }
  ttl: 60 seconds

Player Ratings:
  key: "rating:{user_id}:{game_mode}"
  value: { rating: 1500, wins: 100, losses: 50, ... }
  ttl: 3600 seconds

Active Matches:
  key: "match:{match_id}"
  value: { participants: [...], status: 'active', ... }
  ttl: 3600 seconds

Team Formations:
  key: "team:{team_id}"
  value: { members: [...], rating: 1500, ... }
  ttl: 1800 seconds
```

## 🔌 API Endpoints

### Queue Management Endpoints
```yaml
POST /queues/join:
  Description: Join matchmaking queue
  Request: { 
    game_mode, 
    queue_type, 
    preferences 
  }
  Response: { 
    queue_id, 
    estimated_wait_time, 
    position_in_queue 
  }
  Rate Limit: 10/minute per user

POST /queues/leave:
  Description: Leave matchmaking queue
  Request: { queue_id }
  Response: { 
    success: true, 
    time_in_queue 
  }
  Rate Limit: 10/minute per user

GET /queues/status:
  Description: Get queue status
  Request: { 
    game_mode, 
    queue_type 
  }
  Response: { 
    queue_size, 
    estimated_wait_time, 
    average_wait_time 
  }
  Rate Limit: 1000/hour per user
```

### Match Management Endpoints
```yaml
GET /matches/{match_id}:
  Description: Get match information
  Request: { match_id }
  Response: { 
    match_info, 
    participants, 
    status, 
    match_data 
  }
  Rate Limit: 1000/hour per user

POST /matches/{match_id}/accept:
  Description: Accept match
  Request: { 
    match_id, 
    user_id 
  }
  Response: { 
    success: true, 
    match_status 
  }
  Rate Limit: 10/minute per user

POST /matches/{match_id}/decline:
  Description: Decline match
  Request: { 
    match_id, 
    user_id, 
    reason 
  }
  Response: { 
    success: true 
  }
  Rate Limit: 10/minute per user
```

### Rating Endpoints
```yaml
GET /ratings/{user_id}:
  Description: Get player ratings
  Request: { 
    user_id, 
    game_mode 
  }
  Response: { 
    ratings: [...], 
    overall_rating, 
    rank 
  }
  Rate Limit: 1000/hour per user

GET /ratings/leaderboard:
  Description: Get leaderboard
  Request: { 
    game_mode, 
    season, 
    page, 
    limit 
  }
  Response: { 
    leaderboard: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user

POST /ratings/update:
  Description: Update ratings after match
  Request: { 
    match_id, 
    results 
  }
  Response: { 
    success: true, 
    updated_ratings 
  }
  Rate Limit: 1000/hour per system
```

### Team Formation Endpoints
```yaml
POST /teams/create:
  Description: Create team
  Request: { 
    team_name, 
    team_type, 
    members 
  }
  Response: { 
    team_id, 
    status: 'created' 
  }
  Rate Limit: 10/hour per user

POST /teams/{team_id}/invite:
  Description: Invite player to team
  Request: { 
    team_id, 
    user_id 
  }
  Response: { 
    invitation_id, 
    status: 'sent' 
  }
  Rate Limit: 50/hour per team leader

POST /teams/{team_id}/join:
  Description: Join team
  Request: { 
    team_id, 
    invitation_id 
  }
  Response: { 
    success: true, 
    team_info 
  }
  Rate Limit: 10/hour per user

GET /teams/{team_id}:
  Description: Get team information
  Request: { team_id }
  Response: { 
    team_info, 
    members, 
    rating, 
    stats 
  }
  Rate Limit: 1000/hour per user
```

## 🎯 Matchmaking Algorithms

### ELO Rating System
```yaml
ELO Calculation:
  - Initial rating: 1200
  - K-factor: 32 (new players), 16 (experienced)
  - Expected score: 1 / (1 + 10^((opponent_rating - player_rating) / 400))
  - Rating change: K * (actual_score - expected_score)

Rating Tiers:
  - Bronze: 0-999
  - Silver: 1000-1299
  - Gold: 1300-1599
  - Platinum: 1600-1899
  - Diamond: 1900-2199
  - Master: 2200-2499
  - Grandmaster: 2500+
```

### Skill-based Matching
```yaml
Matching Criteria:
  - Rating difference: ±200 points
  - Level difference: ±5 levels
  - Equipment score: ±100 points
  - Ping: < 100ms
  - Recent performance: Last 10 matches

Matching Timeout:
  - Initial: 30 seconds
  - Extended: 60 seconds
  - Maximum: 120 seconds
  - Fallback: Relaxed criteria
```

### Queue Management
```yaml
Queue Priorities:
  - VIP players: Priority 1
  - Premium players: Priority 2
  - Regular players: Priority 3
  - New players: Priority 4

Queue Balancing:
  - Role balance (Tank, Healer, DPS)
  - Skill balance
  - Ping optimization
  - Geographic proximity
  - Language preference
```

## 📊 Monitoring & Analytics

### Matchmaking Metrics
```yaml
Performance Metrics:
  - Average wait time
  - Match success rate
  - Queue abandonment rate
  - Rating accuracy
  - System latency

Quality Metrics:
  - Match balance
  - Player satisfaction
  - Fair play score
  - Skill distribution
  - Queue efficiency
```

### Real-time Dashboards
```yaml
Live Monitoring:
  - Active queues
  - Queue sizes
  - Average wait times
  - Match success rate
  - System health

Historical Analysis:
  - Queue trends
  - Match patterns
  - Rating distributions
  - Player behavior
  - Performance trends
```

## 🧪 Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_pvp_matchmaking() {
        // Test PvP matching algorithm
    }
    
    #[tokio::test]
    async fn test_dungeon_matchmaking() {
        // Test dungeon matching
    }
    
    #[tokio::test]
    async fn test_elo_calculation() {
        // Test ELO rating calculation
    }
    
    #[tokio::test]
    async fn test_queue_management() {
        // Test queue operations
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_complete_matchmaking_flow() {
    // Test complete matchmaking flow
    let service = MatchmakingService::new();
    
    // Join PvP queue
    let queue = service.join_queue(QueueRequest {
        game_mode: "pvp_1v1",
        queue_type: "pvp",
        user_id: "player_1",
    }).await.unwrap();
    
    // Join another player
    service.join_queue(QueueRequest {
        game_mode: "pvp_1v1",
        queue_type: "pvp",
        user_id: "player_2",
    }).await.unwrap();
    
    // Wait for match
    let match_result = service.wait_for_match(&queue.queue_id).await.unwrap();
    
    // Verify match
    assert_eq!(match_result.participants.len(), 2);
    assert!(match_result.participants.contains(&"player_1"));
    assert!(match_result.participants.contains(&"player_2"));
    
    // Complete match
    service.complete_match(CompleteMatchRequest {
        match_id: &match_result.match_id,
        winner: "player_1",
        results: vec![
            MatchResult { user_id: "player_1", result: "win", rating_change: 16 },
            MatchResult { user_id: "player_2", result: "loss", rating_change: -16 },
        ],
    }).await.unwrap();
    
    // Verify rating updates
    let player1_rating = service.get_rating("player_1", "pvp_1v1").await.unwrap();
    let player2_rating = service.get_rating("player_2", "pvp_1v1").await.unwrap();
    
    assert_eq!(player1_rating.rating, 1216);
    assert_eq!(player2_rating.rating, 1184);
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
  let response = http.post('http://localhost:8091/queues/join', JSON.stringify({
    game_mode: 'pvp_1v1',
    queue_type: 'pvp',
    user_id: 'test_user'
  }));
  
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
COPY --from=builder /app/target/release/matchmaking-service /usr/local/bin/
EXPOSE 8091
CMD ["matchmaking-service"]
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: matchmaking-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: matchmaking-service
  template:
    metadata:
      labels:
        app: matchmaking-service
    spec:
      containers:
      - name: matchmaking-service
        image: matchmaking-service:latest
        ports:
        - containerPort: 8091
        env:
        - name: DATABASE_URL
          value: "postgresql://user:pass@postgres:5432/matchmaking_db"
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
DATABASE_URL=postgresql://user:pass@localhost:5432/matchmaking_db
DATABASE_POOL_SIZE=20
DATABASE_TIMEOUT=30s

# Redis Configuration
REDIS_URL=redis://localhost:6379
REDIS_PASSWORD=secret
REDIS_DB=9

# Kafka Configuration
KAFKA_BROKERS=localhost:9092
KAFKA_TOPIC_PREFIX=matchmaking
KAFKA_GROUP_ID=matchmaking_service

# External Services
CHAOS_BACKEND_URL=grpc://chaos-backend:9090
USER_MANAGEMENT_URL=grpc://user-management:9091
ANALYTICS_SERVICE_URL=grpc://analytics-service:9097

# Server Configuration
SERVER_PORT=8091
SERVER_HOST=0.0.0.0
SERVER_WORKERS=4

# Matchmaking Configuration
MAX_QUEUE_SIZE=10000
MATCH_TIMEOUT=120
RATING_UPDATE_BATCH_SIZE=100
```

### Configuration File
```yaml
# matchmaking-config.yaml
server:
  port: 8091
  host: "0.0.0.0"
  workers: 4
  max_connections: 10000

database:
  url: "postgresql://user:pass@localhost:5432/matchmaking_db"
  pool_size: 20
  timeout: "30s"
  ssl_mode: "prefer"

redis:
  url: "redis://localhost:6379"
  password: "secret"
  db: 9
  pool_size: 100

kafka:
  brokers: ["localhost:9092"]
  topic_prefix: "matchmaking"
  group_id: "matchmaking_service"
  auto_offset_reset: "latest"

external_services:
  chaos_backend:
    url: "grpc://chaos-backend:9090"
    timeout: "5s"
    retries: 3
  
  user_management:
    url: "grpc://user-management:9091"
    timeout: "5s"
    retries: 3
  
  analytics_service:
    url: "grpc://analytics-service:9097"
    timeout: "5s"
    retries: 3

matchmaking_settings:
  max_queue_size: 10000
  match_timeout: 120
  rating_update_batch_size: 100
  elo_k_factor_new: 32
  elo_k_factor_experienced: 16
  initial_rating: 1200
  max_rating_difference: 200
  max_level_difference: 5
  max_ping: 100
  queue_priority_levels: 4

game_modes:
  pvp_1v1:
    min_players: 2
    max_players: 2
    max_wait_time: 60
    skill_weight: 0.8
    ping_weight: 0.2
  
  pvp_2v2:
    min_players: 4
    max_players: 4
    max_wait_time: 90
    skill_weight: 0.7
    ping_weight: 0.3
  
  dungeon:
    min_players: 1
    max_players: 5
    max_wait_time: 120
    role_balance: true
    skill_weight: 0.6
    ping_weight: 0.4
  
  raid:
    min_players: 6
    max_players: 20
    max_wait_time: 180
    role_balance: true
    skill_weight: 0.5
    ping_weight: 0.5
```

## 🔗 Related Services

- [Chaos Backend](./chaos-backend/README.md) - Game Logic Core
- [User Management](./user-management/README.md) - User authentication
- [Analytics Service](./analytics-service/README.md) - Matchmaking analytics
- [Guild Service](./guild-service/README.md) - Team formation
- [Microservices Architecture](./microservices-architecture/README.md) - Overall Architecture
