# Inventory Service - Chaos World

## 📋 Overview

Inventory Service chịu trách nhiệm quản lý tất cả items, inventory operations, trading system, và storage management cho Chaos World MMORPG.

## 🎯 Responsibilities

### Core Functions
- **Item Management**: Quản lý item definitions và properties
- **Inventory Operations**: Add, remove, move, stack items
- **Trading System**: Player-to-player trading
- **Storage Management**: Bank, warehouse, guild storage
- **Item Crafting**: Crafting recipes và materials
- **Item Enchanting**: Item enhancement và upgrades

### Performance Requirements
- **Latency**: < 50ms cho inventory operations
- **Throughput**: 10,000+ operations/second
- **Availability**: 99.9% uptime
- **Concurrency**: Handle 50,000+ concurrent operations

## 🏗️ Architecture

### Technology Stack
```yaml
Language: Rust
Framework: Axum
Database: PostgreSQL (ACID compliance)
Caching: Redis
Message Queue: Apache Kafka
Search: Elasticsearch (item search)
```

### Core Components
```rust
pub struct InventoryService {
    // Database
    item_repository: Arc<ItemRepository>,
    inventory_repository: Arc<InventoryRepository>,
    trade_repository: Arc<TradeRepository>,
    storage_repository: Arc<StorageRepository>,
    
    // Item Management
    item_manager: Arc<ItemManager>,
    crafting_manager: Arc<CraftingManager>,
    enchanting_manager: Arc<EnchantingManager>,
    
    // Trading
    trade_manager: Arc<TradeManager>,
    auction_house: Arc<AuctionHouse>,
    
    // Storage
    bank_manager: Arc<BankManager>,
    warehouse_manager: Arc<WarehouseManager>,
    guild_storage: Arc<GuildStorage>,
    
    // External Services
    chaos_backend_client: Arc<ChaosBackendClient>,
    payment_service_client: Arc<PaymentServiceClient>,
    
    // Configuration
    config: InventoryConfig,
}
```

## 🎒 Item System

### Item Types
```yaml
Equipment:
  - Weapons (swords, bows, staffs)
  - Armor (helmet, chest, legs, boots)
  - Accessories (rings, necklaces, bracelets)
  - Shields

Consumables:
  - Potions (health, mana, stamina)
  - Food (buffs, healing)
  - Scrolls (spells, teleportation)
  - Books (skills, knowledge)

Materials:
  - Ores (iron, gold, mithril)
  - Gems (ruby, sapphire, diamond)
  - Herbs (healing, magical)
  - Cloth (silk, cotton, leather)

Special:
  - Keys (doors, chests)
  - Tools (mining, crafting)
  - Tokens (currency, rewards)
  - Quest Items
```

### Item Properties
```yaml
Basic Properties:
  - Item ID
  - Name
  - Description
  - Type
  - Rarity (common, uncommon, rare, epic, legendary)
  - Level requirement
  - Value

Equipment Properties:
  - Stats (strength, agility, intelligence)
  - Durability
  - Enhancement level
  - Sockets
  - Set bonuses

Consumable Properties:
  - Effect type
  - Effect value
  - Duration
  - Cooldown
  - Stack size

Material Properties:
  - Quality
  - Purity
  - Crafting uses
  - Enhancement potential
```

## 🗄️ Database Schema

