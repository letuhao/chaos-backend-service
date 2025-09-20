# Analytics Service - Chaos World

## 📋 Overview

Analytics Service chịu trách nhiệm thu thập, xử lý, và phân tích dữ liệu từ tất cả các services để cung cấp insights cho business intelligence, game balancing, và decision making.

## 🎯 Responsibilities

### Core Functions
- **Data Collection**: Thu thập dữ liệu từ tất cả services
- **Data Processing**: Xử lý và transform dữ liệu
- **Real-time Analytics**: Phân tích real-time
- **Business Intelligence**: Báo cáo và dashboard
- **Machine Learning**: ML models cho predictions
- **Data Warehousing**: Lưu trữ dữ liệu lịch sử

### Performance Requirements
- **Latency**: < 1s cho real-time analytics
- **Throughput**: 100,000+ events/second
- **Availability**: 99.9% uptime
- **Storage**: Petabyte-scale data storage

## 🏗️ Architecture

### Technology Stack
```yaml
Language: Python/Go
Stream Processing: Apache Kafka, Apache Flink
Batch Processing: Apache Spark
Database: ClickHouse, BigQuery
Data Lake: S3, HDFS
ML Framework: TensorFlow, PyTorch, Scikit-learn
Visualization: Grafana, Tableau
```

### Core Components
```python
class AnalyticsService:
    def __init__(self):
        # Data Collection
        self.event_collector = EventCollector()
        self.kafka_consumer = KafkaConsumer()
        self.api_collector = APICollector()
        
        # Data Processing
        self.stream_processor = StreamProcessor()
        self.batch_processor = BatchProcessor()
        self.data_transformer = DataTransformer()
        
        # Storage
        self.clickhouse_client = ClickHouseClient()
        self.bigquery_client = BigQueryClient()
        self.s3_client = S3Client()
        
        # Analytics
        self.realtime_analyzer = RealtimeAnalyzer()
        self.batch_analyzer = BatchAnalyzer()
        self.ml_pipeline = MLPipeline()
        
        # Reporting
        self.dashboard_service = DashboardService()
        self.report_generator = ReportGenerator()
        self.alert_service = AlertService()
```

## 📊 Data Collection

### Event Types
```yaml
Game Events:
  - Player actions (movement, combat, social)
  - Item transactions
  - Guild activities
  - World events
  - Combat outcomes

User Events:
  - Login/logout
  - Session duration
  - Feature usage
  - Error events
  - Performance metrics

Business Events:
  - Purchases
  - Subscriptions
  - Refunds
  - Customer support
  - Marketing campaigns

System Events:
  - Server performance
  - Database metrics
  - Network latency
  - Error rates
  - Resource usage
```

### Data Schema
```sql
-- Events table (ClickHouse)
CREATE TABLE events (
    id UUID,
    event_type String,
    user_id UUID,
    session_id UUID,
    timestamp DateTime64(3),
    server_timestamp DateTime64(3),
    properties Map(String, String),
    metrics Map(String, Float64),
    dimensions Map(String, String),
    created_at DateTime DEFAULT now()
) ENGINE = MergeTree()
ORDER BY (event_type, timestamp, user_id)
PARTITION BY toYYYYMM(timestamp);

-- User analytics table
CREATE TABLE user_analytics (
    user_id UUID,
    date Date,
    session_count UInt32,
    session_duration UInt32,
    actions_count UInt32,
    purchases_count UInt32,
    purchases_amount Decimal(15,2),
    level UInt16,
    experience UInt32,
    created_at DateTime DEFAULT now()
) ENGINE = SummingMergeTree()
ORDER BY (user_id, date)
PARTITION BY toYYYYMM(date);

-- Game metrics table
CREATE TABLE game_metrics (
    metric_name String,
    metric_value Float64,
    dimensions Map(String, String),
    timestamp DateTime64(3),
    created_at DateTime DEFAULT now()
) ENGINE = MergeTree()
ORDER BY (metric_name, timestamp)
PARTITION BY toYYYYMM(timestamp);
```

## 🔌 API Endpoints

