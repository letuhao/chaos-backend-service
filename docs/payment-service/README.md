# Payment Service - Chaos World

## 📋 Overview

Payment Service chịu trách nhiệm xử lý tất cả giao dịch tài chính, thanh toán, và quản lý ví điện tử cho Chaos World MMORPG.

## 🎯 Responsibilities

### Core Functions
- **Payment Processing**: Xử lý thanh toán từ nhiều nguồn
- **Transaction Management**: Quản lý lịch sử giao dịch
- **Wallet Management**: Quản lý ví điện tử của người chơi
- **Billing & Invoicing**: Tạo hóa đơn và quản lý billing
- **Refund Processing**: Xử lý hoàn tiền
- **Compliance**: Tuân thủ các quy định tài chính (PCI DSS)

### Performance Requirements
- **Latency**: < 500ms cho payment processing
- **Throughput**: 100+ TPS
- **Availability**: 99.99% uptime (critical service)
- **Concurrency**: Handle 1,000+ concurrent transactions

## 🏗️ Architecture

### Technology Stack
```yaml
Language: Rust/Go
Framework: Axum/Gin
Database: PostgreSQL (ACID compliance)
Caching: Redis
Payment Gateway: Stripe, PayPal, Razorpay
Encryption: AES-256, TLS 1.3
```

### Core Components
```rust
pub struct PaymentService {
    // Database
    transaction_repository: Arc<TransactionRepository>,
    wallet_repository: Arc<WalletRepository>,
    billing_repository: Arc<BillingRepository>,
    
    // Payment gateways
    stripe_client: Arc<StripeClient>,
    paypal_client: Arc<PayPalClient>,
    razorpay_client: Arc<RazorpayClient>,
    
    // Security
    encryption_service: Arc<EncryptionService>,
    fraud_detection: Arc<FraudDetectionService>,
    
    // External services
    notification_service: Arc<NotificationService>,
    audit_service: Arc<AuditService>,
    
    // Configuration
    config: PaymentServiceConfig,
}
```

## 💳 Payment Methods

### Supported Payment Methods
```yaml
Credit/Debit Cards:
  - Visa, Mastercard, American Express
  - 3D Secure authentication
  - Tokenization for security

Digital Wallets:
  - PayPal
  - Apple Pay
  - Google Pay
  - Samsung Pay

Bank Transfers:
  - ACH (US)
  - SEPA (EU)
  - Wire transfers

Cryptocurrency:
  - Bitcoin
  - Ethereum
  - USDC, USDT

Gift Cards:
  - Prepaid cards
  - Store credit
  - Promotional credits
```

### Payment Flow
```mermaid
sequenceDiagram
    participant C as Client
    participant AG as API Gateway
    participant PS as Payment Service
    participant PG as Payment Gateway
    participant CB as Chaos Backend
    participant DB as Database

    C->>AG: POST /shop/purchase
    AG->>PS: Process payment
    PS->>PS: Validate payment data
    PS->>PG: Charge payment method
    PG-->>PS: Payment result
    PS->>DB: Create transaction record
    PS->>CB: Add item to inventory
    CB-->>PS: Item added confirmation
    PS->>PS: Send confirmation email
    PS-->>AG: Payment success
    AG-->>C: Purchase success
```

## 🗄️ Database Schema

