# Content Management Service - Chaos World

## 📋 Overview

Content Management Service chịu trách nhiệm quản lý tất cả nội dung game, bao gồm item definitions, quest management, world configuration, và A/B testing content.

## 🎯 Responsibilities

### Core Functions
- **Item Management**: Quản lý item definitions và properties
- **Quest Management**: Tạo và quản lý quests
- **World Configuration**: Cấu hình thế giới game
- **A/B Testing**: Quản lý content cho A/B testing
- **Localization**: Quản lý đa ngôn ngữ
- **Version Control**: Quản lý phiên bản content

### Performance Requirements
- **Latency**: < 20ms cho content retrieval
- **Throughput**: 50,000+ requests/second
- **Availability**: 99.9% uptime
- **Concurrency**: Handle 1,000,000+ concurrent content requests

## 🏗️ Architecture

### Technology Stack
```yaml
Language: Rust
Framework: Axum
Database: PostgreSQL (content data), Redis (caching)
Search: Elasticsearch (content search)
File Storage: S3, MinIO
CDN: CloudFront, CloudFlare
```

### Core Components
```rust
pub struct ContentManagementService {
    // Database
    item_repository: Arc<ItemRepository>,
    quest_repository: Arc<QuestRepository>,
    world_repository: Arc<WorldRepository>,
    localization_repository: Arc<LocalizationRepository>,
    
    // Content Management
    item_manager: Arc<ItemManager>,
    quest_manager: Arc<QuestManager>,
    world_manager: Arc<WorldManager>,
    localization_manager: Arc<LocalizationManager>,
    
    // A/B Testing
    ab_test_manager: Arc<ABTestManager>,
    variant_manager: Arc<VariantManager>,
    experiment_manager: Arc<ExperimentManager>,
    
    // Version Control
    version_manager: Arc<VersionManager>,
    deployment_manager: Arc<DeploymentManager>,
    rollback_manager: Arc<RollbackManager>,
    
    // External Services
    chaos_backend_client: Arc<ChaosBackendClient>,
    analytics_service_client: Arc<AnalyticsServiceClient>,
    
    // Configuration
    config: ContentConfig,
}
```

## 📦 Content Types

### Item Definitions
```yaml
Equipment Items:
  - Weapons (swords, bows, staffs)
  - Armor (helmet, chest, legs, boots)
  - Accessories (rings, necklaces, bracelets)
  - Shields

Consumable Items:
  - Potions (health, mana, stamina)
  - Food (buffs, healing)
  - Scrolls (spells, teleportation)
  - Books (skills, knowledge)

Material Items:
  - Ores (iron, gold, mithril)
  - Gems (ruby, sapphire, diamond)
  - Herbs (healing, magical)
  - Cloth (silk, cotton, leather)

Special Items:
  - Keys (doors, chests)
  - Tools (mining, crafting)
  - Tokens (currency, rewards)
  - Quest Items
```

### Quest System
```yaml
Quest Types:
  - Main Quests (storyline)
  - Side Quests (optional)
  - Daily Quests (repeatable)
  - Weekly Quests (repeatable)
  - Guild Quests (guild-specific)
  - Event Quests (time-limited)

Quest Objectives:
  - Kill monsters
  - Collect items
  - Talk to NPCs
  - Explore areas
  - Complete dungeons
  - Craft items

Quest Rewards:
  - Experience points
  - Gold
  - Items
  - Skills
  - Reputation
  - Achievements
```

### World Configuration
```yaml
Zone Configuration:
  - Zone boundaries
  - Spawn points
  - Safe zones
  - PvP zones
  - Dungeon entrances
  - NPC locations

Environment Configuration:
  - Weather patterns
  - Day/night cycle
  - Seasonal changes
  - Environmental effects
  - Resource spawns
  - Event triggers
```

## 🗄️ Database Schema