### Analytics Endpoints
```yaml
GET /analytics/real-time:
  Description: Get real-time analytics
  Request: { metric_names, time_range, dimensions }
  Response: { 
    metrics: [...], 
    timestamp, 
    status 
  }
  Rate Limit: 1000/minute per user

GET /analytics/historical:
  Description: Get historical analytics
  Request: { 
    metric_names, 
    start_date, 
    end_date, 
    granularity,
    dimensions 
  }
  Response: { 
    data: [...], 
    metadata: {...} 
  }
  Rate Limit: 100/minute per user

POST /analytics/query:
  Description: Custom analytics query
  Request: { 
    query, 
    parameters, 
    format 
  }
  Response: { 
    results: [...], 
    execution_time,
    row_count 
  }
  Rate Limit: 10/minute per user
```

### Dashboard Endpoints
```yaml
GET /dashboards:
  Description: Get available dashboards
  Request: { user_id, category }
  Response: { 
    dashboards: [...], 
    total 
  }
  Rate Limit: 1000/hour per user

GET /dashboards/{dashboard_id}:
  Description: Get dashboard data
  Request: { dashboard_id, time_range, filters }
  Response: { 
    dashboard: {...}, 
    widgets: [...], 
    data: {...} 
  }
  Rate Limit: 1000/hour per user

POST /dashboards:
  Description: Create custom dashboard
  Request: { 
    name, 
    description, 
    widgets, 
    filters 
  }
  Response: { 
    dashboard_id, 
    status: 'created' 
  }
  Rate Limit: 10/hour per user
```

### Report Endpoints
```yaml
GET /reports:
  Description: Get available reports
  Request: { category, status }
  Response: { 
    reports: [...], 
    total 
  }
  Rate Limit: 1000/hour per user

POST /reports/generate:
  Description: Generate custom report
  Request: { 
    report_type, 
    parameters, 
    format,
    schedule 
  }
  Response: { 
    report_id, 
    status: 'generating',
    estimated_completion 
  }
  Rate Limit: 5/hour per user

GET /reports/{report_id}:
  Description: Get report status and data
  Request: { report_id }
  Response: { 
    status, 
    progress, 
    download_url,
    expires_at 
  }
  Rate Limit: 1000/hour per user
```

## 📈 Analytics Categories

### Player Analytics
```yaml
Engagement Metrics:
  - Daily/Monthly Active Users (DAU/MAU)
  - Session duration and frequency
  - Retention rates (1-day, 7-day, 30-day)
  - Churn analysis
  - Player lifetime value (LTV)

Behavior Analytics:
  - Feature adoption rates
  - User journey analysis
  - A/B test results
  - Cohort analysis
  - Funnel analysis

Performance Analytics:
  - Level progression rates
  - Achievement completion
  - Skill development
  - Social interactions
  - Content consumption
```

### Business Analytics
```yaml
Revenue Analytics:
  - Daily/Monthly Recurring Revenue (DRR/MRR)
  - Average Revenue Per User (ARPU)
  - Revenue by segment
  - Conversion rates
  - Payment method analysis

Product Analytics:
  - Feature usage statistics
  - Content performance
  - Item popularity
  - Pricing optimization
  - Market analysis

Operational Analytics:
  - Server performance
  - Error rates and patterns
  - Resource utilization
  - Cost analysis
  - Efficiency metrics
```

### Game Analytics
```yaml
Gameplay Analytics:
  - Combat statistics
  - Quest completion rates
  - PvP/PvE participation
  - Guild activities
  - World events

Balance Analytics:
  - Class/character popularity
  - Item usage statistics
  - Skill effectiveness
  - Difficulty curves
  - Meta analysis

Social Analytics:
  - Guild formation and activity
  - Friend networks
  - Communication patterns
  - Community engagement
  - Moderation metrics
```

## 🤖 Machine Learning

### Predictive Models
```yaml
Churn Prediction:
  - Model: Random Forest, XGBoost
  - Features: Engagement, spending, behavior
  - Output: Churn probability
  - Action: Retention campaigns

Revenue Prediction:
  - Model: Linear Regression, Neural Networks
  - Features: Historical spending, engagement
  - Output: Expected revenue
  - Action: Revenue forecasting

Player Segmentation:
  - Model: K-means, DBSCAN
  - Features: Behavior, spending, engagement
  - Output: Player segments
  - Action: Targeted marketing

Content Recommendation:
  - Model: Collaborative Filtering, Matrix Factorization
  - Features: Player preferences, history
  - Output: Recommended content
  - Action: Personalized experience
```

