# World Service - Chaos World

## 📋 Overview

World Service chịu trách nhiệm quản lý thế giới game, bao gồm world state, environment simulation, weather system, day/night cycle, và world events.

## 🎯 Responsibilities

### Core Functions
- **World State Management**: Quản lý trạng thái thế giới
- **Environment Simulation**: Mô phỏng môi trường
- **Weather System**: Hệ thống thời tiết
- **Day/Night Cycle**: Chu kỳ ngày/đêm
- **World Events**: Sự kiện thế giới
- **Territory Management**: Quản lý lãnh thổ

### Performance Requirements
- **Latency**: < 50ms cho world updates
- **Throughput**: 20,000+ updates/second
- **Availability**: 99.9% uptime
- **Concurrency**: Handle 1,000,000+ concurrent world interactions

## 🏗️ Architecture

### Technology Stack
```yaml
Language: Rust
Framework: Axum
Database: PostgreSQL (world state), Redis (real-time)
Message Queue: Apache Kafka
Time Series: InfluxDB (weather data)
Cache: Redis Cluster
```

### Core Components
```rust
pub struct WorldService {
    // Database
    world_repository: Arc<WorldRepository>,
    environment_repository: Arc<EnvironmentRepository>,
    event_repository: Arc<EventRepository>,
    territory_repository: Arc<TerritoryRepository>,
    
    // World Management
    world_manager: Arc<WorldManager>,
    environment_manager: Arc<EnvironmentManager>,
    weather_manager: Arc<WeatherManager>,
    time_manager: Arc<TimeManager>,
    
    // World Events
    event_manager: Arc<EventManager>,
    event_scheduler: Arc<EventScheduler>,
    event_processor: Arc<EventProcessor>,
    
    // Territory System
    territory_manager: Arc<TerritoryManager>,
    zone_manager: Arc<ZoneManager>,
    spawn_manager: Arc<SpawnManager>,
    
    // External Services
    chaos_backend_client: Arc<ChaosBackendClient>,
    guild_service_client: Arc<GuildServiceClient>,
    analytics_service_client: Arc<AnalyticsServiceClient>,
    
    // Configuration
    config: WorldConfig,
}
```

## 🌍 World System

### World Structure
```yaml
World Zones:
  - Starting Areas (Level 1-10)
  - Intermediate Areas (Level 11-30)
  - Advanced Areas (Level 31-50)
  - Elite Areas (Level 51-70)
  - Legendary Areas (Level 71-100)

Zone Types:
  - Safe Zones (No PvP)
  - PvP Zones (Player vs Player)
  - PvE Zones (Player vs Environment)
  - Dungeon Zones (Instanced)
  - Raid Zones (Group content)
  - Guild Zones (Guild territory)
```

### Environment Features
```yaml
Weather System:
  - Sunny (Normal conditions)
  - Rainy (Water magic bonus)
  - Snowy (Ice magic bonus)
  - Foggy (Stealth bonus)
  - Stormy (Lightning magic bonus)
  - Windy (Air magic bonus)

Day/Night Cycle:
  - Dawn (6:00-8:00)
  - Day (8:00-18:00)
  - Dusk (18:00-20:00)
  - Night (20:00-6:00)
  - Midnight (0:00-2:00)

Seasonal Changes:
  - Spring (Growth bonus)
  - Summer (Fire magic bonus)
  - Autumn (Harvest bonus)
  - Winter (Ice magic bonus)
```

### World Events
```yaml
Scheduled Events:
  - Daily events
  - Weekly events
  - Monthly events
  - Seasonal events
  - Holiday events

Random Events:
  - Weather changes
  - Monster spawns
  - Resource spawns
  - Treasure discoveries
  - Environmental hazards

Special Events:
  - World boss spawns
  - Guild wars
  - Server competitions
  - Community events
  - Developer events
```

## 🗄️ Database Schema

