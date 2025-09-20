# Anti-Cheat Service - Chaos World

## 📋 Overview

Anti-Cheat Service chịu trách nhiệm phát hiện và ngăn chặn các hành vi gian lận, hack, và cheat trong Chaos World MMORPG sử dụng machine learning và behavioral analysis.

## 🎯 Responsibilities

### Core Functions
- **Behavioral Analysis**: Phân tích hành vi người chơi
- **Cheat Detection**: Phát hiện các tool cheat và hack
- **Pattern Recognition**: Nhận diện các pattern bất thường
- **Real-time Monitoring**: Giám sát real-time
- **Automated Response**: Tự động phản ứng với các vi phạm
- **Manual Review**: Hỗ trợ review thủ công

### Performance Requirements
- **Latency**: < 50ms cho real-time analysis
- **Throughput**: 5,000+ TPS
- **Availability**: 99.9% uptime
- **Concurrency**: Handle 10,000+ concurrent analyses

## 🏗️ Architecture

### Technology Stack
```yaml
Language: Python/Rust
ML Framework: TensorFlow, PyTorch
Database: ClickHouse (time-series data)
Caching: Redis
Message Queue: Apache Kafka
ML Serving: TensorFlow Serving, TorchServe
```

### Core Components
```python
class AntiCheatService:
    def __init__(self):
        # ML Models
        self.behavior_model = BehaviorAnalysisModel()
        self.anomaly_detector = AnomalyDetectionModel()
        self.cheat_classifier = CheatClassificationModel()
        
        # Data Processing
        self.feature_extractor = FeatureExtractor()
        self.data_preprocessor = DataPreprocessor()
        
        # Real-time Processing
        self.stream_processor = StreamProcessor()
        self.rule_engine = RuleEngine()
        
        # Storage
        self.clickhouse_client = ClickHouseClient()
        self.redis_client = RedisClient()
        
        # External Services
        self.notification_service = NotificationService()
        self.ban_service = BanService()
```

## 🤖 Machine Learning Models

### Behavioral Analysis Model
```yaml
Purpose: Phân tích hành vi người chơi bình thường vs bất thường
Input Features:
  - Movement patterns
  - Action sequences
  - Timing patterns
  - Resource usage
  - Social interactions

Model Architecture:
  - LSTM for sequence analysis
  - CNN for pattern recognition
  - Attention mechanism
  - Ensemble methods

Output:
  - Anomaly score (0-1)
  - Risk level (low, medium, high, critical)
  - Confidence score
```

### Cheat Detection Model
```yaml
Purpose: Phát hiện các loại cheat cụ thể
Cheat Types:
  - Speed hacks
  - Teleportation
  - Auto-aim
  - Resource duplication
  - Invisible movement
  - Damage modification

Model Architecture:
  - Multi-class classification
  - Deep neural networks
  - Feature engineering
  - Ensemble learning

Output:
  - Cheat type classification
  - Confidence score
  - Evidence features
```

### Anomaly Detection Model
```yaml
Purpose: Phát hiện các pattern bất thường
Detection Methods:
  - Statistical analysis
  - Isolation Forest
  - One-Class SVM
  - Autoencoders
  - GAN-based detection

Features:
  - Statistical features
  - Temporal features
  - Spatial features
  - Network features

Output:
  - Anomaly score
  - Anomaly type
  - Severity level
```

## 📊 Data Collection

### Game Events
```yaml
Player Actions:
  - Movement events (position, velocity, acceleration)
  - Combat actions (attack, defend, cast spell)
  - Resource usage (mana, stamina, health)
  - Item interactions (pickup, use, trade)
  - Social actions (chat, guild, party)

System Events:
  - Login/logout events
  - Session duration
  - IP address changes
  - Device information
  - Network latency

Performance Events:
  - FPS data
  - Input lag
  - Network jitter
  - Memory usage
  - CPU usage
```

