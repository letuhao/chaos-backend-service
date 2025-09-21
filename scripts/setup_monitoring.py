#!/usr/bin/env python3
"""
Setup Monitoring Stack (Grafana + Prometheus)
Configures and starts Grafana and Prometheus for Chaos World monitoring
"""

import os
import sys
import subprocess
import shutil
from pathlib import Path
import json
import time

class MonitoringSetup:
    def __init__(self):
        self.grafana_path = r"C:\ProgramData\chocolatey\lib\grafana\tools\grafana-11.5.8"
        self.prometheus_path = r"C:\ProgramData\chocolatey\lib\prometheus\tools\prometheus-2.2.1.windows-amd64"
        self.config_dir = Path("C:/ChaosWorld/monitoring")
        self.prometheus_port = 9091  # Avoid conflict with CMS metrics on 9090
        self.grafana_port = 3001     # Use port 3001 to avoid conflicts
        
    def create_directories(self):
        """Create monitoring configuration directories"""
        print("📁 Creating monitoring directories...")
        self.config_dir.mkdir(parents=True, exist_ok=True)
        (self.config_dir / "prometheus").mkdir(exist_ok=True)
        (self.config_dir / "grafana").mkdir(exist_ok=True)
        print(f"✅ Directories created: {self.config_dir}")
    
    def create_prometheus_config(self):
        """Create Prometheus configuration"""
        print("⚙️ Creating Prometheus configuration...")
        
        config = {
            "global": {
                "scrape_interval": "15s",
                "evaluation_interval": "15s"
            },
            "rule_files": [],
            "scrape_configs": [
                {
                    "job_name": "chaos-world-api-gateway",
                    "static_configs": [
                        {
                            "targets": ["localhost:8080"]
                        }
                    ],
                    "metrics_path": "/metrics",
                    "scrape_interval": "5s"
                },
                {
                    "job_name": "chaos-world-backend",
                    "static_configs": [
                        {
                            "targets": ["localhost:8081"]
                        }
                    ],
                    "metrics_path": "/metrics",
                    "scrape_interval": "5s"
                },
                {
                    "job_name": "chaos-world-user-management",
                    "static_configs": [
                        {
                            "targets": ["localhost:8082"]
                        }
                    ],
                    "metrics_path": "/metrics",
                    "scrape_interval": "5s"
                },
                {
                    "job_name": "chaos-world-cms",
                    "static_configs": [
                        {
                            "targets": ["localhost:9090"]  # CMS metrics server
                        }
                    ],
                    "metrics_path": "/metrics",
                    "scrape_interval": "5s"
                }
            ]
        }
        
        config_file = self.config_dir / "prometheus" / "prometheus.yml"
        with open(config_file, 'w') as f:
            import yaml
            yaml.dump(config, f, default_flow_style=False)
        
        print(f"✅ Prometheus config created: {config_file}")
        return config_file
    
    def start_prometheus(self):
        """Start Prometheus on port 9091"""
        print("🚀 Starting Prometheus...")
        
        config_file = self.config_dir / "prometheus" / "prometheus.yml"
        prometheus_exe = Path(self.prometheus_path) / "prometheus.exe"
        
        if not prometheus_exe.exists():
            print(f"❌ Prometheus executable not found: {prometheus_exe}")
            return False
        
        # Kill existing Prometheus if running
        try:
            subprocess.run(["taskkill", "/F", "/IM", "prometheus.exe"], 
                         capture_output=True, check=False)
            time.sleep(2)
        except:
            pass
        
        # Start Prometheus
        cmd = [
            str(prometheus_exe),
            f"--config.file={config_file}",
            f"--web.listen-address=0.0.0.0:{self.prometheus_port}",
            f"--storage.tsdb.path={self.config_dir / 'prometheus' / 'data'}",
            "--web.enable-lifecycle"
        ]
        
        try:
            subprocess.Popen(cmd, 
                           stdout=subprocess.DEVNULL, 
                           stderr=subprocess.DEVNULL)
            time.sleep(3)
            
            # Test if Prometheus is running
            import requests
            response = requests.get(f"http://localhost:{self.prometheus_port}", timeout=5)
            if response.status_code == 200:
                print(f"✅ Prometheus started on port {self.prometheus_port}")
                return True
            else:
                print(f"❌ Prometheus failed to start")
                return False
        except Exception as e:
            print(f"❌ Error starting Prometheus: {e}")
            return False
    
    def start_grafana(self):
        """Start Grafana on port 3001"""
        print(f"🚀 Starting Grafana on port {self.grafana_port}...")
        
        grafana_exe = Path(self.grafana_path) / "bin" / "grafana-server.exe"
        config_file = Path(self.grafana_path) / "conf" / "defaults.ini"
        
        if not grafana_exe.exists():
            print(f"❌ Grafana executable not found: {grafana_exe}")
            return False
        
        print(f"📁 Grafana executable: {grafana_exe}")
        print(f"📁 Config file: {config_file}")
        
        # Kill existing Grafana if running
        try:
            print("🔄 Stopping existing Grafana processes...")
            subprocess.run(["taskkill", "/F", "/IM", "grafana-server.exe"], 
                         capture_output=True, check=False)
            time.sleep(3)
        except:
            pass
        
        # Create a custom Grafana config file with the correct port
        custom_config = self.config_dir / "grafana" / "grafana.ini"
        custom_config.parent.mkdir(parents=True, exist_ok=True)
        
        # Ensure data and logs directories exist
        (self.config_dir / "grafana" / "data").mkdir(parents=True, exist_ok=True)
        (self.config_dir / "grafana" / "logs").mkdir(parents=True, exist_ok=True)
        
        # Read the default config and modify it
        with open(config_file, 'r') as f:
            config_content = f.read()
        
        # Add port and security configuration
        config_content += f"""
[server]
http_port = {self.grafana_port}

[paths]
data = {self.config_dir / 'grafana' / 'data'}
logs = {self.config_dir / 'grafana' / 'logs'}

[security]
admin_user = admin
admin_password = admin123

[log]
level = info
"""
        
        with open(custom_config, 'w') as f:
            f.write(config_content)
        
        # Start Grafana with the custom config
        cmd = [
            str(grafana_exe),
            f"--config={custom_config}",
            f"--homepath={self.grafana_path}"
        ]
        
        print(f"🔧 Starting Grafana with command: {' '.join(cmd)}")
        
        try:
            # Start Grafana in background
            process = subprocess.Popen(cmd, 
                                     stdout=subprocess.PIPE, 
                                     stderr=subprocess.PIPE,
                                     text=True)
            
            # Wait a bit for startup
            print("⏳ Waiting for Grafana to start...")
            time.sleep(10)
            
            # Check if process is still running
            if process.poll() is not None:
                stdout, stderr = process.communicate()
                print(f"❌ Grafana process exited early")
                print(f"STDOUT: {stdout}")
                print(f"STDERR: {stderr}")
                return False
            
            # Test if Grafana is running
            print(f"🔍 Testing Grafana on http://localhost:{self.grafana_port}")
            import requests
            response = requests.get(f"http://localhost:{self.grafana_port}", timeout=15)
            if response.status_code == 200:
                print(f"✅ Grafana started successfully on port {self.grafana_port}")
                return True
            else:
                print(f"❌ Grafana responded with status {response.status_code}")
                return False
        except Exception as e:
            print(f"❌ Error starting Grafana: {e}")
            return False
    
    def setup_grafana_datasource(self):
        """Configure Grafana to use Prometheus as datasource"""
        print("🔗 Setting up Grafana datasource...")
        
        # Wait for Grafana to be ready
        time.sleep(10)
        
        datasource_config = {
            "name": "Prometheus",
            "type": "prometheus",
            "url": f"http://localhost:{self.prometheus_port}",
            "access": "proxy",
            "isDefault": True
        }
        
        try:
            import requests
            
            # Create datasource (Grafana runs on port 3001)
            response = requests.post(
                "http://localhost:3001/api/datasources",
                json=datasource_config,
                headers={"Content-Type": "application/json"},
                auth=("admin", "admin123")
            )
            
            if response.status_code in [200, 409]:  # 409 = already exists
                print("✅ Grafana datasource configured")
                return True
            else:
                print(f"⚠️ Could not configure datasource: {response.status_code}")
                return False
        except Exception as e:
            print(f"⚠️ Could not configure datasource: {e}")
            return False
    
    def run_setup(self):
        """Run the complete monitoring setup"""
        print("🔧 Setting up Chaos World Monitoring Stack")
        print("=" * 50)
        
        # Create directories
        self.create_directories()
        
        # Create Prometheus config
        self.create_prometheus_config()
        
        # Start services
        if not self.start_prometheus():
            print("❌ Failed to start Prometheus")
            return False
        
        if not self.start_grafana():
            print("❌ Failed to start Grafana")
            return False
        
        # Setup datasource
        self.setup_grafana_datasource()
        
        print("\n🎉 Monitoring setup completed!")
        print(f"📊 Prometheus: http://localhost:{self.prometheus_port}")
        print(f"📈 Grafana: http://localhost:{self.grafana_port}")
        print("🔑 Grafana login: admin / admin")
        print(f"📊 CMS Metrics: http://localhost:9090 (your existing service)")
        
        return True

def main():
    """Main function"""
    setup = MonitoringSetup()
    success = setup.run_setup()
    
    if success:
        print("\n✅ Monitoring stack is ready!")
        print("Next steps:")
        print(f"1. Open Grafana: http://localhost:{setup.grafana_port}")
        print("2. Login with admin/admin")
        print("3. Create dashboards for your services")
        print(f"4. Check Prometheus: http://localhost:{setup.prometheus_port}")
    else:
        print("\n❌ Setup failed!")
        sys.exit(1)

if __name__ == "__main__":
    main()
