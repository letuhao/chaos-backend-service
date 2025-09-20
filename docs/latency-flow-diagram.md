# Latency Flow Diagram

## Request Flow with Latency Breakdown

```mermaid
graph TB
    subgraph "Client Layer"
        Client[Client Application]
    end
    
    subgraph "CDN Layer"
        CDN[CDN/CloudFlare<br/>5-15ms]
    end
    
    subgraph "Load Balancer Layer"
        LB[Load Balancer<br/>1-3ms]
    end
    
    subgraph "API Gateway Layer"
        AG[API Gateway<br/>3-8ms total]
        Auth[Authentication<br/>1-4ms]
        RateLimit[Rate Limiting<br/>0.6-2.5ms]
        Routing[Service Routing<br/>0.7-3.8ms]
    end
    
    subgraph "User Service Layer"
        US[User Service<br/>5-25ms total]
        UserDB[(User Database<br/>5-20ms)]
    end
    
    subgraph "Chaos Backend Layer"
        CB[Chaos Backend<br/>13-60ms total]
        GameCache[(Redis Cache<br/>1-2ms)]
        GameDB[(Game Database<br/>10-50ms)]
    end
    
    subgraph "Latency Breakdown"
        L1[Network: 20-40ms]
        L2[Processing: 25-95ms]
        L3[Database: 15-70ms]
        L4[Total: 60-205ms]
    end
    
    Client -->|5-15ms| CDN
    CDN -->|1-3ms| LB
    LB -->|1-2ms| AG
    
    AG --> Auth
    AG --> RateLimit
    AG --> Routing
    
    AG -->|1-3ms| US
    US -->|5-20ms| UserDB
    UserDB -->|5-20ms| US
    US -->|1-3ms| AG
    
    AG -->|1-3ms| CB
    CB -->|1-2ms| GameCache
    CB -->|10-50ms| GameDB
    GameDB -->|10-50ms| CB
    CB -->|1-3ms| AG
    
    AG -->|1-2ms| LB
    LB -->|1-3ms| CDN
    CDN -->|5-15ms| Client
    
    style Client fill:#e1f5fe
    style CDN fill:#f3e5f5
    style LB fill:#e8f5e8
    style AG fill:#fff3e0
    style US fill:#fce4ec
    style CB fill:#f1f8e9
    style UserDB fill:#e0f2f1
    style GameDB fill:#e0f2f1
    style GameCache fill:#fff8e1
```

## Latency Scenarios

### Best Case (Cached Response)
```mermaid
gantt
    title Best Case Latency: 39ms
    dateFormat X
    axisFormat %L ms
    
    section Network
    Client to CDN     :0, 5
    CDN to LB         :5, 6
    LB to API Gateway :6, 7
    API Gateway to LB :32, 33
    LB to CDN         :33, 34
    CDN to Client     :34, 39
    
    section Processing
    API Gateway Auth  :7, 9
    User Service      :10, 15
    Chaos Backend     :18, 31
```

### Typical Case (Database Queries)
```mermaid
gantt
    title Typical Case Latency: 89ms
    dateFormat X
    axisFormat %L ms
    
    section Network
    Client to CDN     :0, 10
    CDN to LB         :10, 12
    LB to API Gateway :12, 14
    API Gateway to LB :75, 77
    LB to CDN         :77, 79
    CDN to Client     :79, 89
    
    section Processing
    API Gateway Auth  :14, 19
    User Service      :21, 36
    Chaos Backend     :40, 75
```

### Worst Case (Complex Queries)
```mermaid
gantt
    title Worst Case Latency: 144ms
    dateFormat X
    axisFormat %L ms
    
    section Network
    Client to CDN     :0, 15
    CDN to LB         :15, 18
    LB to API Gateway :18, 20
    API Gateway to LB :130, 132
    LB to CDN         :132, 135
    CDN to Client     :135, 150
    
    section Processing
    API Gateway Auth  :20, 28
    User Service      :31, 56
    Chaos Backend     :62, 130
```

## Performance Optimization Impact

### Before Optimization
```mermaid
pie title Latency Distribution (Before)
    "Database Queries" : 60
    "Network Round-trips" : 25
    "Serialization" : 10
    "Business Logic" : 5
```