### PostgreSQL Tables
```sql
-- Transactions table
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    transaction_type VARCHAR(20) NOT NULL, -- 'purchase', 'refund', 'transfer'
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    status VARCHAR(20) NOT NULL, -- 'pending', 'completed', 'failed', 'refunded'
    payment_method VARCHAR(50) NOT NULL,
    payment_gateway VARCHAR(50) NOT NULL,
    gateway_transaction_id VARCHAR(255),
    gateway_response JSONB,
    description TEXT,
    metadata JSONB,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    completed_at TIMESTAMP,
    expires_at TIMESTAMP
);

-- Wallets table
CREATE TABLE wallets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID UNIQUE NOT NULL,
    balance DECIMAL(15,2) DEFAULT 0.00,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    status VARCHAR(20) DEFAULT 'active', -- 'active', 'suspended', 'closed'
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    last_transaction_at TIMESTAMP
);

-- Wallet transactions table
CREATE TABLE wallet_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wallet_id UUID REFERENCES wallets(id) ON DELETE CASCADE,
    transaction_id UUID REFERENCES transactions(id),
    amount DECIMAL(15,2) NOT NULL,
    transaction_type VARCHAR(20) NOT NULL, -- 'credit', 'debit'
    description TEXT,
    balance_after DECIMAL(15,2) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Billing addresses table
CREATE TABLE billing_addresses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    company VARCHAR(100),
    address_line_1 VARCHAR(255) NOT NULL,
    address_line_2 VARCHAR(255),
    city VARCHAR(100) NOT NULL,
    state VARCHAR(100),
    postal_code VARCHAR(20) NOT NULL,
    country VARCHAR(2) NOT NULL,
    phone VARCHAR(20),
    is_default BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Payment methods table
CREATE TABLE payment_methods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    payment_type VARCHAR(20) NOT NULL, -- 'card', 'paypal', 'bank', 'crypto'
    gateway_customer_id VARCHAR(255),
    gateway_payment_method_id VARCHAR(255),
    card_last_four VARCHAR(4),
    card_brand VARCHAR(20),
    card_exp_month INTEGER,
    card_exp_year INTEGER,
    is_default BOOLEAN DEFAULT FALSE,
    is_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

### Redis Cache Structure
```yaml
Wallet Balance:
  key: "wallet:{user_id}"
  value: { balance, currency, last_updated }
  ttl: 300 seconds

Transaction Status:
  key: "transaction:{transaction_id}"
  value: { status, amount, gateway_response }
  ttl: 3600 seconds

Rate Limiting:
  key: "rate_limit:payment:{user_id}"
  value: { count, window_start }
  ttl: 3600 seconds
```

## 🔌 API Endpoints

### Payment Endpoints
```yaml
POST /payments/process:
  Description: Process payment
  Request: { 
    amount, 
    currency, 
    payment_method_id, 
    items, 
    billing_address 
  }
  Response: { 
    transaction_id, 
    status, 
    gateway_response,
    redirect_url 
  }
  Rate Limit: 10/minute per user

POST /payments/confirm:
  Description: Confirm payment (for 3D Secure)
  Request: { transaction_id, payment_intent_id }
  Response: { status, transaction_id }
  Rate Limit: 20/minute per user

GET /payments/{transaction_id}:
  Description: Get transaction details
  Request: { transaction_id }
  Response: { 
    transaction_id, 
    status, 
    amount, 
    currency, 
    created_at,
    items 
  }
  Rate Limit: 1000/hour per user
```

### Wallet Endpoints
```yaml
GET /wallet/balance:
  Description: Get wallet balance
  Request: { token }
  Response: { balance, currency, last_updated }
  Rate Limit: 1000/hour per user

POST /wallet/add-funds:
  Description: Add funds to wallet
  Request: { amount, payment_method_id }
  Response: { transaction_id, status }
  Rate Limit: 5/minute per user

POST /wallet/withdraw:
  Description: Withdraw funds from wallet
  Request: { amount, bank_account }
  Response: { transaction_id, status }
  Rate Limit: 3/minute per user

GET /wallet/transactions:
  Description: Get wallet transaction history
  Request: { page, limit, filter }
  Response: { transactions: [...], total, page, limit }
  Rate Limit: 1000/hour per user
```

### Refund Endpoints
```yaml
POST /refunds/create:
  Description: Create refund request
  Request: { transaction_id, amount, reason }
  Response: { refund_id, status }
  Rate Limit: 5/minute per user

GET /refunds/{refund_id}:
  Description: Get refund status
  Request: { refund_id }
  Response: { refund_id, status, amount, reason }
  Rate Limit: 1000/hour per user

POST /refunds/process:
  Description: Process refund (admin only)
  Request: { refund_id, admin_notes }
  Response: { success: true }
  Rate Limit: 100/hour per admin
```

### Payment Methods Endpoints
```yaml
GET /payment-methods:
  Description: Get user's payment methods
  Request: { token }
  Response: { payment_methods: [...] }
  Rate Limit: 1000/hour per user

POST /payment-methods:
  Description: Add new payment method
  Request: { 
    payment_type, 
    card_number, 
    expiry_month, 
    expiry_year, 
    cvv,
    billing_address 
  }
  Response: { payment_method_id, status }
  Rate Limit: 5/minute per user

DELETE /payment-methods/{id}:
  Description: Remove payment method
  Request: { payment_method_id }
  Response: { success: true }
  Rate Limit: 10/minute per user