### PostgreSQL Tables
```sql
-- World zones table
CREATE TABLE world_zones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    zone_id VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    zone_type VARCHAR(20) NOT NULL, -- 'safe', 'pvp', 'pve', 'dungeon', 'raid', 'guild'
    level_range_min INTEGER NOT NULL,
    level_range_max INTEGER NOT NULL,
    coordinates JSONB NOT NULL, -- {x_min, x_max, y_min, y_max, z_min, z_max}
    spawn_points JSONB,
    safe_zones JSONB,
    pvp_zones JSONB,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- World state table
CREATE TABLE world_state (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    zone_id VARCHAR(50) NOT NULL,
    state_type VARCHAR(50) NOT NULL, -- 'weather', 'time', 'season', 'event'
    state_value JSONB NOT NULL,
    timestamp TIMESTAMP DEFAULT NOW(),
    expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);

-- World events table
CREATE TABLE world_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id VARCHAR(50) UNIQUE NOT NULL,
    event_type VARCHAR(50) NOT NULL, -- 'scheduled', 'random', 'special'
    title VARCHAR(255) NOT NULL,
    description TEXT,
    zone_id VARCHAR(50),
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP,
    duration INTEGER, -- seconds
    max_participants INTEGER,
    current_participants INTEGER DEFAULT 0,
    status VARCHAR(20) DEFAULT 'scheduled', -- 'scheduled', 'active', 'completed', 'cancelled'
    rewards JSONB,
    requirements JSONB,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Territory table
CREATE TABLE territories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    territory_id VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    zone_id VARCHAR(50) NOT NULL,
    owner_guild_id VARCHAR(50),
    territory_type VARCHAR(20) NOT NULL, -- 'guild', 'neutral', 'contested'
    coordinates JSONB NOT NULL,
    resources JSONB,
    buildings JSONB,
    defenses JSONB,
    level INTEGER DEFAULT 1,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Spawn points table
CREATE TABLE spawn_points (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    spawn_id VARCHAR(50) UNIQUE NOT NULL,
    zone_id VARCHAR(50) NOT NULL,
    spawn_type VARCHAR(50) NOT NULL, -- 'monster', 'npc', 'resource', 'treasure'
    entity_id VARCHAR(50) NOT NULL,
    coordinates JSONB NOT NULL,
    spawn_radius INTEGER DEFAULT 100,
    spawn_count INTEGER DEFAULT 1,
    respawn_time INTEGER DEFAULT 300, -- seconds
    spawn_chance DECIMAL(5,2) DEFAULT 100.00,
    level_range JSONB,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Weather data table
CREATE TABLE weather_data (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    zone_id VARCHAR(50) NOT NULL,
    weather_type VARCHAR(20) NOT NULL,
    intensity DECIMAL(5,2) NOT NULL, -- 0.0 to 1.0
    temperature DECIMAL(5,2),
    humidity DECIMAL(5,2),
    wind_speed DECIMAL(5,2),
    wind_direction INTEGER, -- degrees
    effects JSONB,
    timestamp TIMESTAMP DEFAULT NOW()
);
```

### Redis Cache Structure
```yaml
World State:
  key: "world_state:{zone_id}"
  value: { weather, time, season, events, ... }
  ttl: 300 seconds

Active Events:
  key: "active_events:{zone_id}"
  value: { events: [...], count: 5 }
  ttl: 60 seconds

Territory Info:
  key: "territory:{territory_id}"
  value: { owner, level, resources, buildings, ... }
  ttl: 600 seconds

Spawn Points:
  key: "spawn_points:{zone_id}"
  value: { spawns: [...], active_count: 25 }
  ttl: 300 seconds
```

## 🔌 API Endpoints

