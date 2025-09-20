# API Gateway Endpoints Specification

## Overview
This document defines all API endpoints exposed by the Chaos World API Gateway, including authentication, routing, and service integration patterns.

## Base URL
- **Development**: `http://localhost:8080`
- **Staging**: `https://api-staging.chaosworld.com`
- **Production**: `https://api.chaosworld.com`

## API Versioning
- **Current Version**: v1
- **Version Header**: `API-Version: v1`
- **URL Pattern**: `/api/v1/{service}/{endpoint}`

## Authentication

### Authentication Methods
1. **JWT Bearer Token** (Primary)
2. **API Key** (Service-to-service)
3. **OAuth 2.0** (Third-party integration)

### Authentication Headers
```http
Authorization: Bearer <jwt_token>
X-API-Key: <api_key>
X-Client-ID: <client_id>
```

## Core Endpoints

### 1. **Health & Status**

#### Health Check
```http
GET /health
```
**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T00:00:00Z",
  "version": "1.0.0",
  "services": {
    "user-management": "healthy",
    "chaos-backend": "healthy",
    "inventory-service": "healthy"
  }
}
```

#### Service Status
```http
GET /status
```
**Response:**
```json
{
  "gateway": {
    "status": "running",
    "uptime": "72h15m30s",
    "requests_processed": 1234567
  },
  "services": [
    {
      "name": "user-management",
      "status": "healthy",
      "response_time": "5ms",
      "last_check": "2024-01-01T00:00:00Z"
    }
  ]
}
```

### 2. **User Management Endpoints**

#### User Registration
```http
POST /api/v1/users/register
Content-Type: application/json

{
  "username": "player123",
  "email": "player@example.com",
  "password": "secure_password",
  "display_name": "Player One"
}
```

#### User Login
```http
POST /api/v1/users/login
Content-Type: application/json

{
  "username": "player123",
  "password": "secure_password"
}
```

#### User Profile
```http
GET /api/v1/users/profile
Authorization: Bearer <jwt_token>
```

#### Update Profile
```http
PUT /api/v1/users/profile
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "display_name": "New Display Name",
  "email": "newemail@example.com"
}
```

### 3. **Game Logic Endpoints**

#### Get Actor Data
```http
GET /api/v1/game/actors/{actor_id}
Authorization: Bearer <jwt_token>
```

#### Update Actor Stats
```http
PUT /api/v1/game/actors/{actor_id}/stats
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "health": 100,
  "mana": 50,
  "experience": 1250
}
```

#### Perform Action
```http
POST /api/v1/game/actions
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "actor_id": "actor_123",
  "action_type": "cast_spell",
  "target_id": "target_456",
  "parameters": {
    "spell_id": "fireball",
    "power_level": 5
  }
}
```

### 4. **Inventory Management**

#### Get Inventory
```http
GET /api/v1/inventory
Authorization: Bearer <jwt_token>
```

#### Add Item
```http
POST /api/v1/inventory/items
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "item_id": "sword_001",
  "quantity": 1,
  "properties": {
    "durability": 100,
    "enchantment": "fire"
  }
}
```

#### Use Item
```http
POST /api/v1/inventory/items/{item_id}/use
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "target_id": "actor_123",
  "parameters": {}
}
```

### 5. **Chat System**

#### Send Message
```http
POST /api/v1/chat/messages
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "channel": "global",
  "message": "Hello world!",
  "type": "text"
}
```

#### Get Channel History
```http
GET /api/v1/chat/channels/{channel_id}/messages
Authorization: Bearer <jwt_token>
Query Parameters:
  - limit: 50
  - offset: 0
  - since: 2024-01-01T00:00:00Z
```

#### Join Channel
```http
POST /api/v1/chat/channels/{channel_id}/join
Authorization: Bearer <jwt_token>
```

### 6. **Guild Management**

#### Create Guild
```http
POST /api/v1/guilds
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "name": "Dragon Slayers",
  "description": "Elite guild for dragon hunting",
  "recruitment_open": true
}
```

#### Get Guild Info
```http
GET /api/v1/guilds/{guild_id}
Authorization: Bearer <jwt_token>
```

#### Join Guild
```http
POST /api/v1/guilds/{guild_id}/members
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "message": "I would like to join your guild"
}
```

### 7. **World & Environment**

#### Get World State
```http
GET /api/v1/world/state
Authorization: Bearer <jwt_token>
Query Parameters:
  - region: "forest"
  - include_players: true
```

#### Get Weather
```http
GET /api/v1/world/weather
Authorization: Bearer <jwt_token>
Query Parameters:
  - location: "forest_001"
