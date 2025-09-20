#!/usr/bin/env python3
"""
Verify Monitoring Setup
Check what's available in Grafana and Prometheus
"""

import requests
import json
import time

def check_prometheus_targets():
    """Check Prometheus targets"""
    print("🔍 Checking Prometheus targets...")
    try:
        response = requests.get("http://localhost:9091/api/v1/targets", timeout=10)
        if response.status_code == 200:
            data = response.json()
            targets = data['data']['activeTargets']
            
            print(f"📊 Found {len(targets)} targets:")
            for target in targets:
                status = "✅ UP" if target['health'] == 'up' else "❌ DOWN"
                print(f"  {status} {target['labels']['job']} - {target['scrapeUrl']}")
            
            return len([t for t in targets if t['health'] == 'up'])
        else:
            print(f"❌ Prometheus API error: {response.status_code}")
            return 0
    except Exception as e:
        print(f"❌ Prometheus error: {e}")
        return 0

def check_prometheus_metrics():
    """Check available metrics in Prometheus"""
    print("\n📈 Checking Prometheus metrics...")
    try:
        response = requests.get("http://localhost:9091/api/v1/label/__name__/values", timeout=10)
        if response.status_code == 200:
            data = response.json()
            metrics = data['data']
            
            print(f"📊 Found {len(metrics)} metrics:")
            for metric in sorted(metrics)[:20]:  # Show first 20
                print(f"  • {metric}")
            
            if len(metrics) > 20:
                print(f"  ... and {len(metrics) - 20} more")
            
            return len(metrics)
        else:
            print(f"❌ Prometheus metrics API error: {response.status_code}")
            return 0
    except Exception as e:
        print(f"❌ Prometheus metrics error: {e}")
        return 0

def check_grafana_dashboards():
    """Check Grafana dashboards"""
    print("\n📊 Checking Grafana dashboards...")
    try:
        response = requests.get("http://localhost:3001/api/search?type=dash-db", 
                              auth=("admin", "Ab123456"), timeout=10)
        if response.status_code == 200:
            dashboards = response.json()
            
            print(f"📈 Found {len(dashboards)} dashboards:")
            for dashboard in dashboards:
                print(f"  • {dashboard['title']} (ID: {dashboard['id']})")
            
            return len(dashboards)
        else:
            print(f"❌ Grafana API error: {response.status_code}")
            return 0
    except Exception as e:
        print(f"❌ Grafana error: {e}")
        return 0

def check_grafana_datasources():
    """Check Grafana datasources"""
    print("\n🔗 Checking Grafana datasources...")
    try:
        response = requests.get("http://localhost:3001/api/datasources", 
                              auth=("admin", "Ab123456"), timeout=10)
        if response.status_code == 200:
            datasources = response.json()
            
            print(f"📊 Found {len(datasources)} datasources:")
            for ds in datasources:
                print(f"  • {ds['name']} ({ds['type']}) - {ds['url']}")
            
            return len(datasources)
        else:
            print(f"❌ Grafana datasources API error: {response.status_code}")
            return 0
    except Exception as e:
        print(f"❌ Grafana datasources error: {e}")
        return 0

def check_cms_metrics():
    """Check CMS metrics directly"""
    print("\n🔍 Checking CMS metrics...")
    try:
        response = requests.get("http://localhost:9090/metrics", timeout=10)
        if response.status_code == 200:
            metrics_text = response.text
            lines = metrics_text.split('\n')
            metric_lines = [line for line in lines if not line.startswith('#') and line.strip()]
            
            print(f"📊 CMS exposing {len(metric_lines)} metrics:")
            for line in metric_lines[:10]:  # Show first 10
                print(f"  • {line}")
            
            if len(metric_lines) > 10:
                print(f"  ... and {len(metric_lines) - 10} more")
            
            return len(metric_lines)
        else:
            print(f"❌ CMS metrics error: {response.status_code}")
            return 0
    except Exception as e:
        print(f"❌ CMS metrics error: {e}")
        return 0

def main():
    """Main verification function"""
    print("🧪 Chaos World Monitoring Verification")
    print("=" * 50)
    
    # Check all components
    targets_up = check_prometheus_targets()
    metrics_count = check_prometheus_metrics()
    dashboards_count = check_grafana_dashboards()
    datasources_count = check_grafana_datasources()
    cms_metrics_count = check_cms_metrics()
    
    print("\n📋 Summary:")
    print(f"  Prometheus targets UP: {targets_up}")
    print(f"  Prometheus metrics: {metrics_count}")
    print(f"  Grafana dashboards: {dashboards_count}")
    print(f"  Grafana datasources: {datasources_count}")
    print(f"  CMS metrics: {cms_metrics_count}")
    
    if targets_up > 0 and metrics_count > 0 and dashboards_count > 0:
        print("\n✅ Monitoring stack is working!")
        print("📈 Open Grafana: http://localhost:3001")
        print("📊 Open Prometheus: http://localhost:9091")
        print("\n🎯 Next steps:")
        print("  1. Explore the pre-configured dashboards")
        print("  2. Create custom dashboards for your needs")
        print("  3. Set up alerts and notifications")
        print("  4. Add more services as you build them")
    else:
        print("\n⚠️ Some components need attention")
        if targets_up == 0:
            print("  • Check Prometheus targets configuration")
        if metrics_count == 0:
            print("  • Check if services are exposing metrics")
        if dashboards_count == 0:
            print("  • Check Grafana dashboard provisioning")

if __name__ == "__main__":
    main()