```

## 🔒 Security & Compliance

### PCI DSS Compliance
```yaml
Data Protection:
  - Card data never stored in plain text
  - Tokenization for card numbers
  - Encryption at rest and in transit
  - Secure key management

Access Control:
  - Role-based access control
  - Multi-factor authentication
  - Audit logging
  - Regular security assessments

Network Security:
  - TLS 1.3 encryption
  - Firewall protection
  - Intrusion detection
  - Regular penetration testing
```

### Fraud Detection
```yaml
Risk Assessment:
  - Transaction amount analysis
  - Velocity checks
  - Geographic analysis
  - Device fingerprinting
  - Behavioral analysis

Machine Learning:
  - Anomaly detection
  - Pattern recognition
  - Risk scoring
  - Real-time decisioning

Manual Review:
  - High-risk transaction flagging
  - Manual verification process
  - Customer verification
  - Documentation requirements
```

### Data Encryption
```yaml
Encryption Standards:
  - AES-256 for data at rest
  - TLS 1.3 for data in transit
  - RSA-2048 for key exchange
  - SHA-256 for hashing

Key Management:
  - Hardware Security Modules (HSM)
  - Key rotation policies
  - Secure key storage
  - Access logging
```

## 📊 Monitoring & Analytics

### Financial Metrics
```yaml
Transaction Metrics:
  - Total transaction volume
  - Success rate by payment method
  - Average transaction value
  - Refund rate
  - Chargeback rate

Revenue Metrics:
  - Daily/monthly revenue
  - Revenue by payment method
  - Revenue by region
  - Growth rate
  - Conversion rate

Operational Metrics:
  - Payment processing time
  - Gateway response time
  - Error rate by gateway
  - System availability
  - Queue depth
```

### Security Monitoring
```yaml
Fraud Detection:
  - Suspicious transaction alerts
  - High-risk payment patterns
  - Unusual user behavior
  - Failed authentication attempts
  - Geographic anomalies

Compliance Monitoring:
  - PCI DSS compliance status
  - Data access logging
  - Security event monitoring
  - Vulnerability assessments
  - Penetration test results
```

### Business Intelligence
```yaml
Customer Analytics:
  - Payment method preferences
  - Spending patterns
  - Geographic distribution
  - Customer lifetime value
  - Churn analysis

Product Analytics:
  - Most purchased items
  - Revenue by product category
  - Seasonal trends
  - Price sensitivity analysis
  - Bundle performance
```

## 🧪 Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_payment_processing() {
        // Test payment processing flow
    }
    
    #[tokio::test]
    async fn test_wallet_operations() {
        // Test wallet credit/debit operations
    }
    
    #[tokio::test]
    async fn test_fraud_detection() {
        // Test fraud detection logic
    }
    
    #[tokio::test]
    async fn test_refund_processing() {
        // Test refund processing flow
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_complete_payment_flow() {
    // Test complete payment flow with mock gateway
    let client = reqwest::Client::new();
    
    // Create payment
    let payment_response = client
        .post("http://localhost:8082/payments/process")
        .json(&PaymentRequest {
            amount: 1000,
            currency: "USD",
            payment_method_id: "pm_test_123",
            items: vec![Item {
                id: "item_1",
                name: "Gold Coins",
                quantity: 100,
                price: 1000,
            }],
        })
        .send()
        .await
        .unwrap();
    
    assert_eq!(payment_response.status(), 200);
    
    // Verify transaction in database
    let transaction = get_transaction(&payment_id).await.unwrap();
    assert_eq!(transaction.status, "completed");
}
```

### Load Tests
```yaml
# k6 load test
import http from 'k6/http';
import { check } from 'k6';

export let options = {
  stages: [
    { duration: '2m', target: 50 },
    { duration: '5m', target: 50 },
    { duration: '2m', target: 100 },
    { duration: '5m', target: 100 },
    { duration: '2m', target: 0 },
  ],
};

export default function() {
  let response = http.post('http://localhost:8082/payments/process', {
    amount: 1000,
    currency: 'USD',
    payment_method_id: 'pm_test_123',
    items: [{
      id: 'item_1',
      name: 'Gold Coins',
      quantity: 100,
      price: 1000
    }]
  });
  
  check(response, {
    'status is 200': (r) => r.status === 200,
    'response time < 500ms': (r) => r.timings.duration < 500,
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
COPY --from=builder /app/target/release/payment-service /usr/local/bin/
EXPOSE 8082
CMD ["payment-service"]
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: payment-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: payment-service
  template:
    metadata:
      labels:
        app: payment-service
    spec:
      containers:
      - name: payment-service
        image: payment-service:latest
        ports:
        - containerPort: 8082
        env:
        - name: DATABASE_URL
          value: "postgresql://user:pass@postgres:5432/payment_db"
        - name: REDIS_URL
          value: "redis://redis:6379"
        - name: STRIPE_SECRET_KEY
          valueFrom:
            secretKeyRef:
              name: stripe-secret
              key: secret-key
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
```