### Real-time ML
```yaml
Anomaly Detection:
  - Model: Isolation Forest, One-Class SVM
  - Features: Real-time metrics
  - Output: Anomaly scores
  - Action: Alert generation

Fraud Detection:
  - Model: Random Forest, Neural Networks
  - Features: Transaction patterns
  - Output: Fraud probability
  - Action: Transaction blocking

Dynamic Pricing:
  - Model: Reinforcement Learning
  - Features: Demand, supply, competition
  - Output: Optimal prices
  - Action: Price updates
```

## 📊 Dashboards

### Executive Dashboard
```yaml
Key Metrics:
  - Total Revenue
  - Active Users
  - Retention Rate
  - Customer Acquisition Cost (CAC)
  - Lifetime Value (LTV)

Visualizations:
  - Revenue trends
  - User growth
  - Geographic distribution
  - Top performing features
  - Risk indicators
```

### Operations Dashboard
```yaml
System Metrics:
  - Server performance
  - Database health
  - API response times
  - Error rates
  - Resource utilization

Alerts:
  - Performance degradation
  - Error spikes
  - Capacity issues
  - Security events
  - Data quality issues
```

### Game Design Dashboard
```yaml
Game Metrics:
  - Player progression
  - Content engagement
  - Balance metrics
  - Social features
  - Monetization

A/B Tests:
  - Test results
  - Statistical significance
  - Conversion rates
  - User feedback
  - Recommendations
```

## 🧪 Testing

### Unit Tests
```python
import pytest
from analytics_service import AnalyticsService

class TestAnalyticsService:
    def test_event_collection(self):
        service = AnalyticsService()
        event = {
            'event_type': 'player_action',
            'user_id': 'test_user',
            'properties': {'action': 'move', 'x': 100, 'y': 200},
            'timestamp': '2023-01-01T00:00:00Z'
        }
        
        result = service.collect_event(event)
        assert result['status'] == 'success'
        assert result['event_id'] is not None
    
    def test_metric_calculation(self):
        service = AnalyticsService()
        events = generate_test_events()
        
        metrics = service.calculate_metrics(events)
        assert 'dau' in metrics
        assert 'session_duration' in metrics
        assert metrics['dau'] > 0
```

### Integration Tests
```python
def test_end_to_end_analytics():
    service = AnalyticsService()
    
    # Collect events
    events = generate_test_events(1000)
    for event in events:
        service.collect_event(event)
    
    # Process data
    service.process_batch_data()
    
    # Generate analytics
    analytics = service.generate_analytics()
    assert analytics is not None
    assert 'player_metrics' in analytics
    assert 'business_metrics' in analytics
```

### Load Tests
```python
# Load test with locust
from locust import HttpUser, task, between

class AnalyticsLoadTest(HttpUser):
    wait_time = between(0.1, 0.5)
    
    @task
    def collect_event(self):
        event_data = {
            'event_type': 'player_action',
            'user_id': 'test_user',
            'properties': {'action': 'move', 'x': 100, 'y': 200},
            'timestamp': '2023-01-01T00:00:00Z'
        }
        
        response = self.client.post('/analytics/events', json=event_data)
        assert response.status_code == 200
    
    @task
    def get_analytics(self):
        response = self.client.get('/analytics/real-time')
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
EXPOSE 8084

# Run the application
CMD ["python", "main.py"]
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: analytics-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: analytics-service
  template:
    metadata:
      labels:
        app: analytics-service
    spec:
      containers:
      - name: analytics-service
        image: analytics-service:latest
        ports:
        - containerPort: 8084
        env:
        - name: CLICKHOUSE_URL
          value: "clickhouse://clickhouse:9000"
        - name: KAFKA_BROKERS
          value: "kafka:9092"
        - name: S3_ENDPOINT
          value: "https://s3.amazonaws.com"
        resources:
          requests:
            memory: "2Gi"
            cpu: "1000m"
          limits:
            memory: "4Gi"
            cpu: "2000m"
```