### Data Schema
```sql
-- Player actions table
CREATE TABLE player_actions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    session_id UUID NOT NULL,
    action_type VARCHAR(50) NOT NULL,
    action_data JSONB NOT NULL,
    position_x FLOAT,
    position_y FLOAT,
    position_z FLOAT,
    timestamp TIMESTAMP NOT NULL,
    server_timestamp TIMESTAMP NOT NULL,
    client_timestamp TIMESTAMP,
    network_latency INTEGER,
    created_at TIMESTAMP DEFAULT NOW()
) ENGINE = MergeTree()
ORDER BY (user_id, timestamp);

-- Cheat detection results table
CREATE TABLE cheat_detections (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    session_id UUID NOT NULL,
    cheat_type VARCHAR(50) NOT NULL,
    confidence_score FLOAT NOT NULL,
    risk_level VARCHAR(20) NOT NULL,
    evidence JSONB NOT NULL,
    model_version VARCHAR(20) NOT NULL,
    detected_at TIMESTAMP NOT NULL,
    status VARCHAR(20) DEFAULT 'pending', -- 'pending', 'confirmed', 'false_positive'
    reviewed_by UUID,
    reviewed_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
) ENGINE = MergeTree()
ORDER BY (user_id, detected_at);

-- Behavioral analysis results table
CREATE TABLE behavior_analysis (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    session_id UUID NOT NULL,
    analysis_period_start TIMESTAMP NOT NULL,
    analysis_period_end TIMESTAMP NOT NULL,
    anomaly_score FLOAT NOT NULL,
    behavior_pattern JSONB NOT NULL,
    risk_factors JSONB NOT NULL,
    recommendations JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
) ENGINE = MergeTree()
ORDER BY (user_id, analysis_period_start);
```

## 🔌 API Endpoints

### Analysis Endpoints
```yaml
POST /analysis/player-action:
  Description: Analyze single player action
  Request: { 
    user_id, 
    action_type, 
    action_data, 
    position, 
    timestamp 
  }
  Response: { 
    anomaly_score, 
    risk_level, 
    recommendations 
  }
  Rate Limit: 1000/minute per user

POST /analysis/session:
  Description: Analyze entire session
  Request: { 
    user_id, 
    session_id, 
    start_time, 
    end_time 
  }
  Response: { 
    session_analysis, 
    cheat_detections, 
    risk_assessment 
  }
  Rate Limit: 10/minute per user

GET /analysis/history/{user_id}:
  Description: Get analysis history for user
  Request: { user_id, page, limit }
  Response: { 
    analyses: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per user
```

### Detection Endpoints
```yaml
GET /detections/active:
  Description: Get active cheat detections
  Request: { page, limit, status, risk_level }
  Response: { 
    detections: [...], 
    total, 
    page, 
    limit 
  }
  Rate Limit: 1000/hour per admin

POST /detections/{detection_id}/review:
  Description: Review cheat detection
  Request: { 
    detection_id, 
    status, 
    reviewer_notes, 
    action_taken 
  }
  Response: { success: true }
  Rate Limit: 100/hour per admin

POST /detections/{detection_id}/ban:
  Description: Ban user based on detection
  Request: { 
    detection_id, 
    ban_duration, 
    ban_reason, 
    admin_notes 
  }
  Response: { 
    ban_id, 
    success: true 
  }
  Rate Limit: 50/hour per admin
```

### Model Management Endpoints
```yaml
POST /models/train:
  Description: Train new model
  Request: { 
    model_type, 
    training_data_period, 
    hyperparameters 
  }
  Response: { 
    training_id, 
    status: 'started' 
  }
  Rate Limit: 10/hour per admin

GET /models/status/{training_id}:
  Description: Get training status
  Request: { training_id }
  Response: { 
    status, 
    progress, 
    metrics, 
    estimated_completion 
  }
  Rate Limit: 1000/hour per admin

POST /models/deploy:
  Description: Deploy trained model
  Request: { 
    model_id, 
    version, 
    deployment_config 
  }
  Response: { 
    deployment_id, 
    status: 'deploying' 
  }
  Rate Limit: 5/hour per admin
```

## 🔍 Detection Rules