### Database Migration
```sql
-- Migration script for payment service
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create tables
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    transaction_type VARCHAR(20) NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    status VARCHAR(20) NOT NULL,
    payment_method VARCHAR(50) NOT NULL,
    payment_gateway VARCHAR(50) NOT NULL,
    gateway_transaction_id VARCHAR(255),
    gateway_response JSONB,
    description TEXT,
    metadata JSONB,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    completed_at TIMESTAMP,
    expires_at TIMESTAMP
);

-- Create indexes
CREATE INDEX idx_transactions_user_id ON transactions(user_id);
CREATE INDEX idx_transactions_status ON transactions(status);
CREATE INDEX idx_transactions_created_at ON transactions(created_at);
CREATE INDEX idx_transactions_gateway_id ON transactions(gateway_transaction_id);

-- Create triggers
CREATE TRIGGER update_transactions_updated_at 
    BEFORE UPDATE ON transactions 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();
```

## 🔧 Configuration

### Environment Variables
```yaml
# Database Configuration
DATABASE_URL=postgresql://user:pass@localhost:5432/payment_db
DATABASE_POOL_SIZE=20
DATABASE_TIMEOUT=30s

# Redis Configuration
REDIS_URL=redis://localhost:6379
REDIS_PASSWORD=secret
REDIS_DB=1

# Payment Gateway Configuration
STRIPE_SECRET_KEY=sk_test_...
STRIPE_PUBLISHABLE_KEY=pk_test_...
PAYPAL_CLIENT_ID=your_paypal_client_id
PAYPAL_CLIENT_SECRET=your_paypal_secret
RAZORPAY_KEY_ID=your_razorpay_key_id
RAZORPAY_KEY_SECRET=your_razorpay_secret

# Security Configuration
ENCRYPTION_KEY=your-256-bit-encryption-key
JWT_SECRET=your-jwt-secret
HSM_ENDPOINT=https://hsm.example.com

# Server Configuration
SERVER_PORT=8082
SERVER_HOST=0.0.0.0
SERVER_WORKERS=4

# Fraud Detection
FRAUD_DETECTION_ENABLED=true
FRAUD_THRESHOLD=0.8
ML_MODEL_ENDPOINT=https://ml.example.com/predict
```

### Configuration File
```yaml
# payment-service-config.yaml
server:
  port: 8082
  host: "0.0.0.0"
  workers: 4
  max_connections: 1000

database:
  url: "postgresql://user:pass@localhost:5432/payment_db"
  pool_size: 20
  timeout: "30s"
  ssl_mode: "require"

redis:
  url: "redis://localhost:6379"
  password: "secret"
  db: 1
  pool_size: 100

payment_gateways:
  stripe:
    secret_key: "sk_test_..."
    publishable_key: "pk_test_..."
    webhook_secret: "whsec_..."
    enabled: true
  
  paypal:
    client_id: "your_paypal_client_id"
    client_secret: "your_paypal_secret"
    sandbox: true
    enabled: true
  
  razorpay:
    key_id: "your_razorpay_key_id"
    key_secret: "your_razorpay_secret"
    enabled: true

security:
  encryption_key: "your-256-bit-encryption-key"
  jwt_secret: "your-jwt-secret"
  hsm_endpoint: "https://hsm.example.com"

fraud_detection:
  enabled: true
  threshold: 0.8
  ml_model_endpoint: "https://ml.example.com/predict"
  manual_review_threshold: 0.6

compliance:
  pci_dss_enabled: true
  data_retention_days: 2555  # 7 years
  audit_logging: true
```

## 🔗 Related Services

- [API Gateway](./api-gateway/README.md) - Entry point for all requests
- [User Management](./user-management/README.md) - User authentication
- [Chaos Backend](./chaos-backend/README.md) - Game Logic Core
- [Microservices Architecture](./microservices-architecture/README.md) - Overall Architecture
