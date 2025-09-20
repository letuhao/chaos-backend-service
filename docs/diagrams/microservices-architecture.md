# Microservices Architecture Diagrams

## 🏗️ Overall Architecture

```mermaid
graph TB
    subgraph "Client Layer"
        Web[Web Client]
        Mobile[Mobile App]
        Desktop[Desktop App]
    end
    
    subgraph "API Gateway Layer"
        Gateway[API Gateway]
        LoadBalancer[Load Balancer]
    end
    
    subgraph "Core Game Services"
        ChaosBackend[Chaos Backend<br/>Game Logic Core]
        ActorCore[Actor Core<br/>Library]
    end
    
    subgraph "User & Authentication Services"
        UserMgmt[User Management]
        Notification[Notification Service]
    end
    
    subgraph "Game Feature Services"
        Inventory[Inventory Service]
        Chat[Chat Service]
        Guild[Guild Service]
        World[World Service]
        Matchmaking[Matchmaking Service]
        Event[Event Service]
    end
    
    subgraph "Content & Analytics Services"
        ContentMgmt[Content Management]
        Analytics[Analytics Service]
    end
    
    subgraph "Business & Security Services"
        Payment[Payment Service]
        AntiCheat[Anti-Cheat Service]
    end
    
    subgraph "Data Layer"
        PostgreSQL[(PostgreSQL)]
        Redis[(Redis Cache)]
        MongoDB[(MongoDB)]
        Kafka[(Apache Kafka)]
    end
    
    Web --> Gateway
    Mobile --> Gateway
    Desktop --> Gateway
    
    Gateway --> LoadBalancer
    LoadBalancer --> ChaosBackend
    LoadBalancer --> UserMgmt
    LoadBalancer --> Inventory
    LoadBalancer --> Chat
    LoadBalancer --> Guild
    LoadBalancer --> World
    LoadBalancer --> Matchmaking
    LoadBalancer --> Event
    LoadBalancer --> ContentMgmt
    LoadBalancer --> Analytics
    LoadBalancer --> Payment
    LoadBalancer --> AntiCheat
    LoadBalancer --> Notification
    
    ChaosBackend --> ActorCore
    ChaosBackend --> PostgreSQL
    ChaosBackend --> Redis
    
    UserMgmt --> PostgreSQL
    UserMgmt --> Redis
    
    Inventory --> PostgreSQL
    Inventory --> Redis
    Inventory --> Kafka
    
    Chat --> PostgreSQL
    Chat --> Redis
    Chat --> Kafka
    
    Guild --> PostgreSQL
    Guild --> Redis
    Guild --> Kafka
    
    World --> PostgreSQL
    World --> Redis
    World --> Kafka
    
    Matchmaking --> PostgreSQL
    Matchmaking --> Redis
    Matchmaking --> Kafka
    
    Event --> PostgreSQL
    Event --> Redis
    Event --> Kafka
    
    ContentMgmt --> PostgreSQL
    ContentMgmt --> Redis
    ContentMgmt --> MongoDB
    
    Analytics --> PostgreSQL
    Analytics --> MongoDB
    Analytics --> Kafka
    
    Payment --> PostgreSQL
    Payment --> Redis
    Payment --> Kafka
    
    AntiCheat --> PostgreSQL
    AntiCheat --> Redis
    AntiCheat --> Kafka
    
    Notification --> PostgreSQL
    Notification --> Redis
    Notification --> Kafka
```

## 🔄 Service Communication Flow

