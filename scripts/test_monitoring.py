#!/usr/bin/env python3
"""
Test Monitoring Stack
Verify that Grafana and Prometheus are running and configure datasource
"""

import requests
import time
import json

def test_prometheus():
    """Test Prometheus connectivity"""
    print("🔍 Testing Prometheus...")
    try:
        response = requests.get("http://localhost:9091", timeout=10)
        if response.status_code == 200:
            print("✅ Prometheus is running on port 9091")
            return True
        else:
            print(f"❌ Prometheus returned status {response.status_code}")
            return False
    except Exception as e:
        print(f"❌ Prometheus error: {e}")
        return False

def test_grafana():
    """Test Grafana connectivity"""
    print("🔍 Testing Grafana...")
    try:
        response = requests.get("http://localhost:3001", timeout=10)
        if response.status_code == 200:
            print("✅ Grafana is running on port 3001")
            return True
        else:
            print(f"❌ Grafana returned status {response.status_code}")
            return False
    except Exception as e:
        print(f"❌ Grafana error: {e}")
        return False

def setup_grafana_datasource():
    """Configure Grafana to use Prometheus as datasource"""
    print("🔗 Setting up Grafana datasource...")
    
    # Wait for Grafana to be fully ready
    time.sleep(5)
    
    datasource_config = {
        "name": "Prometheus",
        "type": "prometheus",
        "url": "http://localhost:9091",
        "access": "proxy",
        "isDefault": True
    }
    
    try:
        # Create datasource
        response = requests.post(
            "http://localhost:3001/api/datasources",
            json=datasource_config,
            headers={"Content-Type": "application/json"},
            auth=("admin", "admin")  # Default Grafana credentials
        )
        
        if response.status_code in [200, 409]:  # 409 = already exists
            print("✅ Grafana datasource configured")
            return True
        else:
            print(f"⚠️ Could not configure datasource: {response.status_code}")
            print(f"Response: {response.text}")
            return False
    except Exception as e:
        print(f"⚠️ Could not configure datasource: {e}")
        return False

def test_cms_metrics():
    """Test if CMS metrics are accessible"""
    print("🔍 Testing CMS metrics...")
    try:
        response = requests.get("http://localhost:9090/metrics", timeout=5)
        if response.status_code == 200:
            print("✅ CMS metrics are accessible on port 9090")
            return True
        else:
            print(f"❌ CMS metrics returned status {response.status_code}")
            return False
    except Exception as e:
        print(f"❌ CMS metrics error: {e}")
        return False

def main():
    """Main test function"""
    print("🧪 Testing Chaos World Monitoring Stack")
    print("=" * 50)
    
    # Test all services
    prometheus_ok = test_prometheus()
    grafana_ok = test_grafana()
    cms_metrics_ok = test_cms_metrics()
    
    if grafana_ok:
        setup_grafana_datasource()
    
    print("\n📊 Monitoring Status:")
    print(f"  Prometheus (9091): {'✅ Running' if prometheus_ok else '❌ Failed'}")
    print(f"  Grafana (3001): {'✅ Running' if grafana_ok else '❌ Failed'}")
    print(f"  CMS Metrics (9090): {'✅ Running' if cms_metrics_ok else '❌ Failed'}")
    
    if prometheus_ok and grafana_ok:
        print("\n🎉 Monitoring stack is ready!")
        print("📈 Access Grafana: http://localhost:3001")
        print("📊 Access Prometheus: http://localhost:9091")
        print("🔑 Grafana login: admin / admin")
        print("\nNext steps:")
        print("1. Open Grafana and create dashboards")
        print("2. Add your services to Prometheus targets")
        print("3. Set up alerts and notifications")
    else:
        print("\n❌ Some services are not running properly")

if __name__ == "__main__":
    main()
