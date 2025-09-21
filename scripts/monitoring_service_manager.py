#!/usr/bin/env python3
"""
Monitoring Service Manager
A Python script to manage Prometheus and Grafana services for Chaos World monitoring.
"""

import os
import sys
import subprocess
import shutil
import time
import argparse
from pathlib import Path
import logging

# Import optional dependencies with fallback
try:
    import requests  # type: ignore
except ImportError:
    print("Warning: requests module not found. Please install with: pip install requests")
    requests = None

try:
    import yaml  # type: ignore
except ImportError:
    print("Warning: yaml module not found. Please install with: pip install PyYAML")
    yaml = None

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.StreamHandler(sys.stdout)
    ]
)
logger = logging.getLogger(__name__)

class MonitoringServiceManager:
    def __init__(self):
        self.grafana_path = r"C:\ProgramData\chocolatey\lib\grafana\tools\grafana-11.5.8"
        self.prometheus_path = r"C:\ProgramData\chocolatey\lib\prometheus\tools\prometheus-2.2.1.windows-amd64"
        self.config_dir = Path("C:/ChaosWorld/monitoring")
        self.prometheus_port = 9091
        self.grafana_port = 3001
        self.prometheus_url = f"http://localhost:{self.prometheus_port}"
        self.grafana_url = f"http://localhost:{self.grafana_port}"

    def log(self, message: str, level: str = "INFO"):
        """Log a message with timestamp"""
        logger.log(getattr(logging, level.upper()), message)

    def check_admin(self) -> bool:
        """Check if running as administrator"""
        try:
            import ctypes
            return ctypes.windll.shell32.IsUserAnAdmin()
        except:
            return False

    def check_grafana_installed(self) -> bool:
        """Check if Grafana is installed"""
        return os.path.exists(self.grafana_path)

    def check_prometheus_installed(self) -> bool:
        """Check if Prometheus is installed"""
        return os.path.exists(self.prometheus_path)

    def create_directories(self):
        """Create monitoring configuration directories"""
        self.log("📁 Creating monitoring directories...")
        self.config_dir.mkdir(parents=True, exist_ok=True)
        (self.config_dir / "prometheus").mkdir(exist_ok=True)
        (self.config_dir / "grafana").mkdir(exist_ok=True)
        self.log(f"✅ Directories created: {self.config_dir}")

    def create_prometheus_config(self):
        """Create Prometheus configuration"""
        self.log("⚙️ Creating Prometheus configuration...")
        
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
            if yaml is not None:
                yaml.dump(config, f, default_flow_style=False)
            else:
                # Fallback to JSON if yaml is not available
                import json
                json.dump(config, f, indent=2)
        
        self.log(f"✅ Prometheus config created: {config_file}")
        return config_file

    def start_prometheus(self):
        """Start Prometheus"""
        self.log("🚀 Starting Prometheus...")
        
        config_file = self.config_dir / "prometheus" / "prometheus.yml"
        prometheus_exe = Path(self.prometheus_path) / "prometheus.exe"
        
        if not prometheus_exe.exists():
            self.log(f"❌ Prometheus executable not found: {prometheus_exe}", "ERROR")
            self.log("Please install Prometheus via Chocolatey: choco install prometheus", "ERROR")
            return False
        
        # Kill existing Prometheus if running
        try:
            subprocess.run(["taskkill", "/F", "/IM", "prometheus.exe"], 
                         capture_output=True, check=False)
            time.sleep(2)
        except:
            pass
        
        # Start Prometheus
        try:
            cmd = [
                str(prometheus_exe),
                f"--config.file={config_file}",
                f"--web.listen-address=0.0.0.0:{self.prometheus_port}",
                f"--storage.tsdb.path={self.config_dir / 'prometheus' / 'data'}",
                "--web.enable-lifecycle"
            ]
            
            # Start in background
            subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            time.sleep(3)
            
            # Check if it's running
            if self.check_prometheus_running():
                self.log(f"✅ Prometheus started on {self.prometheus_url}")
                return True
            else:
                self.log("❌ Prometheus failed to start", "ERROR")
                return False
                
        except Exception as e:
            self.log(f"❌ Failed to start Prometheus: {e}", "ERROR")
            return False

    def create_grafana_config(self):
        """Create Grafana configuration"""
        self.log("⚙️ Creating Grafana configuration...")
        
        config_content = f"""[server]
http_port = {self.grafana_port}
http_addr = 0.0.0.0

[security]
admin_user = admin
admin_password = admin123

[database]
type = sqlite3
path = {self.config_dir / 'grafana' / 'grafana.db'}

[log]
mode = console
level = info
"""
        
        config_file = self.config_dir / "grafana" / "grafana.ini"
        with open(config_file, 'w') as f:
            f.write(config_content)
        
        self.log(f"✅ Grafana config created: {config_file}")
        return config_file

    def start_grafana(self):
        """Start Grafana"""
        self.log("🚀 Starting Grafana...")
        
        grafana_exe = Path(self.grafana_path) / "bin" / "grafana-server.exe"
        
        if not grafana_exe.exists():
            self.log(f"❌ Grafana executable not found: {grafana_exe}", "ERROR")
            self.log("Please install Grafana via Chocolatey: choco install grafana", "ERROR")
            return False
        
        # Kill existing Grafana if running
        try:
            subprocess.run(["taskkill", "/F", "/IM", "grafana-server.exe"], 
                         capture_output=True, check=False)
            time.sleep(2)
        except:
            pass
        
        # Create Grafana config
        config_file = self.create_grafana_config()
        
        # Start Grafana
        try:
            cmd = [
                str(grafana_exe),
                f"--config={config_file}",
                f"--homepath={self.grafana_path}"
            ]
            
            # Start in background
            subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            time.sleep(8)  # Give Grafana more time to start
            
            # Check if it's running
            if self.check_grafana_running():
                self.log(f"✅ Grafana started on {self.grafana_url}")
                return True
            else:
                self.log("❌ Grafana failed to start", "ERROR")
                return False
                
        except Exception as e:
            self.log(f"❌ Failed to start Grafana: {e}", "ERROR")
            return False

    def stop_prometheus(self):
        """Stop Prometheus"""
        self.log("🛑 Stopping Prometheus...")
        try:
            subprocess.run(["taskkill", "/F", "/IM", "prometheus.exe"], 
                         capture_output=True, check=False)
            time.sleep(2)
            if not self.check_prometheus_running():
                self.log("✅ Prometheus stopped")
                return True
            else:
                self.log("⚠️ Prometheus may still be running", "WARNING")
                return False
        except Exception as e:
            self.log(f"❌ Failed to stop Prometheus: {e}", "ERROR")
            return False

    def stop_grafana(self):
        """Stop Grafana"""
        self.log("🛑 Stopping Grafana...")
        try:
            subprocess.run(["taskkill", "/F", "/IM", "grafana-server.exe"], 
                         capture_output=True, check=False)
            time.sleep(2)
            if not self.check_grafana_running():
                self.log("✅ Grafana stopped")
                return True
            else:
                self.log("⚠️ Grafana may still be running", "WARNING")
                return False
        except Exception as e:
            self.log(f"❌ Failed to stop Grafana: {e}", "ERROR")
            return False

    def check_prometheus_running(self) -> bool:
        """Check if Prometheus is running"""
        if requests is None:
            return False
        try:
            response = requests.get(f"{self.prometheus_url}/api/v1/status/config", timeout=5)
            return response.status_code == 200
        except:
            return False

    def check_grafana_running(self) -> bool:
        """Check if Grafana is running"""
        if requests is None:
            return False
        try:
            response = requests.get(f"{self.grafana_url}/api/health", timeout=5)
            return response.status_code == 200
        except:
            return False

    def status(self):
        """Show status of monitoring services"""
        self.log("📊 Monitoring Services Status:")
        self.log("=" * 40)
        
        prometheus_status = "✅ Running" if self.check_prometheus_running() else "❌ Stopped"
        grafana_status = "✅ Running" if self.check_grafana_running() else "❌ Stopped"
        
        self.log(f"Prometheus: {prometheus_status} ({self.prometheus_url})")
        self.log(f"Grafana: {grafana_status} ({self.grafana_url})")
        
        if self.check_prometheus_running() and self.check_grafana_running():
            self.log("\n🎉 All monitoring services are running!")
            self.log(f"📊 Access Grafana: {self.grafana_url} (admin/admin)")
            self.log(f"📈 Access Prometheus: {self.prometheus_url}")
        else:
            self.log("\n⚠️ Some monitoring services are not running")
            self.log("Use 'python monitoring_service_manager.py start' to start them")

    def start_all(self):
        """Start all monitoring services"""
        self.log("🚀 Starting all monitoring services...")
        
        if not self.check_grafana_installed():
            self.log("❌ Grafana not installed. Please run: choco install grafana", "ERROR")
            return False
            
        if not self.check_prometheus_installed():
            self.log("❌ Prometheus not installed. Please run: choco install prometheus", "ERROR")
            return False
        
        # Create directories and config
        self.create_directories()
        self.create_prometheus_config()
        self.create_grafana_config()
        
        # Start services
        prometheus_ok = self.start_prometheus()
        grafana_ok = self.start_grafana()
        
        if prometheus_ok and grafana_ok:
            self.log("🎉 All monitoring services started successfully!")
            self.log(f"📊 Access Grafana: {self.grafana_url} (admin/admin)")
            self.log(f"📈 Access Prometheus: {self.prometheus_url}")
            return True
        else:
            self.log("❌ Some services failed to start", "ERROR")
            return False

    def stop_all(self):
        """Stop all monitoring services"""
        self.log("🛑 Stopping all monitoring services...")
        
        prometheus_ok = self.stop_prometheus()
        grafana_ok = self.stop_grafana()
        
        if prometheus_ok and grafana_ok:
            self.log("✅ All monitoring services stopped")
            return True
        else:
            self.log("⚠️ Some services may still be running", "WARNING")
            return False

    def restart_all(self):
        """Restart all monitoring services"""
        self.log("🔄 Restarting all monitoring services...")
        self.stop_all()
        time.sleep(3)
        return self.start_all()

    def test_services(self):
        """Test monitoring services"""
        self.log("🧪 Testing monitoring services...")
        
        if self.check_prometheus_running():
            try:
                response = requests.get(f"{self.prometheus_url}/api/v1/targets", timeout=5)
                if response.status_code == 200:
                    self.log("✅ Prometheus API is working")
                else:
                    self.log(f"⚠️ Prometheus API returned status {response.status_code}", "WARNING")
            except Exception as e:
                self.log(f"❌ Prometheus API test failed: {e}", "ERROR")
        else:
            self.log("❌ Prometheus is not running", "ERROR")
        
        if self.check_grafana_running():
            try:
                response = requests.get(f"{self.grafana_url}/api/health", timeout=5)
                if response.status_code == 200:
                    self.log("✅ Grafana API is working")
                else:
                    self.log(f"⚠️ Grafana API returned status {response.status_code}", "WARNING")
            except Exception as e:
                self.log(f"❌ Grafana API test failed: {e}", "ERROR")
        else:
            self.log("❌ Grafana is not running", "ERROR")

def main():
    manager = MonitoringServiceManager()
    parser = argparse.ArgumentParser(description="Monitoring Service Manager")
    parser.add_argument("command", choices=["start", "stop", "restart", "status", "test"], 
                       help="Command to execute")
    
    args = parser.parse_args()
    
    if args.command == "start":
        manager.start_all()
    elif args.command == "stop":
        manager.stop_all()
    elif args.command == "restart":
        manager.restart_all()
    elif args.command == "status":
        manager.status()
    elif args.command == "test":
        manager.test_services()

if __name__ == "__main__":
    main()