```mermaid
sequenceDiagram
    participant Client
    participant Gateway as API Gateway
    participant UserMgmt as User Management
    participant ChaosBackend as Chaos Backend
    participant Inventory as Inventory Service
    participant Chat as Chat Service
    participant Guild as Guild Service
    participant Notification as Notification Service
    
    Client->>Gateway: Login Request
    Gateway->>UserMgmt: Authenticate User
    UserMgmt-->>Gateway: Auth Token
    Gateway-->>Client: Login Success
    
    Client->>Gateway: Get Player Data
    Gateway->>ChaosBackend: Get Actor Data
    ChaosBackend-->>Gateway: Actor Data
    Gateway-->>Client: Player Data
    
    Client->>Gateway: Join Guild
    Gateway->>Guild: Create Guild Invitation
    Guild->>Notification: Send Guild Invite
    Notification-->>Guild: Notification Sent
    Guild-->>Gateway: Invitation Created
    Gateway-->>Client: Guild Invitation Sent
    
    Client->>Gateway: Send Chat Message
    Gateway->>Chat: Send Message
    Chat->>Notification: Notify Recipients
    Notification-->>Chat: Notifications Sent
    Chat-->>Gateway: Message Sent
    Gateway-->>Client: Message Delivered
    
    Client->>Gateway: Use Item
    Gateway->>Inventory: Process Item Use
    Inventory->>ChaosBackend: Apply Item Effects
    ChaosBackend-->>Inventory: Effects Applied
    Inventory-->>Gateway: Item Used
    Gateway-->>Client: Item Effect Applied
```

## 🎮 Game Action Flow

```mermaid
sequenceDiagram
    participant Player
    participant Frontend
    participant Gateway as API Gateway
    participant ChaosBackend as Chaos Backend
    participant World as World Service
    participant Inventory as Inventory Service
    participant Guild as Guild Service
    participant Analytics as Analytics Service
    
    Player->>Frontend: Cast Fireball Spell
    Frontend->>Gateway: Use Skill Request
    Gateway->>ChaosBackend: Process Skill
    ChaosBackend->>World: Check Environment
    World-->>ChaosBackend: Environment OK
    ChaosBackend->>Inventory: Consume Mana
    Inventory-->>ChaosBackend: Mana Consumed
    ChaosBackend->>World: Apply Damage
    World->>Guild: Update Territory
    Guild-->>World: Territory Updated
    World-->>ChaosBackend: Damage Applied
    ChaosBackend->>Analytics: Log Action
    Analytics-->>ChaosBackend: Action Logged
    ChaosBackend-->>Gateway: Skill Executed
    Gateway-->>Frontend: Success Response
    Frontend-->>Player: Fireball Cast
```

## 🗄️ Database Architecture

```mermaid
erDiagram
    USERS ||--o{ ACTORS : has
    USERS ||--o{ GUILD_MEMBERS : member_of
    USERS ||--o{ INVENTORY_ITEMS : owns
    USERS ||--o{ CHAT_MESSAGES : sends
    USERS ||--o{ EVENT_PARTICIPATIONS : participates
    
    ACTORS ||--o{ ACTOR_RESOURCES : has
    ACTORS ||--o{ ACTOR_SKILLS : learns
    ACTORS ||--o{ ACTOR_EQUIPMENT : wears
    
    GUILDS ||--o{ GUILD_MEMBERS : contains
    GUILDS ||--o{ GUILD_STORAGE : owns
    GUILDS ||--o{ TERRITORIES : controls
    
    ITEMS ||--o{ INVENTORY_ITEMS : instance_of
    ITEMS ||--o{ CRAFTING_RECIPES : used_in
    
    EVENTS ||--o{ EVENT_PARTICIPATIONS : has
    EVENTS ||--o{ EVENT_REWARDS : gives
    
    USERS {
        uuid id PK
        string username
        string email
        timestamp created_at
        timestamp last_login
    }
    
    ACTORS {
        uuid id PK
        uuid user_id FK
        string name
        int level
        int experience
        json stats
    }
    
    GUILDS {
        uuid id PK
        string name
        string description
        int level
        int member_count
    }
    
    ITEMS {
        uuid id PK
        string item_id
        string name
        string type
        string rarity
        json properties
    }
    
    EVENTS {
        uuid id PK
        string event_id
        string name
        string type
        timestamp start_time
        timestamp end_time
    }
```