### PostgreSQL Tables
```sql
-- Items table
CREATE TABLE items (
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
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Player inventories table
CREATE TABLE player_inventories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    slot_position INTEGER NOT NULL,
    item_id VARCHAR(50) NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    durability INTEGER,
    enhancement_level INTEGER DEFAULT 0,
    properties JSONB,
    acquired_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, slot_position)
);

-- Trading table
CREATE TABLE trades (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    initiator_id UUID NOT NULL,
    target_id UUID NOT NULL,
    status VARCHAR(20) DEFAULT 'pending', -- 'pending', 'accepted', 'completed', 'cancelled'
    initiator_items JSONB NOT NULL,
    target_items JSONB NOT NULL,
    gold_amount INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    completed_at TIMESTAMP
);

-- Storage table
CREATE TABLE storage (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    storage_type VARCHAR(20) NOT NULL, -- 'bank', 'warehouse', 'guild'
    slot_position INTEGER NOT NULL,
    item_id VARCHAR(50) NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    durability INTEGER,
    enhancement_level INTEGER DEFAULT 0,
    properties JSONB,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, storage_type, slot_position)
);

-- Crafting recipes table
CREATE TABLE crafting_recipes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    recipe_id VARCHAR(50) UNIQUE NOT NULL,
    result_item_id VARCHAR(50) NOT NULL,
    result_quantity INTEGER DEFAULT 1,
    required_items JSONB NOT NULL, -- {item_id: quantity}
    required_level INTEGER DEFAULT 1,
    required_skill VARCHAR(50),
    crafting_time INTEGER DEFAULT 0, -- seconds
    success_rate DECIMAL(5,2) DEFAULT 100.00,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Item transactions table
CREATE TABLE item_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    transaction_type VARCHAR(20) NOT NULL, -- 'acquired', 'sold', 'traded', 'crafted', 'enchanted'
    item_id VARCHAR(50) NOT NULL,
    quantity INTEGER NOT NULL,
    value INTEGER DEFAULT 0,
    source VARCHAR(50), -- 'drop', 'quest', 'purchase', 'trade', 'craft'
    metadata JSONB,
    created_at TIMESTAMP DEFAULT NOW()
);
```

### Redis Cache Structure
```yaml
Player Inventory:
  key: "inventory:{user_id}"
  value: { slots: [...], total_slots: 100, used_slots: 50 }
  ttl: 3600 seconds

Item Cache:
  key: "item:{item_id}"
  value: { name, type, rarity, properties, ... }
  ttl: 86400 seconds

Trading Cache:
  key: "trade:{trade_id}"
  value: { initiator, target, items, status, ... }
  ttl: 1800 seconds

Storage Cache:
  key: "storage:{user_id}:{type}"
  value: { slots: [...], total_slots: 200, used_slots: 100 }
  ttl: 3600 seconds
```

## 🔌 API Endpoints

### Inventory Endpoints
```yaml
GET /inventory/{user_id}:
  Description: Get player inventory
  Request: { user_id }
  Response: { 
    slots: [...], 
    total_slots, 
    used_slots, 
    gold 
  }
  Rate Limit: 1000/hour per user

POST /inventory/add-item:
  Description: Add item to inventory
  Request: { 
    user_id, 
    item_id, 
    quantity, 
    properties 
  }
  Response: { 
    success: true, 
    slot_position, 
    remaining_quantity 
  }
  Rate Limit: 100/minute per user

POST /inventory/remove-item:
  Description: Remove item from inventory
  Request: { 
    user_id, 
    slot_position, 
    quantity 
  }
  Response: { 
    success: true, 
    removed_item, 
    remaining_quantity 
  }
  Rate Limit: 100/minute per user

POST /inventory/move-item:
  Description: Move item between slots
  Request: { 
    user_id, 
    from_slot, 
    to_slot 
  }
  Response: { 
    success: true, 
    moved_item 
  }
  Rate Limit: 100/minute per user
```

### Trading Endpoints
```yaml
POST /trades/create:
  Description: Create trade offer
  Request: { 
    initiator_id, 
    target_id, 
    initiator_items, 
    target_items, 
    gold_amount 
  }
  Response: { 
    trade_id, 
    status: 'pending' 
  }
  Rate Limit: 10/minute per user

POST /trades/{trade_id}/accept:
  Description: Accept trade offer
  Request: { 
    trade_id, 
    user_id 
  }
  Response: { 
    success: true, 
    status: 'accepted' 
  }
  Rate Limit: 10/minute per user

POST /trades/{trade_id}/cancel:
  Description: Cancel trade offer
  Request: { 
    trade_id, 
    user_id 
  }
  Response: { 
    success: true, 
    status: 'cancelled' 
  }
  Rate Limit: 10/minute per user

GET /trades/{user_id}/active:
  Description: Get active trades for user
  Request: { user_id }
  Response: { 
    trades: [...], 
    total 
  }
  Rate Limit: 1000/hour per user
```