### Data Pipeline
```yaml
# Apache Airflow DAG
from airflow import DAG
from airflow.operators.python_operator import PythonOperator
from datetime import datetime, timedelta

default_args = {
    'owner': 'analytics-team',
    'depends_on_past': False,
    'start_date': datetime(2023, 1, 1),
    'email_on_failure': True,
    'email_on_retry': False,
    'retries': 1,
    'retry_delay': timedelta(minutes=5)
}

dag = DAG(
    'analytics_pipeline',
    default_args=default_args,
    description='Analytics data processing pipeline',
    schedule_interval=timedelta(hours=1),
    catchup=False
)

# Data collection task
collect_data = PythonOperator(
    task_id='collect_data',
    python_callable=collect_events,
    dag=dag
)

# Data processing task
process_data = PythonOperator(
    task_id='process_data',
    python_callable=process_events,
    dag=dag
)

# Analytics generation task
generate_analytics = PythonOperator(
    task_id='generate_analytics',
    python_callable=generate_analytics,
    dag=dag
)

# Set task dependencies
collect_data >> process_data >> generate_analytics
```

## 🔧 Configuration

### Environment Variables
```yaml
# Database Configuration
CLICKHOUSE_URL=clickhouse://localhost:9000
CLICKHOUSE_DATABASE=analytics
BIGQUERY_PROJECT_ID=chaos-world-analytics
BIGQUERY_DATASET=analytics

# Storage Configuration
S3_BUCKET=chaos-world-analytics
S3_ENDPOINT=https://s3.amazonaws.com
S3_ACCESS_KEY=your-access-key
S3_SECRET_KEY=your-secret-key

# Kafka Configuration
KAFKA_BROKERS=localhost:9092
KAFKA_TOPIC_PREFIX=analytics
KAFKA_GROUP_ID=analytics_service

# ML Configuration
ML_MODEL_PATH=/models
TENSORFLOW_SERVING_URL=http://localhost:8501
PYTORCH_SERVING_URL=http://localhost:8080

# Server Configuration
SERVER_PORT=8084
SERVER_HOST=0.0.0.0
WORKERS=4

# Processing Configuration
BATCH_SIZE=10000
PROCESSING_INTERVAL=300
REAL_TIME_ENABLED=true
```

### Configuration File
```yaml
# analytics-config.yaml
server:
  port: 8084
  host: "0.0.0.0"
  workers: 4

database:
  clickhouse:
    url: "clickhouse://localhost:9000"
    database: "analytics"
    username: "default"
    password: "secret"
    pool_size: 50
  
  bigquery:
    project_id: "chaos-world-analytics"
    dataset: "analytics"
    credentials_path: "/path/to/credentials.json"

storage:
  s3:
    bucket: "chaos-world-analytics"
    endpoint: "https://s3.amazonaws.com"
    access_key: "your-access-key"
    secret_key: "your-secret-key"
    region: "us-east-1"

kafka:
  brokers: ["localhost:9092"]
  topic_prefix: "analytics"
  group_id: "analytics_service"
  auto_offset_reset: "latest"

processing:
  batch_size: 10000
  processing_interval: 300
  real_time_enabled: true
  parallel_workers: 4

ml_models:
  churn_prediction:
    model_path: "/models/churn_prediction"
    version: "v1.0.0"
    serving_url: "http://localhost:8501"
  
  revenue_prediction:
    model_path: "/models/revenue_prediction"
    version: "v1.0.0"
    serving_url: "http://localhost:8501"

dashboards:
  default_time_range: "7d"
  refresh_interval: 300
  max_widgets: 50
  cache_ttl: 3600

alerts:
  enabled: true
  channels: ["email", "slack", "webhook"]
  thresholds:
    error_rate: 0.05
    response_time: 1000
    availability: 0.99
```

## 🔗 Related Services

- [Chaos Backend](./chaos-backend/README.md) - Game Logic Core
- [User Management](./user-management/README.md) - User data
- [Payment Service](./payment-service/README.md) - Transaction data
- [Anti-Cheat Service](./anti-cheat-service/README.md) - Security data
- [Microservices Architecture](./microservices-architecture/README.md) - Overall Architecture