### PostgreSQL Tables
```sql
-- Item definitions table
CREATE TABLE item_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    item_id VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    item_type VARCHAR(50) NOT NULL,
    rarity VARCHAR(20) NOT NULL,
    level_requirement INTEGER DEFAULT 1,
    value INTEGER DEFAULT 0,
    stack_size INTEGER DEFAULT 1,
    properties JSONB,
    requirements JSONB,
    effects JSONB,
    is_active BOOLEAN DEFAULT TRUE,
    version INTEGER DEFAULT 1,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Quest definitions table
CREATE TABLE quest_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    quest_id VARCHAR(50) UNIQUE NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    quest_type VARCHAR(50) NOT NULL,
    level_requirement INTEGER DEFAULT 1,
    prerequisites JSONB,
    objectives JSONB,
    rewards JSONB,
    npc_id VARCHAR(50),
    zone_id VARCHAR(50),
    is_repeatable BOOLEAN DEFAULT FALSE,
    cooldown INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    version INTEGER DEFAULT 1,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- World configuration table
CREATE TABLE world_configurations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    config_id VARCHAR(50) UNIQUE NOT NULL,
    config_type VARCHAR(50) NOT NULL, -- 'zone', 'environment', 'spawn', 'event'
    name VARCHAR(255) NOT NULL,
    description TEXT,
    configuration JSONB NOT NULL,
    zone_id VARCHAR(50),
    is_active BOOLEAN DEFAULT TRUE,
    version INTEGER DEFAULT 1,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Localization table
CREATE TABLE localizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content_id VARCHAR(50) NOT NULL,
    content_type VARCHAR(50) NOT NULL, -- 'item', 'quest', 'npc', 'zone'
    language VARCHAR(10) NOT NULL,
    field_name VARCHAR(50) NOT NULL, -- 'name', 'description', 'title'
    translated_text TEXT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(content_id, content_type, language, field_name)
);

-- A/B test variants table
CREATE TABLE ab_test_variants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    test_id VARCHAR(50) NOT NULL,
    variant_name VARCHAR(100) NOT NULL,
    variant_config JSONB NOT NULL,
    traffic_percentage DECIMAL(5,2) DEFAULT 0.0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Content versions table
CREATE TABLE content_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    version_id VARCHAR(50) UNIQUE NOT NULL,
    content_type VARCHAR(50) NOT NULL,
    content_id VARCHAR(50) NOT NULL,
    version_number INTEGER NOT NULL,
    changes JSONB,
    status VARCHAR(20) DEFAULT 'draft', -- 'draft', 'review', 'approved', 'deployed'
    created_by UUID NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    deployed_at TIMESTAMP
);
```

### Redis Cache Structure
```yaml
Item Definitions:
  key: "item_def:{item_id}"
  value: { name, type, rarity, properties, ... }
  ttl: 86400 seconds

Quest Definitions:
  key: "quest_def:{quest_id}"
  value: { title, objectives, rewards, ... }
  ttl: 86400 seconds

World Configuration:
  key: "world_config:{config_id}"
  value: { configuration, zone_id, ... }
  ttl: 3600 seconds

Localization:
  key: "localization:{content_id}:{language}"
  value: { name, description, title, ... }
  ttl: 86400 seconds
```

## 🔌 API Endpoints

### Item Management Endpoints
```yaml
GET /items:
  Description: Get item definitions
  Request: { 
    item_type, 
    rarity, 
    level_range, 
    page, 
    limit 
  }
  Response: { 
    items: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user

GET /items/{item_id}:
  Description: Get specific item definition
  Request: { item_id }
  Response: { 
    item_definition, 
    properties, 
    effects 
  }
  Rate Limit: 1000/hour per user

POST /items:
  Description: Create item definition (admin only)
  Request: { 
    item_id, 
    name, 
    description, 
    item_type, 
    rarity, 
    properties 
  }
  Response: { 
    item_id, 
    status: 'created' 
  }
  Rate Limit: 100/hour per admin

PUT /items/{item_id}:
  Description: Update item definition (admin only)
  Request: { 
    item_id, 
    updates 
  }
  Response: { 
    success: true, 
    updated_item 
  }
  Rate Limit: 100/hour per admin
```

### Quest Management Endpoints
```yaml
GET /quests:
  Description: Get quest definitions
  Request: { 
    quest_type, 
    level_range, 
    zone_id, 
    page, 
    limit 
  }
  Response: { 
    quests: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user

GET /quests/{quest_id}:
  Description: Get specific quest definition
  Request: { quest_id }
  Response: { 
    quest_definition, 
    objectives, 
    rewards 
  }
  Rate Limit: 1000/hour per user

POST /quests:
  Description: Create quest definition (admin only)
  Request: { 
    quest_id, 
    title, 
    description, 
    quest_type, 
    objectives, 
    rewards 
  }
  Response: { 
    quest_id, 
    status: 'created' 
  }
  Rate Limit: 100/hour per admin

PUT /quests/{quest_id}:
  Description: Update quest definition (admin only)
  Request: { 
    quest_id, 
    updates 
  }
  Response: { 
    success: true, 
    updated_quest 
  }
  Rate Limit: 100/hour per admin
```

### World Configuration Endpoints
```yaml
GET /world-configs:
  Description: Get world configurations
  Request: { 
    config_type, 
    zone_id, 
    page, 
    limit 
  }
  Response: { 
    configurations: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user

GET /world-configs/{config_id}:
  Description: Get specific world configuration
  Request: { config_id }
  Response: { 
    configuration, 
    zone_id, 
    config_type 
  }
  Rate Limit: 1000/hour per user

POST /world-configs:
  Description: Create world configuration (admin only)
  Request: { 
    config_id, 
    config_type, 
    name, 
    configuration 
  }
  Response: { 
    config_id, 
    status: 'created' 
  }
  Rate Limit: 100/hour per admin
```