### Real-time Rules
```yaml
Speed Detection:
  - Maximum movement speed: 50 units/second
  - Acceleration threshold: 100 units/second²
  - Teleportation detection: > 1000 units instant
  - Wall clipping detection: movement through solid objects

Combat Detection:
  - Attack speed: maximum 10 attacks/second
  - Damage per second: based on weapon stats
  - Critical hit rate: maximum 50%
  - Spell cooldown: respect cooldown timers

Resource Detection:
  - Mana regeneration: maximum 100/second
  - Health regeneration: maximum 50/second
  - Item usage: respect cooldown timers
  - Resource duplication: detect impossible gains

Social Detection:
  - Chat spam: maximum 10 messages/minute
  - Trade frequency: maximum 100 trades/hour
  - Guild actions: respect cooldown timers
  - Friend requests: maximum 50/hour
```

### Machine Learning Rules
```yaml
Behavioral Anomalies:
  - Unusual movement patterns
  - Impossible action sequences
  - Inhuman reaction times
  - Suspicious timing patterns

Pattern Recognition:
  - Bot-like behavior
  - Scripted actions
  - Repetitive patterns
  - Lack of human variability

Statistical Analysis:
  - Outlier detection
  - Distribution analysis
  - Correlation analysis
  - Trend analysis
```

## 📈 Monitoring & Analytics

### Detection Metrics
```yaml
Detection Rate:
  - Total detections per day
  - Detection rate by cheat type
  - False positive rate
  - True positive rate
  - Precision and recall

Performance Metrics:
  - Analysis latency
  - Model accuracy
  - Processing throughput
  - System availability
  - Error rate

Business Metrics:
  - Banned users per day
  - Appeal success rate
  - Player retention impact
  - Revenue impact
  - Customer satisfaction
```

### Real-time Dashboards
```yaml
Live Monitoring:
  - Active detections
  - Risk level distribution
  - Top cheat types
  - Geographic distribution
  - System health

Historical Analysis:
  - Detection trends
  - Model performance
  - False positive analysis
  - Cheat evolution
  - Effectiveness metrics
```

## 🧪 Testing

### Unit Tests
```python
import pytest
from anti_cheat_service import AntiCheatService

class TestAntiCheatService:
    def test_speed_detection(self):
        # Test speed hack detection
        service = AntiCheatService()
        action = {
            'user_id': 'test_user',
            'action_type': 'movement',
            'position': {'x': 0, 'y': 0, 'z': 0},
            'velocity': 100,  # Too fast
            'timestamp': '2023-01-01T00:00:00Z'
        }
        
        result = service.analyze_action(action)
        assert result['anomaly_score'] > 0.8
        assert result['risk_level'] == 'high'
    
    def test_cheat_classification(self):
        # Test cheat type classification
        service = AntiCheatService()
        features = extract_features(suspicious_actions)
        
        prediction = service.classify_cheat(features)
        assert prediction['cheat_type'] in ['speed_hack', 'auto_aim', 'teleport']
        assert prediction['confidence'] > 0.7
```

### Integration Tests
```python
def test_complete_detection_flow():
    # Test complete detection flow
    service = AntiCheatService()
    
    # Simulate player actions
    actions = generate_suspicious_actions()
    
    for action in actions:
        result = service.analyze_action(action)
        assert result is not None
        assert 'anomaly_score' in result
        assert 'risk_level' in result
    
    # Test session analysis
    session_result = service.analyze_session('test_user', 'session_123')
    assert session_result['cheat_detections'] is not None
    assert session_result['risk_assessment'] is not None
```

### Load Tests
```python
# Load test with locust
from locust import HttpUser, task, between

class AntiCheatLoadTest(HttpUser):
    wait_time = between(0.1, 0.5)
    
    @task
    def analyze_action(self):
        action_data = {
            'user_id': 'test_user',
            'action_type': 'movement',
            'action_data': {'x': 100, 'y': 200, 'z': 50},
            'timestamp': '2023-01-01T00:00:00Z'
        }
        
        response = self.client.post('/analysis/player-action', json=action_data)
        assert response.status_code == 200
```

## 🚀 Deployment

### Docker
```dockerfile
FROM python:3.9-slim

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    gcc \
    g++ \
    && rm -rf /var/lib/apt/lists/*

# Install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy application code
COPY . .

# Expose port
EXPOSE 8083

# Run the application
CMD ["python", "main.py"]
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: anti-cheat-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: anti-cheat-service
  template:
    metadata:
      labels:
        app: anti-cheat-service
    spec:
      containers:
      - name: anti-cheat-service
        image: anti-cheat-service:latest
        ports:
        - containerPort: 8083
        env:
        - name: CLICKHOUSE_URL
          value: "clickhouse://clickhouse:9000"
        - name: REDIS_URL
          value: "redis://redis:6379"
        - name: KAFKA_BROKERS
          value: "kafka:9092"
        resources:
          requests:
            memory: "2Gi"
            cpu: "1000m"
          limits:
            memory: "4Gi"
            cpu: "2000m"
```