```

### 8. **Matchmaking**

#### Join Queue
```http
POST /api/v1/matchmaking/queue
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "queue_type": "pvp",
  "preferences": {
    "min_players": 2,
    "max_players": 4,
    "skill_level": "intermediate"
  }
}
```

#### Get Queue Status
```http
GET /api/v1/matchmaking/queue/status
Authorization: Bearer <jwt_token>
```

#### Leave Queue
```http
DELETE /api/v1/matchmaking/queue
Authorization: Bearer <jwt_token>
```

### 9. **Events & Activities**

#### Get Active Events
```http
GET /api/v1/events
Authorization: Bearer <jwt_token>
Query Parameters:
  - type: "daily"
  - status: "active"
```

#### Join Event
```http
POST /api/v1/events/{event_id}/join
Authorization: Bearer <jwt_token>
```

#### Get Event Progress
```http
GET /api/v1/events/{event_id}/progress
Authorization: Bearer <jwt_token>
```

### 10. **Notifications**

#### Get Notifications
```http
GET /api/v1/notifications
Authorization: Bearer <jwt_token>
Query Parameters:
  - unread_only: true
  - limit: 20
```

#### Mark as Read
```http
PUT /api/v1/notifications/{notification_id}/read
Authorization: Bearer <jwt_token>
```

### 11. **Payment & Transactions**

#### Get Payment Methods
```http
GET /api/v1/payments/methods
Authorization: Bearer <jwt_token>
```

#### Create Payment
```http
POST /api/v1/payments
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "amount": 9.99,
  "currency": "USD",
  "item_id": "premium_package",
  "payment_method": "credit_card"
}
```

#### Get Transaction History
```http
GET /api/v1/payments/transactions
Authorization: Bearer <jwt_token>
Query Parameters:
  - limit: 50
  - offset: 0
  - status: "completed"
```

### 12. **Analytics & Statistics**

#### Get Player Stats
```http
GET /api/v1/analytics/player/{player_id}/stats
Authorization: Bearer <jwt_token>
Query Parameters:
  - period: "30d"
  - metrics: "kills,deaths,experience"
```

#### Get Game Metrics
```http
GET /api/v1/analytics/game/metrics
Authorization: Bearer <jwt_token>
Query Parameters:
  - metric: "active_players"
  - period: "1h"
```

## Error Responses

### Standard Error Format
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid input parameters",
    "details": {
      "field": "email",
      "reason": "Invalid email format"
    },
    "timestamp": "2024-01-01T00:00:00Z",
    "request_id": "req_123456789"
  }
}
```

### Common Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `VALIDATION_ERROR` | 400 | Invalid request parameters |
| `UNAUTHORIZED` | 401 | Authentication required |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `NOT_FOUND` | 404 | Resource not found |
| `RATE_LIMITED` | 429 | Too many requests |
| `SERVICE_UNAVAILABLE` | 503 | Service temporarily unavailable |
| `INTERNAL_ERROR` | 500 | Internal server error |

## Rate Limiting

### Rate Limit Headers
```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1640995200
```

### Rate Limit Tiers

| Tier | Requests/Minute | Burst |
|------|----------------|-------|
| **Free** | 100 | 200 |
| **Premium** | 1000 | 2000 |
| **VIP** | 10000 | 20000 |
| **Service** | 100000 | 200000 |

## WebSocket Endpoints

### Real-time Chat
```javascript
ws://api.chaosworld.com/ws/chat
```

### Game Events
```javascript
ws://api.chaosworld.com/ws/game/{actor_id}
```

### Notifications
```javascript
ws://api.chaosworld.com/ws/notifications
```

## SDK Examples

### JavaScript/TypeScript
```typescript
import { ChaosWorldAPI } from '@chaosworld/api-client';

const api = new ChaosWorldAPI({
  baseURL: 'https://api.chaosworld.com',
  apiKey: 'your-api-key'
});

// Login
const user = await api.users.login({
  username: 'player123',
  password: 'secure_password'
});

// Get actor data
const actor = await api.game.getActor('actor_123');
```

### Rust
```rust
use chaos_world_api::ChaosWorldClient;

let client = ChaosWorldClient::new("https://api.chaosworld.com")
    .with_auth("your-api-key");

// Login
let user = client.users().login(LoginRequest {
    username: "player123".to_string(),
    password: "secure_password".to_string(),
}).await?;

// Get actor data
let actor = client.game().get_actor("actor_123").await?;
```

## Testing

### Postman Collection
- Import URL: `https://api.chaosworld.com/docs/postman/collection.json`
- Environment: `https://api.chaosworld.com/docs/postman/environment.json`

### OpenAPI Specification
- Swagger UI: `https://api.chaosworld.com/docs`
- OpenAPI JSON: `https://api.chaosworld.com/docs/openapi.json`

## Changelog

### v1.0.0 (2024-01-01)
- Initial API release
- Core endpoints implementation
- JWT authentication
- Rate limiting
- Basic error handling

### v1.1.0 (2024-01-15)
- WebSocket support
- Real-time chat
- Enhanced error responses
- SDK improvements

### v1.2.0 (2024-02-01)
- OAuth 2.0 integration
- Advanced rate limiting
- Caching improvements
- Performance optimizations