### Localization Endpoints
```yaml
GET /localizations:
  Description: Get localizations
  Request: { 
    content_id, 
    content_type, 
    language, 
    page, 
    limit 
  }
  Response: { 
    localizations: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user

POST /localizations:
  Description: Create localization (admin only)
  Request: { 
    content_id, 
    content_type, 
    language, 
    field_name, 
    translated_text 
  }
  Response: { 
    localization_id, 
    status: 'created' 
  }
  Rate Limit: 100/hour per admin

PUT /localizations/{localization_id}:
  Description: Update localization (admin only)
  Request: { 
    localization_id, 
    translated_text 
  }
  Response: { 
    success: true, 
    updated_localization 
  }
  Rate Limit: 100/hour per admin
```

### A/B Testing Endpoints
```yaml
GET /ab-tests:
  Description: Get A/B tests
  Request: { 
    status, 
    page, 
    limit 
  }
  Response: { 
    tests: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user

POST /ab-tests:
  Description: Create A/B test (admin only)
  Request: { 
    test_id, 
    test_name, 
    variants, 
    traffic_allocation 
  }
  Response: { 
    test_id, 
    status: 'created' 
  }
  Rate Limit: 10/hour per admin

POST /ab-tests/{test_id}/assign:
  Description: Assign user to A/B test variant
  Request: { 
    test_id, 
    user_id 
  }
  Response: { 
    variant_name, 
    variant_config 
  }
  Rate Limit: 1000/hour per user
```

## 🔄 Version Control

### Version Management
```yaml
Version Types:
  - Major versions (breaking changes)
  - Minor versions (new features)
  - Patch versions (bug fixes)
  - Hotfix versions (critical fixes)

Version Lifecycle:
  - Draft: Initial creation
  - Review: Under review
  - Approved: Ready for deployment
  - Deployed: Live in production
  - Archived: Old versions
```

### Deployment Process
```yaml
Deployment Steps:
  1. Content validation
  2. Dependency checking
  3. Rollback preparation
  4. Gradual rollout
  5. Monitoring
  6. Full deployment

Rollback Strategy:
  - Automatic rollback on errors
  - Manual rollback capability
  - Version comparison
  - Data integrity checks
  - User impact assessment
```

## 🌍 Localization

### Supported Languages
```yaml
Primary Languages:
  - English (en)
  - Chinese Simplified (zh-CN)
  - Chinese Traditional (zh-TW)
  - Japanese (ja)
  - Korean (ko)

Secondary Languages:
  - Spanish (es)
  - French (fr)
  - German (de)
  - Portuguese (pt)
  - Russian (ru)
```

### Localization Features
```yaml
Content Localization:
  - Item names and descriptions
  - Quest titles and objectives
  - NPC dialogue
  - Zone names and descriptions
  - UI text and messages

Cultural Adaptation:
  - Date and time formats
  - Number formats
  - Currency symbols
  - Color preferences
  - Cultural references
```

## 📊 Monitoring & Analytics

### Content Metrics
```yaml
Usage Metrics:
  - Content access frequency
  - Popular items
  - Quest completion rates
  - A/B test performance
  - Localization usage

Performance Metrics:
  - Content retrieval time
  - Cache hit rate
  - Database query time
  - CDN performance
  - System availability
```

### A/B Testing Analytics
```yaml
Test Metrics:
  - Variant performance
  - Conversion rates
  - User engagement
  - Statistical significance
  - Confidence intervals

Business Metrics:
  - Revenue impact
  - User retention
  - Feature adoption
  - Content effectiveness
  - ROI analysis
```

