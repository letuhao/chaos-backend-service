#!/usr/bin/env python3
"""
Quick Monitoring Setup
Provides step-by-step instructions for manual setup
"""

import requests
import time

def check_services():
    """Check if all services are running"""
    print("🔍 Checking services...")
    
    services = [
        ("Prometheus", "http://localhost:9091"),
        ("Grafana", "http://localhost:3001"),
        ("CMS Service", "http://localhost:8083/health"),
        ("CMS Metrics", "http://localhost:9090/metrics")
    ]
    
    all_running = True
    for name, url in services:
        try:
            response = requests.get(url, timeout=5)
            if response.status_code == 200:
                print(f"✅ {name}: Running")
            else:
                print(f"⚠️ {name}: Status {response.status_code}")
                all_running = False
        except Exception as e:
            print(f"❌ {name}: Not accessible - {e}")
            all_running = False
    
    return all_running

def show_setup_instructions():
    """Show manual setup instructions"""
    print("\n" + "="*60)
    print("🎯 MANUAL MONITORING SETUP INSTRUCTIONS")
    print("="*60)
    
    print("\n📋 STEP 1: Access Grafana")
    print("   Open your browser and go to: http://localhost:3001")
    print("   Try these login combinations:")
    print("   • admin / admin")
    print("   • admin / admin123") 
    print("   • admin / Ab123456")
    print("   • admin / password")
    
    print("\n📋 STEP 2: Set Up Prometheus Datasource")
    print("   1. In Grafana, go to Configuration → Data Sources")
    print("   2. Click 'Add data source'")
    print("   3. Select 'Prometheus'")
    print("   4. Set URL to: http://localhost:9091")
    print("   5. Click 'Save & Test'")
    print("   6. You should see 'Data source is working'")
    
    print("\n📋 STEP 3: Create Your First Dashboard")
    print("   1. Go to Dashboards → New → New Dashboard")
    print("   2. Click 'Add Panel'")
    print("   3. In the query box, try these metrics:")
    print("")
    print("   🔹 Service Status:")
    print("      up{job=~\"chaos-world-.*\"}")
    print("")
    print("   🔹 Request Rate:")
    print("      rate(cms_request_duration_seconds_count[5m])")
    print("")
    print("   🔹 Response Time:")
    print("      histogram_quantile(0.95, rate(cms_request_duration_seconds_bucket[5m]))")
    print("")
    print("   🔹 Error Rate:")
    print("      rate(cms_errors_total[5m])")
    print("")
    print("   🔹 Active Connections:")
    print("      cms_active_connections")
    
    print("\n📋 STEP 4: Generate Test Data")
    print("   Run this command to create some test traffic:")
    print("   python generate_test_data.py")
    
    print("\n📋 STEP 5: Explore Prometheus")
    print("   • Check targets: http://localhost:9091/targets")
    print("   • Browse metrics: http://localhost:9091/graph")
    print("   • Search for 'cms_' to see your service metrics")

def show_available_metrics():
    """Show available metrics from CMS service"""
    print("\n📊 Available CMS Metrics:")
    print("-" * 30)
    
    try:
        response = requests.get("http://localhost:9090/metrics", timeout=5)
        if response.status_code == 200:
            lines = response.text.split('\n')
            cms_metrics = [line for line in lines if line.startswith('cms_') and not line.startswith('#')]
            
            print(f"Found {len(cms_metrics)} CMS metrics:")
            for metric in cms_metrics[:10]:  # Show first 10
                print(f"  • {metric}")
            
            if len(cms_metrics) > 10:
                print(f"  ... and {len(cms_metrics) - 10} more")
        else:
            print("❌ Could not fetch metrics")
    except Exception as e:
        print(f"❌ Error fetching metrics: {e}")

def main():
    """Main function"""
    print("🚀 Quick Monitoring Setup")
    print("=" * 30)
    
    # Check services
    if not check_services():
        print("\n⚠️ Some services are not running properly")
        print("Make sure all services are started before proceeding")
        return
    
    # Show available metrics
    show_available_metrics()
    
    # Show setup instructions
    show_setup_instructions()
    
    print("\n" + "="*60)
    print("🎉 You're all set! Follow the steps above to complete setup.")
    print("💡 Need help? Check the MONITORING_SETUP_GUIDE.md file")
    print("="*60)

if __name__ == "__main__":
    main()