### Storage Endpoints
```yaml
GET /storage/{user_id}/{type}:
  Description: Get storage contents
  Request: { user_id, type }
  Response: { 
    slots: [...], 
    total_slots, 
    used_slots 
  }
  Rate Limit: 1000/hour per user

POST /storage/deposit:
  Description: Deposit item to storage
  Request: { 
    user_id, 
    storage_type, 
    item_id, 
    quantity, 
    slot_position 
  }
  Response: { 
    success: true, 
    slot_position 
  }
  Rate Limit: 100/minute per user

POST /storage/withdraw:
  Description: Withdraw item from storage
  Request: { 
    user_id, 
    storage_type, 
    slot_position, 
    quantity 
  }
  Response: { 
    success: true, 
    withdrawn_item 
  }
  Rate Limit: 100/minute per user
```

### Crafting Endpoints
```yaml
GET /crafting/recipes:
  Description: Get available crafting recipes
  Request: { user_id, skill_level, item_type }
  Response: { 
    recipes: [...], 
    total 
  }
  Rate Limit: 1000/hour per user

POST /crafting/craft:
  Description: Craft item
  Request: { 
    user_id, 
    recipe_id, 
    quantity 
  }
  Response: { 
    success: true, 
    crafted_item, 
    materials_used 
  }
  Rate Limit: 10/minute per user

GET /crafting/materials/{user_id}:
  Description: Get available materials
  Request: { user_id }
  Response: { 
    materials: [...], 
    total 
  }
  Rate Limit: 1000/hour per user
```

## 🛠️ Item Operations

### Inventory Management
```yaml
Add Item:
  - Check inventory space
  - Validate item properties
  - Handle stacking
  - Update inventory slots
  - Log transaction

Remove Item:
  - Validate ownership
  - Check quantity
  - Remove from slot
  - Update inventory
  - Log transaction

Move Item:
  - Validate slots
  - Check compatibility
  - Swap items if needed
  - Update both slots
  - Log transaction

Stack Items:
  - Find existing stack
  - Calculate new quantity
  - Update stack
  - Handle overflow
  - Log transaction
```

### Trading System
```yaml
Create Trade:
  - Validate both players
  - Check item ownership
  - Reserve items
  - Create trade record
  - Notify target player

Accept Trade:
  - Validate trade status
  - Check item availability
  - Transfer items
  - Complete trade
  - Notify both players

Cancel Trade:
  - Validate ownership
  - Release reserved items
  - Update trade status
  - Notify other player
  - Clean up trade
```

### Storage Management
```yaml
Bank Storage:
  - Unlimited slots
  - Gold storage
  - Item security
  - Access control
  - Transaction logging

Warehouse Storage:
  - Limited slots
  - Item organization
  - Search functionality
  - Bulk operations
  - Access permissions

Guild Storage:
  - Guild-only access
  - Permission levels
  - Contribution tracking
  - Audit logging
  - Resource sharing
```

## 📊 Monitoring & Analytics

### Inventory Metrics
```yaml
Usage Metrics:
  - Items per player
  - Inventory utilization
  - Most popular items
  - Trading volume
  - Storage usage

Performance Metrics:
  - Operation latency
  - Database query time
  - Cache hit rate
  - Error rates
  - Throughput

Business Metrics:
  - Item economy
  - Trading patterns
  - Crafting activity
  - Storage growth
  - Player engagement
```

### Real-time Dashboards
```yaml
Live Monitoring:
  - Active trades
  - Inventory operations
  - Storage usage
  - Error rates
  - System health

Historical Analysis:
  - Item trends
  - Trading patterns
  - Player behavior
  - Economy analysis
  - Performance trends
```