## 🧪 Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_item_management() {
        // Test item creation, retrieval, update
    }
    
    #[tokio::test]
    async fn test_quest_management() {
        // Test quest creation, retrieval, update
    }
    
    #[tokio::test]
    async fn test_localization() {
        // Test localization management
    }
    
    #[tokio::test]
    async fn test_ab_testing() {
        // Test A/B test assignment and tracking
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_complete_content_flow() {
    // Test complete content management flow
    let service = ContentManagementService::new();
    
    // Create item
    let item = service.create_item(ItemRequest {
        item_id: "test_sword",
        name: "Test Sword",
        item_type: "weapon",
        rarity: "rare",
        properties: json!({"damage": 100, "durability": 1000}),
    }).await.unwrap();
    
    // Create quest
    let quest = service.create_quest(QuestRequest {
        quest_id: "test_quest",
        title: "Test Quest",
        quest_type: "main",
        objectives: json!([{"type": "kill", "target": "goblin", "count": 10}]),
        rewards: json!({"experience": 1000, "gold": 100}),
    }).await.unwrap();
    
    // Create localization
    service.create_localization(LocalizationRequest {
        content_id: "test_sword",
        content_type: "item",
        language: "zh-CN",
        field_name: "name",
        translated_text: "测试剑",
    }).await.unwrap();
    
    // Verify content
    let retrieved_item = service.get_item("test_sword").await.unwrap();
    assert_eq!(retrieved_item.name, "Test Sword");
    
    let retrieved_quest = service.get_quest("test_quest").await.unwrap();
    assert_eq!(retrieved_quest.title, "Test Quest");
    
    let localization = service.get_localization("test_sword", "item", "zh-CN").await.unwrap();
    assert_eq!(localization.name, "测试剑");
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
  let response = http.get('http://localhost:8090/items?item_type=weapon');
  check(response, {
    'status is 200': (r) => r.status === 200,
    'response time < 20ms': (r) => r.timings.duration < 20,
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
COPY --from=builder /app/target/release/content-management-service /usr/local/bin/
EXPOSE 8090
CMD ["content-management-service"]
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: content-management-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: content-management-service
  template:
    metadata:
      labels:
        app: content-management-service
    spec:
      containers:
      - name: content-management-service
        image: content-management-service:latest
        ports:
        - containerPort: 8090
        env:
        - name: DATABASE_URL
          value: "postgresql://user:pass@postgres:5432/content_db"
        - name: REDIS_URL
          value: "redis://redis:6379"
        - name: S3_ENDPOINT
          value: "https://s3.amazonaws.com"
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
DATABASE_URL=postgresql://user:pass@localhost:5432/content_db
DATABASE_POOL_SIZE=20
DATABASE_TIMEOUT=30s

# Redis Configuration
REDIS_URL=redis://localhost:6379
REDIS_PASSWORD=secret
REDIS_DB=8

# S3 Configuration
S3_BUCKET=chaos-world-content
S3_ENDPOINT=https://s3.amazonaws.com
S3_ACCESS_KEY=your-access-key
S3_SECRET_KEY=your-secret-key
S3_REGION=us-east-1

# CDN Configuration
CDN_URL=https://cdn.chaos-world.com
CDN_CACHE_TTL=86400

# External Services
CHAOS_BACKEND_URL=grpc://chaos-backend:9090
ANALYTICS_SERVICE_URL=grpc://analytics-service:9097

# Server Configuration
SERVER_PORT=8090
SERVER_HOST=0.0.0.0
SERVER_WORKERS=4

# Content Configuration
CONTENT_CACHE_TTL=86400
MAX_CONTENT_SIZE=10485760
SUPPORTED_LANGUAGES=en,zh-CN,zh-TW,ja,ko
```

### Configuration File
```yaml
# content-management-config.yaml
server:
  port: 8090
  host: "0.0.0.0"
  workers: 4
  max_connections: 10000

database:
  url: "postgresql://user:pass@localhost:5432/content_db"
  pool_size: 20
  timeout: "30s"
  ssl_mode: "prefer"

redis:
  url: "redis://localhost:6379"
  password: "secret"
  db: 8
  pool_size: 100

s3:
  bucket: "chaos-world-content"
  endpoint: "https://s3.amazonaws.com"
  access_key: "your-access-key"
  secret_key: "your-secret-key"
  region: "us-east-1"

cdn:
  url: "https://cdn.chaos-world.com"
  cache_ttl: 86400
  enabled: true

external_services:
  chaos_backend:
    url: "grpc://chaos-backend:9090"
    timeout: "5s"
    retries: 3
  
  analytics_service:
    url: "grpc://analytics-service:9097"
    timeout: "5s"
    retries: 3

content_settings:
  cache_ttl: 86400
  max_content_size: 10485760
  supported_languages: ["en", "zh-CN", "zh-TW", "ja", "ko"]
  default_language: "en"
  version_control_enabled: true
  ab_testing_enabled: true

localization_settings:
  auto_translation: false
  translation_service: "google"
  fallback_language: "en"
  cache_translations: true
  translation_cache_ttl: 86400

ab_testing_settings:
  max_tests_per_user: 10
  test_duration_min: 86400
  test_duration_max: 604800
  min_sample_size: 1000
  confidence_level: 0.95
```

## 🔗 Related Services

- [Chaos Backend](./chaos-backend/README.md) - Game Logic Core
- [Analytics Service](./analytics-service/README.md) - Content analytics
- [Inventory Service](./inventory-service/README.md) - Item usage
- [World Service](./world-service/README.md) - World configuration
- [Microservices Architecture](./microservices-architecture/README.md) - Overall Architecture
