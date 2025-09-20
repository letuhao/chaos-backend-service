# CMS Service API Reference

## Base Information

- **Base URL**: `http://localhost:8080/api/v1`
- **API Version**: v1
- **Content Type**: `application/json`
- **Authentication**: JWT Bearer Token

## Authentication

All API endpoints require authentication using JWT tokens in the Authorization header:

```http
Authorization: Bearer <jwt_token>
```

### Getting a Token

```http
POST /auth/login
Content-Type: application/json

{
  "username": "admin",
  "password": "password"
}
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_in": 3600,
  "user": {
    "id": "user_001",
    "username": "admin",
    "role": "admin"
  }
}
```

## Content Management APIs

### Quests

#### List Quests
```http
GET /quests?page=1&limit=20&status=active&type=main
```

**Query Parameters:**
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 20, max: 100)
- `status` (optional): Filter by status (`active`, `draft`, `archived`)
- `type` (optional): Filter by type (`main`, `side`, `daily`)
- `search` (optional): Search in title and description

**Response:**
```json
{
  "data": [
    {
      "id": "quest_001",
      "title": "The Lost Artifact",
      "description": "Find the ancient artifact hidden in the ruins",
      "type": "main",
      "status": "active",
      "level_required": 10,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 150,
    "pages": 8
  }
}
```

#### Get Quest by ID
```http
GET /quests/{id}
```

**Response:**
```json
{
  "id": "quest_001",
  "title": "The Lost Artifact",
  "description": "Find the ancient artifact hidden in the ruins",
  "type": "main",
  "status": "active",
  "level_required": 10,
  "prerequisites": ["quest_000"],
  "objectives": [
    {
      "id": "obj_001",
      "description": "Find the artifact",
      "type": "collect",
      "target": "ancient_artifact",
      "quantity": 1,
      "optional": false
    }
  ],
  "rewards": {
    "experience": 1000,
    "gold": 500,
    "items": [
      {
        "id": "sword_001",
        "quantity": 1
      }
    ]
  },
  "location": "ruins_ancient",
  "npc_giver": "npc_001",
  "npc_turn_in": "npc_002",
  "time_limit": 3600,
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "version": 1
}
```

#### Create Quest
```http
POST /quests
Content-Type: application/json

{
  "title": "New Quest",
  "description": "Quest description",
  "type": "main",
  "level_required": 10,
  "objectives": [
    {
      "description": "Kill 10 monsters",
      "type": "kill",
      "target": "goblin",
      "quantity": 10
    }
  ],
  "rewards": {
    "experience": 1000,
    "gold": 500
  }
}
```

**Response:**
```json
{
  "id": "quest_002",
  "title": "New Quest",
  "description": "Quest description",
  "type": "main",
  "status": "draft",
  "level_required": 10,
  "objectives": [
    {
      "id": "obj_002",
      "description": "Kill 10 monsters",
      "type": "kill",
      "target": "goblin",
      "quantity": 10,
      "optional": false
    }
  ],
  "rewards": {
    "experience": 1000,
    "gold": 500,
    "items": []
  },
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "version": 1
}
```

#### Update Quest
```http
PUT /quests/{id}
Content-Type: application/json

{
  "title": "Updated Quest Title",
  "description": "Updated description",
  "objectives": [
    {
      "id": "obj_002",
      "description": "Kill 15 monsters",
      "type": "kill",
      "target": "goblin",
      "quantity": 15
    }
  ]
}
```

#### Delete Quest
```http
DELETE /quests/{id}
```

**Response:**
```json
{
  "message": "Quest deleted successfully",
  "id": "quest_002"
}
```

#### Publish Quest
```http
POST /quests/{id}/publish
```

**Response:**
```json
{
  "message": "Quest published successfully",
  "id": "quest_002",
  "status": "active",
  "published_at": "2024-01-01T00:00:00Z"
}
```

### NPCs

#### List NPCs
```http
GET /npcs?page=1&limit=20&type=quest_giver&location=village_center
```

**Query Parameters:**
- `page` (optional): Page number
- `limit` (optional): Items per page
- `type` (optional): Filter by type (`quest_giver`, `merchant`, `guard`, `citizen`)
- `location` (optional): Filter by location
- `search` (optional): Search in name and description

