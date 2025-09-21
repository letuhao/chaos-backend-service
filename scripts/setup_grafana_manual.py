#!/usr/bin/env python3
"""
Manual Grafana Setup
Set up datasources and dashboards manually via API
"""

import requests
import json
import time

def wait_for_grafana():
    """Wait for Grafana to be ready"""
    print("⏳ Waiting for Grafana to be ready...")
    for i in range(30):  # Wait up to 30 seconds
        try:
            response = requests.get("http://localhost:3001", timeout=5)
            if response.status_code == 200:
                print("✅ Grafana is ready!")
                return True
        except:
            pass
        time.sleep(1)
        print(f"   Waiting... ({i+1}/30)")
    
    print("❌ Grafana not ready after 30 seconds")
    return False

def setup_datasource():
    """Set up Prometheus datasource"""
    print("🔗 Setting up Prometheus datasource...")
    
    datasource_config = {
        "name": "Prometheus",
        "type": "prometheus",
        "url": "http://localhost:9091",
        "access": "proxy",
        "isDefault": True,
        "basicAuth": False
    }
    
    try:
        # Check if datasource already exists
        response = requests.get("http://localhost:3001/api/datasources", 
                              auth=("admin", "admin123"), timeout=10)
        
        if response.status_code == 200:
            existing_ds = [ds for ds in response.json() if ds['name'] == 'Prometheus']
            if existing_ds:
                print("✅ Prometheus datasource already exists")
                return True
        
        # Create datasource
        response = requests.post("http://localhost:3001/api/datasources",
                               json=datasource_config,
                               auth=("admin", "admin123"),
                               headers={"Content-Type": "application/json"},
                               timeout=10)
        
        if response.status_code in [200, 409]:  # 409 = already exists
            print("✅ Prometheus datasource configured")
            return True
        else:
            print(f"❌ Failed to create datasource: {response.status_code}")
            print(f"Response: {response.text}")
            return False
            
    except Exception as e:
        print(f"❌ Error setting up datasource: {e}")
        return False

def create_simple_dashboard():
    """Create a simple dashboard"""
    print("📊 Creating simple dashboard...")
    
    dashboard = {
        "dashboard": {
            "id": None,
            "title": "Chaos World - Simple Overview",
            "tags": ["chaos-world"],
            "timezone": "browser",
            "panels": [
                {
                    "id": 1,
                    "title": "Service Status",
                    "type": "stat",
                    "targets": [
                        {
                            "expr": "up{job=~\"chaos-world-.*\"}",
                            "legendFormat": "{{job}}"
                        }
                    ],
                    "fieldConfig": {
                        "defaults": {
                            "color": {
                                "mode": "thresholds"
                            },
                            "thresholds": {
                                "steps": [
                                    {"color": "red", "value": 0},
                                    {"color": "green", "value": 1}
                                ]
                            },
                            "mappings": [
                                {"type": "value", "value": "0", "text": "DOWN"},
                                {"type": "value", "value": "1", "text": "UP"}
                            ]
                        }
                    },
                    "gridPos": {"h": 8, "w": 12, "x": 0, "y": 0}
                },
                {
                    "id": 2,
                    "title": "Request Rate",
                    "type": "graph",
                    "targets": [
                        {
                            "expr": "rate(cms_request_duration_seconds_count[5m])",
                            "legendFormat": "CMS Requests/sec"
                        },
                        {
                            "expr": "rate(user_management_request_duration_seconds_count[5m])",
                            "legendFormat": "User Management Requests/sec"
                        }
                    ],
                    "gridPos": {"h": 8, "w": 12, "x": 12, "y": 0}
                },
                {
                    "id": 3,
                    "title": "Response Time",
                    "type": "graph",
                    "targets": [
                        {
                            "expr": "histogram_quantile(0.95, rate(cms_request_duration_seconds_bucket[5m]))",
                            "legendFormat": "95th percentile"
                        },
                        {
                            "expr": "histogram_quantile(0.50, rate(cms_request_duration_seconds_bucket[5m]))",
                            "legendFormat": "50th percentile"
                        }
                    ],
                    "gridPos": {"h": 8, "w": 24, "x": 0, "y": 8}
                },
                {
                    "id": 4,
                    "title": "User Management Metrics",
                    "type": "graph",
                    "targets": [
                        {
                            "expr": "rate(user_management_http_requests_total[5m])",
                            "legendFormat": "User Management HTTP Requests/sec"
                        },
                        {
                            "expr": "rate(user_management_auth_attempts_total[5m])",
                            "legendFormat": "Authentication Attempts/sec"
                        },
                        {
                            "expr": "rate(user_management_registrations_total[5m])",
                            "legendFormat": "User Registrations/sec"
                        }
                    ],
                    "gridPos": {"h": 8, "w": 24, "x": 0, "y": 16}
                }
            ],
            "time": {
                "from": "now-1h",
                "to": "now"
            },
            "refresh": "5s"
        }
    }
    
    try:
        response = requests.post("http://localhost:3001/api/dashboards/db",
                               json=dashboard,
                               auth=("admin", "admin123"),
                               headers={"Content-Type": "application/json"},
                               timeout=10)
        
        if response.status_code in [200, 201]:
            print("✅ Simple dashboard created")
            return True
        else:
            print(f"❌ Failed to create dashboard: {response.status_code}")
            print(f"Response: {response.text}")
            return False
            
    except Exception as e:
        print(f"❌ Error creating dashboard: {e}")
        return False

def test_grafana_access():
    """Test basic Grafana access"""
    print("🔍 Testing Grafana access...")
    
    try:
        # Test basic access
        response = requests.get("http://localhost:3001", timeout=10)
        if response.status_code != 200:
            print(f"❌ Grafana not accessible: {response.status_code}")
            return False
        
        # Test API access
        response = requests.get("http://localhost:3001/api/health", timeout=10)
        if response.status_code == 200:
            print("✅ Grafana API accessible")
            return True
        else:
            print(f"⚠️ Grafana API returned: {response.status_code}")
            return False
            
    except Exception as e:
        print(f"❌ Grafana access error: {e}")
        return False

def main():
    """Main setup function"""
    print("🔧 Manual Grafana Setup")
    print("=" * 30)
    
    # Wait for Grafana
    if not wait_for_grafana():
        return False
    
    # Test access
    if not test_grafana_access():
        print("⚠️ Grafana access issues, but continuing...")
    
    # Setup datasource
    setup_datasource()
    
    # Create dashboard
    create_simple_dashboard()
    
    print("\n🎉 Manual setup completed!")
    print("📈 Open Grafana: http://localhost:3001")
    print("🔑 Login: admin / admin")
    print("📊 Look for 'Chaos World - Simple Overview' dashboard")
    
    return True

if __name__ == "__main__":
    main()