## 🔐 Security Architecture

```mermaid
graph TB
    subgraph "Client Security"
        HTTPS[HTTPS/TLS 1.3]
        JWT[JWT Tokens]
        OAuth[OAuth 2.0]
    end
    
    subgraph "API Gateway Security"
        RateLimit[Rate Limiting]
        AuthCheck[Authentication Check]
        AuthzCheck[Authorization Check]
        InputValidation[Input Validation]
    end
    
    subgraph "Service Security"
        ServiceAuth[Service-to-Service Auth]
        Encryption[Data Encryption]
        AuditLog[Audit Logging]
    end
    
    subgraph "Data Security"
        DBEncryption[Database Encryption]
        BackupEncryption[Backup Encryption]
        AccessControl[Access Control]
    end
    
    subgraph "Network Security"
        VPC[VPC Isolation]
        Firewall[Firewall Rules]
        DDoS[DDoS Protection]
    end
    
    HTTPS --> RateLimit
    JWT --> AuthCheck
    OAuth --> AuthzCheck
    
    RateLimit --> ServiceAuth
    AuthCheck --> Encryption
    AuthzCheck --> AuditLog
    
    ServiceAuth --> DBEncryption
    Encryption --> BackupEncryption
    AuditLog --> AccessControl
    
    DBEncryption --> VPC
    BackupEncryption --> Firewall
    AccessControl --> DDoS
```

## 📊 Monitoring & Observability

```mermaid
graph TB
    subgraph "Application Metrics"
        AppMetrics[Application Metrics]
        BusinessMetrics[Business Metrics]
        CustomMetrics[Custom Metrics]
    end
    
    subgraph "Infrastructure Metrics"
        ServerMetrics[Server Metrics]
        NetworkMetrics[Network Metrics]
        DatabaseMetrics[Database Metrics]
    end
    
    subgraph "Logging"
        AppLogs[Application Logs]
        AccessLogs[Access Logs]
        ErrorLogs[Error Logs]
        AuditLogs[Audit Logs]
    end
    
    subgraph "Tracing"
        RequestTracing[Request Tracing]
        ServiceTracing[Service Tracing]
        DatabaseTracing[Database Tracing]
    end
    
    subgraph "Alerting"
        ThresholdAlerts[Threshold Alerts]
        AnomalyAlerts[Anomaly Alerts]
        BusinessAlerts[Business Alerts]
    end
    
    subgraph "Dashboards"
        RealTimeDash[Real-time Dashboard]
        HistoricalDash[Historical Dashboard]
        BusinessDash[Business Dashboard]
    end
    
    AppMetrics --> RealTimeDash
    BusinessMetrics --> BusinessDash
    CustomMetrics --> HistoricalDash
    
    ServerMetrics --> ThresholdAlerts
    NetworkMetrics --> AnomalyAlerts
    DatabaseMetrics --> BusinessAlerts
    
    AppLogs --> RequestTracing
    AccessLogs --> ServiceTracing
    ErrorLogs --> DatabaseTracing
    AuditLogs --> RequestTracing
    
    RequestTracing --> RealTimeDash
    ServiceTracing --> HistoricalDash
    DatabaseTracing --> BusinessDash
```

## 🚀 Deployment Architecture