**Response:**
```json
{
  "data": [
    {
      "id": "npc_001",
      "name": "Elder Sage",
      "type": "quest_giver",
      "location": "village_center",
      "level": 50,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 75,
    "pages": 4
  }
}
```

#### Get NPC by ID
```http
GET /npcs/{id}
```

**Response:**
```json
{
  "id": "npc_001",
  "name": "Elder Sage",
  "type": "quest_giver",
  "location": "village_center",
  "description": "A wise elder who gives quests to adventurers",
  "level": 50,
  "stats": {
    "health": 1000,
    "mana": 500,
    "defense": 50,
    "magic_resistance": 75
  },
  "dialogue_tree": {
    "greeting": "Welcome, traveler. I have a task for you.",
    "branches": [
      {
        "id": "branch_001",
        "condition": "quest_001_completed",
        "response": "Thank you for completing the quest! Here is your reward.",
        "actions": ["give_reward", "start_quest_002"]
      },
      {
        "id": "branch_002",
        "condition": "level_10_plus",
        "response": "You look strong enough for this challenge.",
        "actions": ["offer_quest_001"]
      }
    ]
  },
  "inventory": [
    {
      "item_id": "item_001",
      "quantity": 5,
      "price": 100
    }
  ],
  "quests": ["quest_001", "quest_002"],
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "version": 1
}
```

#### Create NPC
```http
POST /npcs
Content-Type: application/json

{
  "name": "Blacksmith",
  "type": "merchant",
  "location": "village_center",
  "description": "A skilled blacksmith who sells weapons and armor",
  "level": 30,
  "stats": {
    "health": 500,
    "mana": 200
  },
  "inventory": [
    {
      "item_id": "sword_001",
      "quantity": 3,
      "price": 150
    }
  ]
}
```

### Items

#### List Items
```http
GET /items?page=1&limit=20&type=weapon&rarity=legendary
```

**Query Parameters:**
- `page` (optional): Page number
- `limit` (optional): Items per page
- `type` (optional): Filter by type (`weapon`, `armor`, `consumable`, `misc`)
- `rarity` (optional): Filter by rarity (`common`, `uncommon`, `rare`, `epic`, `legendary`)
- `level_required` (optional): Filter by minimum level required
- `search` (optional): Search in name and description

**Response:**
```json
{
  "data": [
    {
      "id": "item_001",
      "name": "Ancient Sword",
      "type": "weapon",
      "rarity": "legendary",
      "level_required": 20,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 200,
    "pages": 10
  }
}
```

#### Get Item by ID
```http
GET /items/{id}
```

**Response:**
```json
{
  "id": "item_001",
  "name": "Ancient Sword",
  "type": "weapon",
  "subtype": "sword",
  "rarity": "legendary",
  "level_required": 20,
  "description": "A sword forged in ancient times with magical properties",
  "icon": "sword_ancient.png",
  "stats": {
    "damage": 150,
    "durability": 100,
    "critical_chance": 0.15,
    "critical_damage": 2.0
  },
  "requirements": {
    "strength": 25,
    "dexterity": 15
  },
  "effects": [
    {
      "type": "passive",
      "name": "Ancient Power",
      "description": "Increases damage by 10% against undead"
    }
  ],
  "crafting": {
    "materials": [
      {
        "item_id": "iron_ingot",
        "quantity": 5
      },
      {
        "item_id": "magic_crystal",
        "quantity": 2
      }
    ],
    "skill_required": "blacksmithing",
    "skill_level": 50
  },
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "version": 1
}
```

### Locations

#### List Locations
```http
GET /locations?page=1&limit=20&type=city&region=north
```

**Query Parameters:**
- `page` (optional): Page number
- `limit` (optional): Items per page
- `type` (optional): Filter by type (`city`, `dungeon`, `wilderness`, `dungeon`)
- `region` (optional): Filter by region
- `level_range` (optional): Filter by level range (e.g., `10-20`)
- `search` (optional): Search in name and description

**Response:**
```json
{
  "data": [
    {
      "id": "location_001",
      "name": "Village Center",
      "type": "city",
      "region": "north",
      "level_range": "1-10",
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 50,
    "pages": 3
  }
}
```

#### Get Location by ID
```http
GET /locations/{id}
```

