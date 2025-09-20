# Content Management Service (CMS)

## Overview

The Content Management Service (CMS) is a microservice responsible for managing all game content including quests, NPCs, items, locations, dialogues, and other dynamic content in Chaos World. It provides a centralized content management system that allows game designers and administrators to create, update, and manage game content without requiring code deployments.

## Table of Contents

- [Architecture](#architecture)
- [Features](#features)
- [API Reference](#api-reference)
- [Data Models](#data-models)
- [Configuration](#configuration)
- [Deployment](#deployment)
- [Integration](#integration)
- [Development](#development)
- [Testing](#testing)

## Architecture

The CMS service follows a microservices architecture pattern with the following components:

```
┌─────────────────────────────────────────────────────────────┐
│                    CMS Service Architecture                 │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   API       │  │  Content    │  │  Version    │        │
│  │  Gateway    │  │  Manager    │  │  Control    │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
│         │                │                │               │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │  Content    │  │  Search     │  │  Cache      │        │
│  │  Storage    │  │  Engine     │  │  Layer      │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
│         │                │                │               │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │  MongoDB    │  │  Redis      │  │  File       │        │
│  │  Database   │  │  Cache      │  │  Storage    │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

### Core Components

1. **API Gateway**: RESTful API endpoints for content management
2. **Content Manager**: Core business logic for content operations
3. **Version Control**: Content versioning and rollback capabilities
4. **Search Engine**: Full-text search and content discovery
5. **Cache Layer**: High-performance content caching
6. **Storage Layer**: Persistent storage with MongoDB and file system

## Features

### Content Management
- **Quest Management**: Create, update, and manage quests with complex branching logic
- **NPC Management**: Define NPCs with dialogues, behaviors, and interactions
- **Item Management**: Manage game items with properties, stats, and metadata
- **Location Management**: Define game locations, maps, and world areas
- **Dialogue System**: Rich dialogue trees with conditions and responses
- **Event Management**: Game events, triggers, and story progression

### Content Operations
- **CRUD Operations**: Full Create, Read, Update, Delete operations for all content types
- **Bulk Operations**: Batch import/export of content
- **Content Validation**: Schema validation and content integrity checks
- **Dependency Management**: Track content dependencies and relationships
- **Content Publishing**: Staged content publishing with approval workflows

### Advanced Features
- **Version Control**: Git-like versioning for content changes
- **Content Templates**: Reusable content templates and blueprints
- **Localization**: Multi-language content support
- **Content Scheduling**: Time-based content publishing
- **A/B Testing**: Content experimentation and testing
- **Analytics**: Content usage and performance metrics

## API Reference

### Base URL
```
http://localhost:8080/api/v1
```

### Authentication
All API endpoints require authentication using JWT tokens:
```
Authorization: Bearer <jwt_token>
```

### Content Endpoints

#### Quests
- `GET /quests` - List all quests
- `GET /quests/{id}` - Get quest by ID
- `POST /quests` - Create new quest
- `PUT /quests/{id}` - Update quest
- `DELETE /quests/{id}` - Delete quest
- `POST /quests/{id}/publish` - Publish quest

#### NPCs
- `GET /npcs` - List all NPCs
- `GET /npcs/{id}` - Get NPC by ID
- `POST /npcs` - Create new NPC
- `PUT /npcs/{id}` - Update NPC
- `DELETE /npcs/{id}` - Delete NPC

#### Items
- `GET /items` - List all items
- `GET /items/{id}` - Get item by ID
- `POST /items` - Create new item
- `PUT /items/{id}` - Update item
- `DELETE /items/{id}` - Delete item

#### Locations
- `GET /locations` - List all locations
- `GET /locations/{id}` - Get location by ID
- `POST /locations` - Create new location
- `PUT /locations/{id}` - Update location
- `DELETE /locations/{id}` - Delete location

### Search Endpoints
- `GET /search?q={query}` - Search content
- `GET /search/quests?q={query}` - Search quests
- `GET /search/npcs?q={query}` - Search NPCs
- `GET /search/items?q={query}` - Search items

### Version Control Endpoints
- `GET /versions` - List content versions
- `GET /versions/{id}` - Get specific version
- `POST /versions/{id}/rollback` - Rollback to version
- `GET /versions/{id}/diff` - Get version diff

## Data Models

### Quest Model
```json
{
  "id": "quest_001",
  "title": "The Lost Artifact",
  "description": "Find the ancient artifact hidden in the ruins",
  "type": "main",
  "status": "active",
  "prerequisites": ["quest_000"],
  "objectives": [
    {
      "id": "obj_001",
      "description": "Find the artifact",
      "type": "collect",
      "target": "ancient_artifact",
      "quantity": 1
    }
  ],
  "rewards": {
    "experience": 1000,
    "items": ["sword_001", "gold_500"]
  },
  "location": "ruins_ancient",
  "npc_giver": "npc_001",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "version": 1
}
```

### NPC Model
```json
{
  "id": "npc_001",
  "name": "Elder Sage",
  "type": "quest_giver",
  "location": "village_center",
  "dialogue_tree": {
    "greeting": "Welcome, traveler. I have a task for you.",
    "branches": [
      {
        "condition": "quest_001_completed",
        "response": "Thank you for completing the quest!"
      }
    ]
  },
  "inventory": ["item_001", "item_002"],
  "stats": {
    "health": 100,
    "level": 50
  },
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "version": 1
}
```

### Item Model
```json
{
  "id": "item_001",
  "name": "Ancient Sword",
  "type": "weapon",
  "rarity": "legendary",
  "stats": {
    "damage": 150,
    "durability": 100,
    "level_required": 20
  },
  "description": "A sword forged in ancient times",
  "icon": "sword_ancient.png",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "version": 1
}
```

## Configuration

### Environment Variables
```bash
# Server Configuration
CMS_PORT=8080
CMS_HOST=0.0.0.0

# Database Configuration
MONGODB_URI=mongodb://localhost:27017
MONGODB_DATABASE=chaos_cms

# Cache Configuration
REDIS_URL=redis://localhost:6379
CACHE_TTL=3600

# Authentication
JWT_SECRET=your_jwt_secret_here

# File Storage
FILE_STORAGE_PATH=/app/storage
MAX_FILE_SIZE=10485760  # 10MB

# Logging
LOG_LEVEL=info
```

### Configuration File
```yaml
# configs/cms_config.yaml
server:
  port: 8080
  host: "0.0.0.0"
  workers: 4

database:
  mongodb:
    uri: "mongodb://localhost:27017"
    database: "chaos_cms"
    collections:
      quests: "quests"
      npcs: "npcs"
      items: "items"
      locations: "locations"

cache:
  redis:
    url: "redis://localhost:6379"
    ttl: 3600
    max_connections: 100

storage:
  file_path: "/app/storage"
  max_file_size: 10485760
  allowed_types: ["png", "jpg", "jpeg", "gif", "mp3", "wav"]

security:
  jwt_secret: "your_jwt_secret_here"
  token_expiry: 3600

logging:
  level: "info"
  format: "json"
```

## Deployment

### Docker Deployment
```dockerfile
FROM rust:1.70-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/content-management-service /usr/local/bin/
EXPOSE 8080
CMD ["content-management-service"]
```

### Docker Compose
```yaml
version: '3.8'
services:
  cms-service:
    build: .
    ports:
      - "8080:8080"
    environment:
      - MONGODB_URI=mongodb://mongodb:27017
      - REDIS_URL=redis://redis:6379
    depends_on:
      - mongodb
      - redis
    volumes:
      - ./storage:/app/storage

  mongodb:
    image: mongo:7.0
    ports:
      - "27017:27017"
    volumes:
      - mongodb_data:/data/db

  redis:
    image: redis:7.0
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

volumes:
  mongodb_data:
  redis_data:
```

### Kubernetes Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cms-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: cms-service
  template:
    metadata:
      labels:
        app: cms-service
    spec:
      containers:
      - name: cms-service
        image: chaos-world/cms-service:latest
        ports:
        - containerPort: 8080
        env:
        - name: MONGODB_URI
          value: "mongodb://mongodb-service:27017"
        - name: REDIS_URL
          value: "redis://redis-service:6379"
---
apiVersion: v1
kind: Service
metadata:
  name: cms-service
spec:
  selector:
    app: cms-service
  ports:
  - port: 80
    targetPort: 8080
  type: LoadBalancer
```

## Integration

### API Gateway Integration
The CMS service integrates with the API Gateway for:
- Request routing and load balancing
- Authentication and authorization
- Rate limiting and throttling
- Request/response logging

### Game Backend Integration
The CMS service provides content to the game backend through:
- Real-time content updates via WebSocket
- Content synchronization APIs
- Event-driven content changes
- Bulk content loading for game initialization

### Admin Panel Integration
The CMS service supports admin panel integration through:
- RESTful APIs for content management
- WebSocket connections for real-time updates
- File upload APIs for assets
- Search and filtering capabilities

## Development

### Prerequisites
- Rust 1.70+
- MongoDB 6.0+
- Redis 7.0+
- Node.js 18+ (for frontend development)

### Setup
```bash
# Clone the repository
git clone <repository-url>
cd chaos-backend-service/services/content-management-service

# Install dependencies
cargo build

# Start dependencies
docker-compose up -d mongodb redis

# Run the service
cargo run
```

### Project Structure
```
src/
├── main.rs              # Application entry point
├── config/              # Configuration management
├── handlers/            # HTTP request handlers
├── models/              # Data models and schemas
├── services/            # Business logic services
├── middleware/          # HTTP middleware
├── utils/               # Utility functions
└── tests/               # Test files
```

### Adding New Content Types
1. Create a new model in `src/models/`
2. Add CRUD handlers in `src/handlers/`
3. Implement business logic in `src/services/`
4. Add API routes in `main.rs`
5. Create tests in `src/tests/`

## Testing

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
cargo test --test integration
```

### API Tests
```bash
# Start the service
cargo run &

# Run API tests
curl -X GET http://localhost:8080/health
curl -X GET http://localhost:8080/api/v1/quests
```

### Load Testing
```bash
# Install artillery
npm install -g artillery

# Run load tests
artillery run load-tests/quests-load-test.yml
```

## Monitoring and Observability

### Health Checks
- `GET /health` - Basic health check
- `GET /health/detailed` - Detailed health information
- `GET /metrics` - Prometheus metrics

### Logging
- Structured JSON logging
- Request/response logging
- Error tracking and alerting
- Performance metrics

### Metrics
- Request count and latency
- Database query performance
- Cache hit/miss ratios
- Content operation statistics

## Security

### Authentication
- JWT-based authentication
- Role-based access control (RBAC)
- API key authentication for services

### Authorization
- Content-level permissions
- Operation-level permissions
- Resource-based access control

### Data Protection
- Input validation and sanitization
- SQL injection prevention
- XSS protection
- CSRF protection

## Performance

### Caching Strategy
- Redis for frequently accessed content
- In-memory caching for hot data
- CDN integration for static assets

### Database Optimization
- Indexed queries for fast lookups
- Connection pooling
- Query optimization
- Read replicas for scaling

### Scaling
- Horizontal scaling with load balancers
- Database sharding for large datasets
- Microservice decomposition
- Event-driven architecture

## Troubleshooting

### Common Issues
1. **Database Connection Issues**
   - Check MongoDB connection string
   - Verify network connectivity
   - Check authentication credentials

2. **Cache Issues**
   - Verify Redis connection
   - Check cache TTL settings
   - Monitor cache hit/miss ratios

3. **File Upload Issues**
   - Check file size limits
   - Verify file type restrictions
   - Check storage permissions

### Debugging
```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Check service logs
docker logs cms-service

# Monitor database queries
mongosh --eval "db.setProfilingLevel(2)"
```

## Contributing

### Code Style
- Follow Rust naming conventions
- Use `rustfmt` for code formatting
- Run `clippy` for linting
- Write comprehensive tests

### Pull Request Process
1. Create a feature branch
2. Make your changes
3. Add tests for new functionality
4. Update documentation
5. Submit a pull request

### Documentation
- Update README for new features
- Add API documentation
- Include code examples
- Update deployment guides

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For support and questions:
- Create an issue in the repository
- Contact the development team
- Check the documentation wiki
- Join the Discord community
