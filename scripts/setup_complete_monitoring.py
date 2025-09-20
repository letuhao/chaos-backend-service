#!/usr/bin/env python3
"""
Complete Monitoring Setup for Chaos World
Sets up Prometheus, Grafana, dashboards, and alerting
"""

import os
import sys
import subprocess
import shutil
import json
import time
import requests
from pathlib import Path
import yaml

class CompleteMonitoringSetup:
    def __init__(self):
        self.grafana_path = r"C:\ProgramData\chocolatey\lib\grafana\tools\grafana-11.5.8"
        self.prometheus_path = r"C:\ProgramData\chocolatey\lib\prometheus\tools\prometheus-2.2.1.windows-amd64"
        self.config_dir = Path("C:/ChaosWorld/monitoring")
        self.prometheus_port = 9091
        self.grafana_port = 3001
        
    def create_prometheus_config(self):
        """Create comprehensive Prometheus configuration"""
        print("⚙️ Creating comprehensive Prometheus configuration...")
        
        config = {
            "global": {
                "scrape_interval": "15s",
                "evaluation_interval": "15s",
                "external_labels": {
                    "cluster": "chaos-world",
                    "environment": "development"
                }
            },
            "rule_files": [
                str(self.config_dir / "prometheus" / "rules" / "*.yml")
            ],
            "scrape_configs": [
                {
                    "job_name": "chaos-world-api-gateway",
                    "static_configs": [
                        {
                            "targets": ["localhost:8080"],
                            "labels": {
                                "service": "api-gateway",
                                "environment": "development",
                                "team": "backend"
                            }
                        }
                    ],
                    "metrics_path": "/metrics",
                    "scrape_interval": "5s",
                    "scrape_timeout": "5s"
                },
                {
                    "job_name": "chaos-world-backend",
                    "static_configs": [
                        {
                            "targets": ["localhost:8081"],
                            "labels": {
                                "service": "chaos-backend",
                                "environment": "development",
                                "team": "backend"
                            }
                        }
                    ],
                    "metrics_path": "/metrics",
                    "scrape_interval": "5s",
                    "scrape_timeout": "5s"
                },
                {
                    "job_name": "chaos-world-cms",
                    "static_configs": [
                        {
                            "targets": ["localhost:9090"],
                            "labels": {
                                "service": "content-management",
                                "environment": "development",
                                "team": "backend"
                            }
                        }
                    ],
                    "metrics_path": "/metrics",
                    "scrape_interval": "5s",
                    "scrape_timeout": "5s"
                },
                {
                    "job_name": "prometheus",
                    "static_configs": [
                        {
                            "targets": [f"localhost:{self.prometheus_port}"],
                            "labels": {
                                "service": "prometheus",
                                "environment": "development"
                            }
                        }
                    ],
                    "scrape_interval": "15s"
                }
            ],
            "alerting": {
                "alertmanagers": [
                    {
                        "static_configs": [
                            {
                                "targets": ["localhost:9093"]
                            }
                        ]
                    }
                ]
            }
        }
        
        # Create rules directory
        rules_dir = self.config_dir / "prometheus" / "rules"
        rules_dir.mkdir(parents=True, exist_ok=True)
        
        # Write main config
        config_file = self.config_dir / "prometheus" / "prometheus.yml"
        with open(config_file, 'w') as f:
            yaml.dump(config, f, default_flow_style=False)
        
        print(f"✅ Prometheus config created: {config_file}")
        return config_file
    
    def create_alerting_rules(self):
        """Create Prometheus alerting rules"""
        print("🚨 Creating alerting rules...")
        
        rules_dir = self.config_dir / "prometheus" / "rules"
        
        # Service down alerts
        service_down_rules = {
            "groups": [
                {
                    "name": "chaos-world-services",
                    "rules": [
                        {
                            "alert": "ServiceDown",
                            "expr": "up == 0",
                            "for": "1m",
                            "labels": {
                                "severity": "critical"
                            },
                            "annotations": {
                                "summary": "Service {{ $labels.job }} is down",
                                "description": "Service {{ $labels.job }} has been down for more than 1 minute."
                            }
                        },
                        {
                            "alert": "HighErrorRate",
                            "expr": "rate(cms_errors_total[5m]) > 0.1",
                            "for": "2m",
                            "labels": {
                                "severity": "warning"
                            },
                            "annotations": {
                                "summary": "High error rate in {{ $labels.job }}",
                                "description": "Error rate is {{ $value }} errors per second"
                            }
                        },
                        {
                            "alert": "HighResponseTime",
                            "expr": "histogram_quantile(0.95, rate(cms_request_duration_seconds_bucket[5m])) > 1",
                            "for": "3m",
                            "labels": {
                                "severity": "warning"
                            },
                            "annotations": {
                                "summary": "High response time in {{ $labels.job }}",
                                "description": "95th percentile response time is {{ $value }} seconds"
                            }
                        }
                    ]
                }
            ]
        }
        
        rules_file = rules_dir / "chaos-world-alerts.yml"
        with open(rules_file, 'w') as f:
            yaml.dump(service_down_rules, f, default_flow_style=False)
        
        print(f"✅ Alerting rules created: {rules_file}")
    
    def create_grafana_dashboards(self):
        """Create Grafana dashboards"""
        print("📊 Creating Grafana dashboards...")
        
        dashboards_dir = self.config_dir / "grafana" / "dashboards"
        dashboards_dir.mkdir(parents=True, exist_ok=True)
        
        # Main Chaos World Overview Dashboard
        overview_dashboard = {
            "dashboard": {
                "id": None,
                "title": "Chaos World - Service Overview",
                "tags": ["chaos-world", "overview"],
                "timezone": "browser",
                "panels": [
                    {
                        "id": 1,
                        "title": "Service Status",
                        "type": "stat",
                        "targets": [
                            {
                                "expr": "up",
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
                                }
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
                                "legendFormat": "{{job}} requests/sec"
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
                        "title": "Error Rate",
                        "type": "graph",
                        "targets": [
                            {
                                "expr": "rate(cms_errors_total[5m])",
                                "legendFormat": "{{job}} errors/sec"
                            }
                        ],
                        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 16}
                    },
                    {
                        "id": 5,
                        "title": "Active Connections",
                        "type": "graph",
                        "targets": [
                            {
                                "expr": "cms_active_connections",
                                "legendFormat": "Active Connections"
                            }
                        ],
                        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 16}
                    }
                ],
                "time": {
                    "from": "now-1h",
                    "to": "now"
                },
                "refresh": "5s"
            }
        }
        
        # Save overview dashboard
        overview_file = dashboards_dir / "chaos-world-overview.json"
        with open(overview_file, 'w') as f:
            json.dump(overview_dashboard, f, indent=2)
        
        # CMS Detailed Dashboard
        cms_dashboard = {
            "dashboard": {
                "id": None,
                "title": "Chaos World - CMS Service Details",
                "tags": ["chaos-world", "cms", "detailed"],
                "timezone": "browser",
                "panels": [
                    {
                        "id": 1,
                        "title": "HTTP Request Duration",
                        "type": "graph",
                        "targets": [
                            {
                                "expr": "histogram_quantile(0.99, rate(cms_request_duration_seconds_bucket[5m]))",
                                "legendFormat": "99th percentile"
                            },
                            {
                                "expr": "histogram_quantile(0.95, rate(cms_request_duration_seconds_bucket[5m]))",
                                "legendFormat": "95th percentile"
                            },
                            {
                                "expr": "histogram_quantile(0.50, rate(cms_request_duration_seconds_bucket[5m]))",
                                "legendFormat": "50th percentile"
                            }
                        ],
                        "gridPos": {"h": 8, "w": 24, "x": 0, "y": 0}
                    },
                    {
                        "id": 2,
                        "title": "Database Queries",
                        "type": "graph",
                        "targets": [
                            {
                                "expr": "rate(cms_database_queries_total[5m])",
                                "legendFormat": "Queries/sec"
                            }
                        ],
                        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 8}
                    },
                    {
                        "id": 3,
                        "title": "Cache Performance",
                        "type": "graph",
                        "targets": [
                            {
                                "expr": "rate(cms_cache_hits_total[5m])",
                                "legendFormat": "Cache Hits/sec"
                            },
                            {
                                "expr": "rate(cms_cache_misses_total[5m])",
                                "legendFormat": "Cache Misses/sec"
                            }
                        ],
                        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 8}
                    },
                    {
                        "id": 4,
                        "title": "Error Count",
                        "type": "graph",
                        "targets": [
                            {
                                "expr": "cms_errors_total",
                                "legendFormat": "Total Errors"
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
        
        # Save CMS dashboard
        cms_file = dashboards_dir / "chaos-world-cms.json"
        with open(cms_file, 'w') as f:
            json.dump(cms_dashboard, f, indent=2)
        
        print(f"✅ Dashboards created in: {dashboards_dir}")
    
    def create_grafana_provisioning(self):
        """Create Grafana provisioning configuration"""
        print("🔧 Creating Grafana provisioning...")
        
        provisioning_dir = self.config_dir / "grafana" / "provisioning"
        provisioning_dir.mkdir(parents=True, exist_ok=True)
        
        # Datasources provisioning
        datasources_config = {
            "apiVersion": 1,
            "datasources": [
                {
                    "name": "Prometheus",
                    "type": "prometheus",
                    "url": f"http://localhost:{self.prometheus_port}",
                    "access": "proxy",
                    "isDefault": True,
                    "editable": True
                }
            ]
        }
        
        datasources_file = provisioning_dir / "datasources" / "prometheus.yml"
        datasources_file.parent.mkdir(exist_ok=True)
        with open(datasources_file, 'w') as f:
            yaml.dump(datasources_config, f, default_flow_style=False)
        
        # Dashboards provisioning
        dashboards_config = {
            "apiVersion": 1,
            "providers": [
                {
                    "name": "chaos-world-dashboards",
                    "type": "file",
                    "options": {
                        "path": str(self.config_dir / "grafana" / "dashboards")
                    }
                }
            ]
        }
        
        dashboards_file = provisioning_dir / "dashboards" / "chaos-world.yml"
        dashboards_file.parent.mkdir(exist_ok=True)
        with open(dashboards_file, 'w') as f:
            yaml.dump(dashboards_config, f, default_flow_style=False)
        
        print(f"✅ Grafana provisioning created in: {provisioning_dir}")
    
    def restart_services(self):
        """Restart Prometheus and Grafana with new configuration"""
        print("🔄 Restarting services with new configuration...")
        
        # Kill existing processes
        try:
            subprocess.run(["taskkill", "/F", "/IM", "prometheus.exe"], 
                         capture_output=True, check=False)
            subprocess.run(["taskkill", "/F", "/IM", "grafana-server.exe"], 
                         capture_output=True, check=False)
            time.sleep(3)
        except:
            pass
        
        # Start Prometheus
        prometheus_exe = Path(self.prometheus_path) / "prometheus.exe"
        config_file = self.config_dir / "prometheus" / "prometheus.yml"
        
        cmd = [
            str(prometheus_exe),
            f"--config.file={config_file}",
            f"--web.listen-address=0.0.0.0:{self.prometheus_port}",
            f"--storage.tsdb.path={self.config_dir / 'prometheus' / 'data'}",
            "--web.enable-lifecycle"
        ]
        
        subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        time.sleep(5)
        
        # Start Grafana
        grafana_exe = Path(self.grafana_path) / "bin" / "grafana-server.exe"
        custom_config = self.config_dir / "grafana" / "grafana.ini"
        
        cmd = [
            str(grafana_exe),
            f"--config={custom_config}",
            f"--homepath={self.grafana_path}"
        ]
        
        subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        time.sleep(10)
        
        print("✅ Services restarted")
    
    def test_setup(self):
        """Test the complete monitoring setup"""
        print("🧪 Testing complete monitoring setup...")
        
        # Test Prometheus
        try:
            response = requests.get(f"http://localhost:{self.prometheus_port}", timeout=10)
            if response.status_code == 200:
                print("✅ Prometheus is running")
            else:
                print("❌ Prometheus not responding")
                return False
        except:
            print("❌ Prometheus not accessible")
            return False
        
        # Test Grafana
        try:
            response = requests.get(f"http://localhost:{self.grafana_port}", timeout=10)
            if response.status_code == 200:
                print("✅ Grafana is running")
            else:
                print("❌ Grafana not responding")
                return False
        except:
            print("❌ Grafana not accessible")
            return False
        
        # Test targets in Prometheus
        try:
            response = requests.get(f"http://localhost:{self.prometheus_port}/api/v1/targets", timeout=10)
            if response.status_code == 200:
                targets = response.json()
                active_targets = [t for t in targets['data']['activeTargets'] if t['health'] == 'up']
                print(f"✅ Prometheus has {len(active_targets)} active targets")
            else:
                print("⚠️ Could not check Prometheus targets")
        except:
            print("⚠️ Could not check Prometheus targets")
        
        return True
    
    def run_complete_setup(self):
        """Run the complete monitoring setup"""
        print("🔧 Setting up Complete Chaos World Monitoring")
        print("=" * 60)
        
        # Create all configurations
        self.create_prometheus_config()
        self.create_alerting_rules()
        self.create_grafana_dashboards()
        self.create_grafana_provisioning()
        
        # Restart services
        self.restart_services()
        
        # Test setup
        if self.test_setup():
            print("\n🎉 Complete monitoring setup finished!")
            print(f"📊 Prometheus: http://localhost:{self.prometheus_port}")
            print(f"📈 Grafana: http://localhost:{self.grafana_port}")
            print("🔑 Grafana login: admin / admin")
            print("\n📋 What's included:")
            print("  • Service status monitoring")
            print("  • Request rate and response time graphs")
            print("  • Error rate tracking")
            print("  • Database and cache performance")
            print("  • Alerting rules for service health")
            print("  • Pre-configured dashboards")
            return True
        else:
            print("\n❌ Setup failed!")
            return False

def main():
    """Main function"""
    setup = CompleteMonitoringSetup()
    success = setup.run_complete_setup()
    
    if success:
        print("\n✅ Your monitoring stack is ready!")
        print("Open Grafana and explore the pre-configured dashboards!")
    else:
        print("\n❌ Setup failed!")
        sys.exit(1)

if __name__ == "__main__":
    main()