**Response:**
```json
{
  "id": "location_001",
  "name": "Village Center",
  "type": "city",
  "region": "north",
  "level_range": "1-10",
  "description": "The bustling center of the village where adventurers gather",
  "coordinates": {
    "x": 100,
    "y": 200,
    "z": 0
  },
  "map": "village_center.png",
  "npcs": ["npc_001", "npc_002", "npc_003"],
  "shops": ["shop_001", "shop_002"],
  "quests": ["quest_001", "quest_002"],
  "connections": [
    {
      "location_id": "location_002",
      "direction": "north",
      "distance": 100
    }
  ],
  "spawns": [
    {
      "monster_id": "monster_001",
      "level": 5,
      "spawn_rate": 0.1
    }
  ],
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "version": 1
}
```

## Search APIs

### Global Search
```http
GET /search?q=ancient&type=all&limit=20
```

**Query Parameters:**
- `q` (required): Search query
- `type` (optional): Content type (`all`, `quests`, `npcs`, `items`, `locations`)
- `limit` (optional): Maximum results (default: 20, max: 100)

**Response:**
```json
{
  "query": "ancient",
  "results": [
    {
      "type": "quest",
      "id": "quest_001",
      "title": "The Lost Artifact",
      "description": "Find the ancient artifact...",
      "score": 0.95
    },
    {
      "type": "item",
      "id": "item_001",
      "name": "Ancient Sword",
      "description": "A sword forged in ancient times...",
      "score": 0.87
    }
  ],
  "total": 15,
  "took": 45
}
```

### Advanced Search
```http
POST /search/advanced
Content-Type: application/json

{
  "query": "ancient sword",
  "filters": {
    "type": "item",
    "rarity": "legendary",
    "level_required": {
      "min": 10,
      "max": 30
    }
  },
  "sort": "name",
  "order": "asc",
  "limit": 20
}
```

## Version Control APIs

### List Versions
```http
GET /versions?content_type=quest&content_id=quest_001&page=1&limit=20
```

**Query Parameters:**
- `content_type` (optional): Filter by content type
- `content_id` (optional): Filter by content ID
- `page` (optional): Page number
- `limit` (optional): Items per page

**Response:**
```json
{
  "data": [
    {
      "id": "version_001",
      "content_type": "quest",
      "content_id": "quest_001",
      "version": 2,
      "changes": "Updated objectives and rewards",
      "author": "admin",
      "created_at": "2024-01-01T00:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 5,
    "pages": 1
  }
}
```

### Get Version Details
```http
GET /versions/{version_id}
```

**Response:**
```json
{
  "id": "version_001",
  "content_type": "quest",
  "content_id": "quest_001",
  "version": 2,
  "changes": "Updated objectives and rewards",
  "author": "admin",
  "created_at": "2024-01-01T00:00:00Z",
  "data": {
    "id": "quest_001",
    "title": "The Lost Artifact",
    "description": "Find the ancient artifact hidden in the ruins",
    "objectives": [
      {
        "id": "obj_001",
        "description": "Find the artifact",
        "type": "collect",
        "target": "ancient_artifact",
        "quantity": 1
      }
    ]
  }
}
```

### Rollback to Version
```http
POST /versions/{version_id}/rollback
```

**Response:**
```json
{
  "message": "Content rolled back successfully",
  "content_id": "quest_001",
  "version": 2,
  "rolled_back_to": 1
}
```

### Get Version Diff
```http
GET /versions/{version_id}/diff
```

**Response:**
```json
{
  "version_id": "version_001",
  "diff": {
    "title": {
      "old": "The Lost Artifact",
      "new": "The Ancient Artifact"
    },
    "objectives": {
      "added": [
        {
          "description": "Defeat the guardian",
          "type": "kill",
          "target": "guardian",
          "quantity": 1
        }
      ],
      "modified": [
        {
          "id": "obj_001",
          "changes": {
            "quantity": {
              "old": 1,
              "new": 2
            }
          }
        }
      ]
    }
  }
}
```

## File Management APIs

### Upload File
```http
POST /files/upload
Content-Type: multipart/form-data

file: <binary_file_data>
type: image
category: item_icons
```

**Response:**
```json
{
  "id": "file_001",
  "filename": "sword_ancient.png",
  "original_name": "ancient_sword.png",
  "type": "image",
  "category": "item_icons",
  "size": 1024000,
  "url": "/files/file_001",
  "created_at": "2024-01-01T00:00:00Z"
}
```