## 🧪 Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_inventory_operations() {
        // Test add, remove, move items
    }
    
    #[tokio::test]
    async fn test_trading_system() {
        // Test trade creation, acceptance, cancellation
    }
    
    #[tokio::test]
    async fn test_storage_management() {
        // Test deposit, withdraw, organization
    }
    
    #[tokio::test]
    async fn test_crafting_system() {
        // Test recipe validation, crafting process
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_complete_trading_flow() {
    // Test complete trading flow
    let service = InventoryService::new();
    
    // Create trade
    let trade = TradeRequest {
        initiator_id: "player_1",
        target_id: "player_2",
        initiator_items: vec![ItemOffer {
            item_id: "sword_001",
            quantity: 1,
        }],
        target_items: vec![ItemOffer {
            item_id: "shield_001",
            quantity: 1,
        }],
        gold_amount: 1000,
    };
    
    let result = service.create_trade(trade).await;
    assert!(result.is_ok());
    
    // Accept trade
    let accept_result = service.accept_trade(&result.unwrap().trade_id, "player_2").await;
    assert!(accept_result.is_ok());
    
    // Verify items transferred
    let player1_items = service.get_inventory("player_1").await.unwrap();
    let player2_items = service.get_inventory("player_2").await.unwrap();
    
    assert!(player1_items.contains_item("shield_001"));
    assert!(player2_items.contains_item("sword_001"));
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
  let response = http.get('http://localhost:8086/inventory/test_user');
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
COPY --from=builder /app/target/release/inventory-service /usr/local/bin/
EXPOSE 8086
CMD ["inventory-service"]
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: inventory-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: inventory-service
  template:
    metadata:
      labels:
        app: inventory-service
    spec:
      containers:
      - name: inventory-service
        image: inventory-service:latest
        ports:
        - containerPort: 8086
        env:
        - name: DATABASE_URL
          value: "postgresql://user:pass@postgres:5432/inventory_db"
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
DATABASE_URL=postgresql://user:pass@localhost:5432/inventory_db
DATABASE_POOL_SIZE=20
DATABASE_TIMEOUT=30s

# Redis Configuration
REDIS_URL=redis://localhost:6379
REDIS_PASSWORD=secret
REDIS_DB=4

# Kafka Configuration
KAFKA_BROKERS=localhost:9092
KAFKA_TOPIC_PREFIX=inventory
KAFKA_GROUP_ID=inventory_service

# External Services
CHAOS_BACKEND_URL=grpc://chaos-backend:9090
PAYMENT_SERVICE_URL=grpc://payment-service:9092

# Server Configuration
SERVER_PORT=8086
SERVER_HOST=0.0.0.0
SERVER_WORKERS=4

# Inventory Configuration
MAX_INVENTORY_SLOTS=100
MAX_BANK_SLOTS=200
MAX_WAREHOUSE_SLOTS=500
MAX_GUILD_STORAGE_SLOTS=1000
```

### Configuration File
```yaml
# inventory-config.yaml
server:
  port: 8086
  host: "0.0.0.0"
  workers: 4
  max_connections: 10000

database:
  url: "postgresql://user:pass@localhost:5432/inventory_db"
  pool_size: 20
  timeout: "30s"
  ssl_mode: "prefer"

redis:
  url: "redis://localhost:6379"
  password: "secret"
  db: 4
  pool_size: 100

kafka:
  brokers: ["localhost:9092"]
  topic_prefix: "inventory"
  group_id: "inventory_service"
  auto_offset_reset: "latest"

external_services:
  chaos_backend:
    url: "grpc://chaos-backend:9090"
    timeout: "5s"
    retries: 3
  
  payment_service:
    url: "grpc://payment-service:9092"
    timeout: "10s"
    retries: 3

inventory_settings:
  max_inventory_slots: 100
  max_bank_slots: 200
  max_warehouse_slots: 500
  max_guild_storage_slots: 1000
  max_stack_size: 999
  trading_timeout: 300
  storage_access_levels: ["owner", "guild_member", "guild_officer", "guild_leader"]

crafting_settings:
  max_crafting_level: 100
  crafting_time_multiplier: 1.0
  success_rate_bonus: 0.0
  material_return_rate: 0.1
  experience_gain_multiplier: 1.0
```

## 🔗 Related Services

- [Chaos Backend](./chaos-backend/README.md) - Game Logic Core
- [Payment Service](./payment-service/README.md) - Transaction processing
- [User Management](./user-management/README.md) - User authentication
- [Guild Service](./guild-service/README.md) - Guild storage
- [Microservices Architecture](./microservices-architecture/README.md) - Overall Architecture