### World State Endpoints
```yaml
GET /world/state:
  Description: Get current world state
  Request: { zone_id }
  Response: { 
    weather, 
    time, 
    season, 
    active_events, 
    territory_info 
  }
  Rate Limit: 1000/hour per user

GET /world/zones:
  Description: Get available zones
  Request: { 
    level_range, 
    zone_type, 
    page, 
    limit 
  }
  Response: { 
    zones: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user

GET /world/zones/{zone_id}:
  Description: Get zone information
  Request: { zone_id }
  Response: { 
    zone_info, 
    spawn_points, 
    safe_zones, 
    pvp_zones 
  }
  Rate Limit: 1000/hour per user
```

### Weather Endpoints
```yaml
GET /world/weather:
  Description: Get current weather
  Request: { zone_id }
  Response: { 
    weather_type, 
    intensity, 
    temperature, 
    effects 
  }
  Rate Limit: 1000/hour per user

GET /world/weather/forecast:
  Description: Get weather forecast
  Request: { 
    zone_id, 
    hours_ahead 
  }
  Response: { 
    forecast: [...], 
    accuracy 
  }
  Rate Limit: 100/hour per user

POST /world/weather/change:
  Description: Change weather (admin only)
  Request: { 
    zone_id, 
    weather_type, 
    intensity, 
    duration 
  }
  Response: { 
    success: true, 
    new_weather 
  }
  Rate Limit: 10/hour per admin
```

### World Events Endpoints
```yaml
GET /world/events:
  Description: Get world events
  Request: { 
    zone_id, 
    event_type, 
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

POST /world/events:
  Description: Create world event (admin only)
  Request: { 
    event_type, 
    title, 
    description, 
    zone_id, 
    start_time, 
    duration, 
    rewards 
  }
  Response: { 
    event_id, 
    status: 'created' 
  }
  Rate Limit: 10/hour per admin

POST /world/events/{event_id}/join:
  Description: Join world event
  Request: { 
    event_id, 
    user_id 
  }
  Response: { 
    success: true, 
    participant_info 
  }
  Rate Limit: 10/hour per user
```

### Territory Endpoints
```yaml
GET /world/territories:
  Description: Get territories
  Request: { 
    zone_id, 
    territory_type, 
    page, 
    limit 
  }
  Response: { 
    territories: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user

GET /world/territories/{territory_id}:
  Description: Get territory information
  Request: { territory_id }
  Response: { 
    territory_info, 
    owner_guild, 
    resources, 
    buildings, 
    defenses 
  }
  Rate Limit: 1000/hour per user

POST /world/territories/{territory_id}/claim:
  Description: Claim territory
  Request: { 
    territory_id, 
    guild_id 
  }
  Response: { 
    success: true, 
    claim_info 
  }
  Rate Limit: 1/hour per guild
```

## 🌦️ Weather System

### Weather Types
```yaml
Sunny:
  - Normal conditions
  - No special effects
  - Good visibility
  - Standard movement speed

Rainy:
  - Water magic +20% damage
  - Fire magic -10% damage
  - Reduced visibility
  - Slower movement speed

Snowy:
  - Ice magic +20% damage
  - Fire magic -20% damage
  - Reduced visibility
  - Slower movement speed

Foggy:
  - Stealth +30% effectiveness
  - Reduced visibility
  - Slower movement speed
  - Ranged attacks -15% accuracy

Stormy:
  - Lightning magic +30% damage
  - Reduced visibility
  - Slower movement speed
  - Chance of lightning strikes

Windy:
  - Air magic +20% damage
  - Ranged attacks +10% range
  - Faster movement speed
  - Projectile deviation
```

### Weather Effects
```yaml
Combat Effects:
  - Damage modifiers
  - Accuracy changes
  - Movement speed
  - Visibility range
  - Special abilities

Environmental Effects:
  - Resource spawn rates
  - Monster behavior
  - NPC interactions
  - World events
  - Territory bonuses

Player Effects:
  - Health regeneration
  - Mana regeneration
  - Stamina regeneration
  - Experience gain
  - Item drop rates
```

## 🕐 Time System