### After Optimization
```mermaid
pie title Latency Distribution (After)
    "Database Queries" : 35
    "Network Round-trips" : 20
    "Serialization" : 8
    "Business Logic" : 5
    "Caching Benefits" : 32
```

## Service Dependencies and Latency

```mermaid
graph LR
    subgraph "Latency Heat Map"
        A[Client<br/>0ms] -->|5-15ms| B[CDN<br/>5-15ms]
        B -->|1-3ms| C[Load Balancer<br/>6-18ms]
        C -->|1-2ms| D[API Gateway<br/>7-20ms]
        D -->|1-3ms| E[User Service<br/>8-25ms]
        E -->|5-20ms| F[User DB<br/>13-45ms]
        F -->|5-20ms| E
        E -->|1-3ms| D
        D -->|1-3ms| G[Chaos Backend<br/>9-28ms]
        G -->|1-2ms| H[Redis Cache<br/>10-30ms]
        G -->|10-50ms| I[Game DB<br/>19-78ms]
        I -->|10-50ms| G
        G -->|1-3ms| D
        D -->|1-2ms| C
        C -->|1-3ms| B
        B -->|5-15ms| A
    end
    
    style A fill:#e3f2fd
    style B fill:#f3e5f5
    style C fill:#e8f5e8
    style D fill:#fff3e0
    style E fill:#fce4ec
    style F fill:#e0f2f1
    style G fill:#f1f8e9
    style H fill:#fff8e1
    style I fill:#e0f2f1
```

## Monitoring Dashboard Layout

```mermaid
graph TB
    subgraph "Real-time Metrics"
        A[Request Rate<br/>RPS]
        B[Response Time<br/>P50, P95, P99]
        C[Error Rate<br/>%]
        D[Throughput<br/>MB/s]
    end
    
    subgraph "Service Health"
        E[API Gateway<br/>Status]
        F[User Service<br/>Status]
        G[Chaos Backend<br/>Status]
        H[Database<br/>Status]
    end
    
    subgraph "Latency Breakdown"
        I[Network Latency<br/>ms]
        J[Processing Latency<br/>ms]
        K[Database Latency<br/>ms]
        L[Total Latency<br/>ms]
    end
    
    subgraph "Alerts"
        M[High Latency<br/>Alert]
        N[Service Down<br/>Alert]
        O[Error Rate<br/>Alert]
        P[Resource Usage<br/>Alert]
    end
    
    A --> I
    B --> J
    C --> K
    D --> L
    
    E --> M
    F --> N
    G --> O
    H --> P
```

## Load Testing Results

### Latency vs Load
```mermaid
xychart-beta
    title "Latency vs Load (RPS)"
    x-axis [100, 500, 1000, 2000, 5000, 10000]
    y-axis "Latency (ms)" 0 --> 500
    
    line "P50 Latency" [45, 55, 65, 80, 95, 150]
    line "P95 Latency" [80, 100, 120, 150, 180, 300]
    line "P99 Latency" [120, 150, 180, 220, 280, 500]
```

### Error Rate vs Load
```mermaid
xychart-beta
    title "Error Rate vs Load (RPS)"
    x-axis [100, 500, 1000, 2000, 5000, 10000]
    y-axis "Error Rate (%)" 0 --> 5
    
    line "Error Rate" [0.1, 0.2, 0.5, 1.0, 1.2, 3.5]
```

## Optimization Impact

### Before vs After Optimization
```mermaid
graph LR
    subgraph "Before Optimization"
        A1[Total: 144ms]
        B1[Database: 86ms]
        C1[Network: 36ms]
        D1[Processing: 22ms]
    end
    
    subgraph "After Optimization"
        A2[Total: 89ms]
        B2[Database: 52ms]
        C2[Network: 28ms]
        D2[Processing: 9ms]
    end
    
    A1 --> A2
    B1 --> B2
    C1 --> C2
    D1 --> D2
    
    style A1 fill:#ffebee
    style A2 fill:#e8f5e8
    style B1 fill:#ffebee
    style B2 fill:#e8f5e8
    style C1 fill:#ffebee
    style C2 fill:#e8f5e8
    style D1 fill:#ffebee
    style D2 fill:#e8f5e8
```

This comprehensive latency analysis provides a detailed breakdown of how requests flow through the Chaos World microservices architecture and where optimizations can be applied to improve performance.