### Get File
```http
GET /files/{file_id}
```

**Response:** Binary file data with appropriate Content-Type header

### List Files
```http
GET /files?type=image&category=item_icons&page=1&limit=20
```

**Response:**
```json
{
  "data": [
    {
      "id": "file_001",
      "filename": "sword_ancient.png",
      "type": "image",
      "category": "item_icons",
      "size": 1024000,
      "url": "/files/file_001",
      "created_at": "2024-01-01T00:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 50,
    "pages": 3
  }
}
```

## Bulk Operations APIs

### Bulk Import
```http
POST /bulk/import
Content-Type: application/json

{
  "type": "quests",
  "data": [
    {
      "title": "Quest 1",
      "description": "Description 1"
    },
    {
      "title": "Quest 2",
      "description": "Description 2"
    }
  ]
}
```

**Response:**
```json
{
  "job_id": "job_001",
  "status": "processing",
  "total_items": 2,
  "processed": 0,
  "errors": []
}
```

### Get Import Status
```http
GET /bulk/import/{job_id}
```

**Response:**
```json
{
  "job_id": "job_001",
  "status": "completed",
  "total_items": 2,
  "processed": 2,
  "successful": 2,
  "failed": 0,
  "errors": []
}
```

### Bulk Export
```http
POST /bulk/export
Content-Type: application/json

{
  "type": "quests",
  "filters": {
    "status": "active"
  },
  "format": "json"
}
```

**Response:**
```json
{
  "job_id": "job_002",
  "status": "processing",
  "download_url": "/bulk/export/job_002/download"
}
```

## Health and Monitoring APIs

### Health Check
```http
GET /health
```

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T00:00:00Z",
  "version": "1.0.0"
}
```

### Detailed Health Check
```http
GET /health/detailed
```

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T00:00:00Z",
  "version": "1.0.0",
  "services": {
    "database": {
      "status": "healthy",
      "response_time": 15
    },
    "cache": {
      "status": "healthy",
      "response_time": 5
    },
    "storage": {
      "status": "healthy",
      "free_space": "50GB"
    }
  }
}
```

### Metrics
```http
GET /metrics
```

**Response:**
```json
{
  "requests_total": 1500,
  "requests_per_second": 25.5,
  "response_time_avg": 120,
  "response_time_p95": 250,
  "error_rate": 0.02,
  "cache_hit_rate": 0.85,
  "database_connections": 10,
  "memory_usage": "256MB",
  "cpu_usage": 15.5
}
```

## Error Responses

All error responses follow this format:

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid input data",
    "details": {
      "field": "title",
      "reason": "Title is required"
    }
  },
  "timestamp": "2024-01-01T00:00:00Z",
  "request_id": "req_001"
}
```

### Common Error Codes

- `VALIDATION_ERROR`: Input validation failed
- `NOT_FOUND`: Resource not found
- `UNAUTHORIZED`: Authentication required
- `FORBIDDEN`: Insufficient permissions
- `CONFLICT`: Resource conflict (e.g., duplicate ID)
- `RATE_LIMITED`: Too many requests
- `INTERNAL_ERROR`: Server error

## Rate Limiting

API requests are rate limited per user:

- **Authenticated users**: 1000 requests per hour
- **Anonymous users**: 100 requests per hour
- **Bulk operations**: 10 requests per hour

Rate limit headers are included in responses:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1640995200
```

## Pagination

All list endpoints support pagination:

- `page`: Page number (1-based)
- `limit`: Items per page (max 100)
- `total`: Total number of items
- `pages`: Total number of pages

## Sorting

Most list endpoints support sorting:

- `sort`: Field to sort by
- `order`: Sort order (`asc` or `desc`)

Example:
```http
GET /quests?sort=created_at&order=desc
```

## Filtering

Many endpoints support filtering using query parameters:

```http
GET /quests?status=active&type=main&level_required=10
```

## Field Selection

Use the `fields` parameter to select specific fields:

```http
GET /quests?fields=id,title,status
```

## Response Format

All successful responses use this format:

```json
{
  "data": <response_data>,
  "pagination": <pagination_info>, // for list endpoints
  "meta": <additional_metadata> // optional
}
```