### Day/Night Cycle
```yaml
Time Periods:
  - Dawn (6:00-8:00): +10% experience gain
  - Day (8:00-18:00): Normal conditions
  - Dusk (18:00-20:00): +10% resource gathering
  - Night (20:00-6:00): +20% stealth, -10% visibility
  - Midnight (0:00-2:00): +30% rare item drops

Seasonal Effects:
  - Spring: +15% growth magic, +10% healing
  - Summer: +20% fire magic, +10% stamina
  - Autumn: +15% harvest, +10% gold drops
  - Winter: +20% ice magic, +10% defense
```

### Time-based Events
```yaml
Daily Events:
  - Morning meditation (6:00-8:00)
  - Midday market (12:00-14:00)
  - Evening gathering (18:00-20:00)
  - Night patrol (22:00-6:00)

Weekly Events:
  - Monday: Guild meetings
  - Wednesday: PvP tournaments
  - Friday: Raid nights
  - Sunday: Community events

Monthly Events:
  - Full moon: Special monsters
  - New moon: Stealth bonuses
  - Equinox: Balance bonuses
  - Solstice: Elemental bonuses
```

## 📊 Monitoring & Analytics

### World Metrics
```yaml
Environment Metrics:
  - Weather distribution
  - Time period activity
  - Seasonal changes
  - Event participation
  - Zone population

Territory Metrics:
  - Territory ownership
  - Guild control
  - Resource production
  - Building levels
  - Defense strength

Performance Metrics:
  - World update latency
  - Event processing time
  - Database query time
  - Cache hit rate
  - System availability
```

### Real-time Dashboards
```yaml
Live Monitoring:
  - Active zones
  - Current weather
  - Active events
  - Territory status
  - System health

Historical Analysis:
  - Weather patterns
  - Event trends
  - Territory changes
  - Player activity
  - Performance trends
```