```mermaid
graph TB
    subgraph "Load Balancer Layer"
        LB[Load Balancer]
        CDN[CDN]
    end
    
    subgraph "Kubernetes Cluster"
        subgraph "API Gateway Namespace"
            GatewayPod1[API Gateway Pod 1]
            GatewayPod2[API Gateway Pod 2]
            GatewayPod3[API Gateway Pod 3]
        end
        
        subgraph "Core Services Namespace"
            ChaosPod1[Chaos Backend Pod 1]
            ChaosPod2[Chaos Backend Pod 2]
            UserPod1[User Management Pod 1]
            UserPod2[User Management Pod 2]
        end
        
        subgraph "Game Services Namespace"
            InventoryPod1[Inventory Pod 1]
            InventoryPod2[Inventory Pod 2]
            ChatPod1[Chat Pod 1]
            ChatPod2[Chat Pod 2]
            GuildPod1[Guild Pod 1]
            GuildPod2[Guild Pod 2]
        end
        
        subgraph "Data Services Namespace"
            PostgresPod1[PostgreSQL Primary]
            PostgresPod2[PostgreSQL Replica]
            RedisPod1[Redis Master]
            RedisPod2[Redis Replica]
        end
    end
    
    subgraph "External Services"
        MongoDB[MongoDB Atlas]
        Kafka[Kafka Cluster]
        S3[S3 Storage]
    end
    
    LB --> GatewayPod1
    LB --> GatewayPod2
    LB --> GatewayPod3
    
    CDN --> LB
    
    GatewayPod1 --> ChaosPod1
    GatewayPod1 --> UserPod1
    GatewayPod1 --> InventoryPod1
    GatewayPod1 --> ChatPod1
    GatewayPod1 --> GuildPod1
    
    ChaosPod1 --> PostgresPod1
    ChaosPod1 --> RedisPod1
    UserPod1 --> PostgresPod1
    UserPod1 --> RedisPod1
    
    InventoryPod1 --> PostgresPod1
    InventoryPod1 --> RedisPod1
    InventoryPod1 --> Kafka
    
    ChatPod1 --> PostgresPod1
    ChatPod1 --> RedisPod1
    ChatPod1 --> Kafka
    
    GuildPod1 --> PostgresPod1
    GuildPod1 --> RedisPod1
    GuildPod1 --> Kafka
    
    PostgresPod1 --> MongoDB
    RedisPod1 --> Kafka
    InventoryPod1 --> S3
```

## 🔄 Data Flow Architecture

```mermaid
graph LR
    subgraph "Data Sources"
        GameEvents[Game Events]
        UserActions[User Actions]
        SystemMetrics[System Metrics]
        BusinessEvents[Business Events]
    end
    
    subgraph "Data Ingestion"
        Kafka[Apache Kafka]
        EventStreams[Event Streams]
        DataValidation[Data Validation]
    end
    
    subgraph "Data Processing"
        StreamProcessing[Stream Processing]
        BatchProcessing[Batch Processing]
        RealTimeProcessing[Real-time Processing]
    end
    
    subgraph "Data Storage"
        OperationalDB[(Operational DB)]
        DataWarehouse[(Data Warehouse)]
        DataLake[(Data Lake)]
        Cache[(Cache)]
    end
    
    subgraph "Data Analytics"
        RealTimeAnalytics[Real-time Analytics]
        BatchAnalytics[Batch Analytics]
        MLModels[ML Models]
    end
    
    subgraph "Data Output"
        Dashboards[Dashboards]
        Reports[Reports]
        Alerts[Alerts]
        APIs[APIs]
    end
    
    GameEvents --> Kafka
    UserActions --> Kafka
    SystemMetrics --> Kafka
    BusinessEvents --> Kafka
    
    Kafka --> EventStreams
    EventStreams --> DataValidation
    
    DataValidation --> StreamProcessing
    DataValidation --> BatchProcessing
    DataValidation --> RealTimeProcessing
    
    StreamProcessing --> OperationalDB
    BatchProcessing --> DataWarehouse
    RealTimeProcessing --> Cache
    
    OperationalDB --> DataLake
    DataWarehouse --> DataLake
    Cache --> DataLake
    
    DataLake --> RealTimeAnalytics
    DataLake --> BatchAnalytics
    DataLake --> MLModels
    
    RealTimeAnalytics --> Dashboards
    BatchAnalytics --> Reports
    MLModels --> Alerts
    RealTimeAnalytics --> APIs
```