### ML Model Serving
```yaml
# TensorFlow Serving
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ml-model-serving
spec:
  replicas: 2
  selector:
    matchLabels:
      app: ml-model-serving
  template:
    metadata:
      labels:
        app: ml-model-serving
    spec:
      containers:
      - name: tensorflow-serving
        image: tensorflow/serving:latest
        ports:
        - containerPort: 8500
        - containerPort: 8501
        env:
        - name: MODEL_NAME
          value: "anti_cheat_model"
        - name: MODEL_BASE_PATH
          value: "/models"
        volumeMounts:
        - name: model-storage
          mountPath: /models
      volumes:
      - name: model-storage
        persistentVolumeClaim:
          claimName: model-pvc
```

## 🔧 Configuration

### Environment Variables
```yaml
# Database Configuration
CLICKHOUSE_URL=clickhouse://localhost:9000
CLICKHOUSE_DATABASE=anti_cheat
CLICKHOUSE_USERNAME=default
CLICKHOUSE_PASSWORD=secret

# Redis Configuration
REDIS_URL=redis://localhost:6379
REDIS_PASSWORD=secret
REDIS_DB=2

# Kafka Configuration
KAFKA_BROKERS=localhost:9092
KAFKA_TOPIC_PREFIX=anti_cheat
KAFKA_GROUP_ID=anti_cheat_service

# ML Model Configuration
MODEL_PATH=/models
MODEL_VERSION=latest
TENSORFLOW_SERVING_URL=http://localhost:8501
PYTORCH_SERVING_URL=http://localhost:8080

# Detection Configuration
ANOMALY_THRESHOLD=0.7
CHEAT_CONFIDENCE_THRESHOLD=0.8
RISK_LEVEL_THRESHOLD=0.6
BATCH_SIZE=1000
PROCESSING_INTERVAL=1000

# Server Configuration
SERVER_PORT=8083
SERVER_HOST=0.0.0.0
WORKERS=4
```

### Configuration File
```yaml
# anti-cheat-config.yaml
server:
  port: 8083
  host: "0.0.0.0"
  workers: 4

database:
  clickhouse:
    url: "clickhouse://localhost:9000"
    database: "anti_cheat"
    username: "default"
    password: "secret"
    pool_size: 20

redis:
  url: "redis://localhost:6379"
  password: "secret"
  db: 2
  pool_size: 100

kafka:
  brokers: ["localhost:9092"]
  topic_prefix: "anti_cheat"
  group_id: "anti_cheat_service"
  auto_offset_reset: "latest"

ml_models:
  behavior_analysis:
    model_path: "/models/behavior_analysis"
    version: "v1.0.0"
    serving_url: "http://localhost:8501"
  
  cheat_detection:
    model_path: "/models/cheat_detection"
    version: "v1.0.0"
    serving_url: "http://localhost:8501"
  
  anomaly_detection:
    model_path: "/models/anomaly_detection"
    version: "v1.0.0"
    serving_url: "http://localhost:8501"

detection:
  anomaly_threshold: 0.7
  cheat_confidence_threshold: 0.8
  risk_level_threshold: 0.6
  batch_size: 1000
  processing_interval: 1000

rules:
  speed_detection:
    max_speed: 50
    max_acceleration: 100
    teleport_threshold: 1000
  
  combat_detection:
    max_attack_speed: 10
    max_critical_rate: 0.5
    respect_cooldowns: true
  
  resource_detection:
    max_mana_regen: 100
    max_health_regen: 50
    detect_duplication: true
```

## 🔗 Related Services

- [Chaos Backend](./chaos-backend/README.md) - Game Logic Core
- [User Management](./user-management/README.md) - User authentication
- [Analytics Service](./analytics-service/README.md) - Data analysis
- [Microservices Architecture](./microservices-architecture/README.md) - Overall Architecture