## 🧪 Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_world_state_management() {
        // Test world state updates
    }
    
    #[tokio::test]
    async fn test_weather_system() {
        // Test weather changes and effects
    }
    
    #[tokio::test]
    async fn test_event_system() {
        // Test event creation and processing
    }
    
    #[tokio::test]
    async fn test_territory_management() {
        // Test territory claiming and management
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_complete_world_simulation() {
    // Test complete world simulation
    let service = WorldService::new();
    
    // Create zone
    let zone = service.create_zone(ZoneRequest {
        name: "Test Zone",
        zone_type: "pve",
        level_range: (1, 10),
        coordinates: Coordinates { x_min: 0, x_max: 1000, y_min: 0, y_max: 1000 },
    }).await.unwrap();
    
    // Set weather
    service.set_weather(&zone.zone_id, WeatherType::Rainy, 0.8).await.unwrap();
    
    // Create event
    let event = service.create_event(EventRequest {
        event_type: "scheduled",
        title: "Test Event",
        zone_id: &zone.zone_id,
        start_time: "2023-01-01T00:00:00Z",
        duration: 3600,
    }).await.unwrap();
    
    // Claim territory
    let territory = service.claim_territory(TerritoryRequest {
        zone_id: &zone.zone_id,
        guild_id: "guild_123",
        coordinates: Coordinates { x_min: 100, x_max: 200, y_min: 100, y_max: 200 },
    }).await.unwrap();
    
    // Verify world state
    let world_state = service.get_world_state(&zone.zone_id).await.unwrap();
    assert_eq!(world_state.weather, WeatherType::Rainy);
    assert_eq!(world_state.active_events.len(), 1);
    assert_eq!(world_state.territories.len(), 1);
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
  let response = http.get('http://localhost:8089/world/state?zone_id=test_zone');
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
COPY --from=builder /app/target/release/world-service /usr/local/bin/
EXPOSE 8089
CMD ["world-service"]
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: world-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: world-service
  template:
    metadata:
      labels:
        app: world-service
    spec:
      containers:
      - name: world-service
        image: world-service:latest
        ports:
        - containerPort: 8089
        env:
        - name: DATABASE_URL
          value: "postgresql://user:pass@postgres:5432/world_db"
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
DATABASE_URL=postgresql://user:pass@localhost:5432/world_db
DATABASE_POOL_SIZE=20
DATABASE_TIMEOUT=30s

# Redis Configuration
REDIS_URL=redis://localhost:6379
REDIS_PASSWORD=secret
REDIS_DB=7

# Kafka Configuration
KAFKA_BROKERS=localhost:9092
KAFKA_TOPIC_PREFIX=world
KAFKA_GROUP_ID=world_service

# InfluxDB Configuration
INFLUXDB_URL=http://localhost:8086
INFLUXDB_DATABASE=world_metrics
INFLUXDB_USERNAME=world_user
INFLUXDB_PASSWORD=world_pass

# External Services
CHAOS_BACKEND_URL=grpc://chaos-backend:9090
GUILD_SERVICE_URL=grpc://guild-service:9096
ANALYTICS_SERVICE_URL=grpc://analytics-service:9097

# Server Configuration
SERVER_PORT=8089
SERVER_HOST=0.0.0.0
SERVER_WORKERS=4

# World Configuration
WORLD_UPDATE_INTERVAL=1000
WEATHER_CHANGE_INTERVAL=3600
EVENT_CHECK_INTERVAL=60
TERRITORY_UPDATE_INTERVAL=300
```

### Configuration File
```yaml
# world-config.yaml
server:
  port: 8089
  host: "0.0.0.0"
  workers: 4
  max_connections: 10000

database:
  url: "postgresql://user:pass@localhost:5432/world_db"
  pool_size: 20
  timeout: "30s"
  ssl_mode: "prefer"

redis:
  url: "redis://localhost:6379"
  password: "secret"
  db: 7
  pool_size: 100

kafka:
  brokers: ["localhost:9092"]
  topic_prefix: "world"
  group_id: "world_service"
  auto_offset_reset: "latest"

influxdb:
  url: "http://localhost:8086"
  database: "world_metrics"
  username: "world_user"
  password: "world_pass"
  retention_policy: "30d"

external_services:
  chaos_backend:
    url: "grpc://chaos-backend:9090"
    timeout: "5s"
    retries: 3
  
  guild_service:
    url: "grpc://guild-service:9096"
    timeout: "5s"
    retries: 3
  
  analytics_service:
    url: "grpc://analytics-service:9097"
    timeout: "5s"
    retries: 3

world_settings:
  update_interval: 1000
  weather_change_interval: 3600
  event_check_interval: 60
  territory_update_interval: 300
  max_zones: 1000
  max_events_per_zone: 10
  max_territories_per_zone: 50

weather_settings:
  weather_types: ["sunny", "rainy", "snowy", "foggy", "stormy", "windy"]
  intensity_range: [0.0, 1.0]
  temperature_range: [-20.0, 40.0]
  humidity_range: [0.0, 100.0]
  wind_speed_range: [0.0, 50.0]

time_settings:
  day_duration: 86400  # 24 hours in seconds
  dawn_start: 6
  dawn_end: 8
  dusk_start: 18
  dusk_end: 20
  midnight_start: 0
  midnight_end: 2

event_settings:
  max_scheduled_events: 100
  max_random_events: 50
  max_special_events: 20
  event_duration_range: [300, 86400]  # 5 minutes to 24 hours
  max_participants_per_event: 1000
```

## 🔗 Related Services

- [Chaos Backend](./chaos-backend/README.md) - Game Logic Core
- [Guild Service](./guild-service/README.md) - Territory management
- [Analytics Service](./analytics-service/README.md) - World analytics
- [User Management](./user-management/README.md) - User authentication
- [Microservices Architecture](./microservices-architecture/README.md) - Overall Architecture
