#!/usr/bin/env python3
"""
CMS Service Manager
A Python script to manage the Content Management Service
"""

import os
import sys
import json
import time
import signal
import subprocess
import requests
import argparse
from pathlib import Path
from typing import Dict, List, Optional

class CMSServiceManager:
    def __init__(self):
        self.service_name = "content-management-service"
        self.service_port = 8083
        self.metrics_port = 9090
        self.base_url = f"http://localhost:{self.service_port}"
        self.metrics_url = f"http://localhost:{self.metrics_port}"
        self.project_root = Path(__file__).parent.parent
        self.service_path = self.project_root / "services" / "content-management-service"
        self.process = None
        
    def log(self, message: str, level: str = "INFO"):
        """Log a message with timestamp"""
        timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
        print(f"[{timestamp}] [{level}] {message}")
    
    def check_port_available(self, port: int) -> bool:
        """Check if a port is available"""
        import socket
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            return s.connect_ex(('localhost', port)) != 0
    
    def wait_for_service(self, timeout: int = 30) -> bool:
        """Wait for service to be ready"""
        self.log(f"Waiting for service to be ready on port {self.service_port}...")
        
        for i in range(timeout):
            try:
                response = requests.get(f"{self.base_url}/health", timeout=1)
                if response.status_code == 200:
                    self.log("Service is ready!")
                    return True
            except requests.exceptions.RequestException:
                pass
            
            time.sleep(1)
            if i % 5 == 0 and i > 0:
                self.log(f"Still waiting... ({i}/{timeout})")
        
        self.log("Service failed to start within timeout", "ERROR")
        return False
    
    def start_service(self, port: Optional[int] = None, background: bool = True) -> bool:
        """Start the CMS service"""
        if port:
            self.service_port = port
            self.base_url = f"http://localhost:{self.service_port}"
        
        # Check if port is already in use
        if not self.check_port_available(self.service_port):
            self.log(f"Port {self.service_port} is already in use", "ERROR")
            return False
        
        if not self.check_port_available(self.metrics_port):
            self.log(f"Metrics port {self.metrics_port} is already in use", "ERROR")
            return False
        
        # Change to service directory
        os.chdir(self.service_path)
        
        # Set environment variables
        env = os.environ.copy()
        env["CMS_PORT"] = str(self.service_port)
        env["RUST_LOG"] = "info"
        
        self.log(f"Starting {self.service_name} on port {self.service_port}...")
        
        try:
            if background:
                # Start in background
                self.process = subprocess.Popen(
                    ["cargo", "run"],
                    env=env,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    text=True
                )
                
                # Wait for service to be ready
                if self.wait_for_service():
                    self.log(f"Service started successfully! PID: {self.process.pid}")
                    self.log(f"Service URL: {self.base_url}")
                    self.log(f"Metrics URL: {self.metrics_url}")
                    return True
                else:
                    self.stop_service()
                    return False
            else:
                # Start in foreground
                self.process = subprocess.run(
                    ["cargo", "run"],
                    env=env
                )
                return self.process.returncode == 0
                
        except Exception as e:
            self.log(f"Failed to start service: {e}", "ERROR")
            return False
    
    def stop_service(self) -> bool:
        """Stop the CMS service"""
        if self.process and self.process.poll() is None:
            self.log(f"Stopping {self.service_name} (PID: {self.process.pid})...")
            self.process.terminate()
            
            # Wait for graceful shutdown
            try:
                self.process.wait(timeout=10)
                self.log("Service stopped gracefully")
                return True
            except subprocess.TimeoutExpired:
                self.log("Force killing service...", "WARN")
                self.process.kill()
                self.process.wait()
                self.log("Service force stopped")
                return True
        else:
            self.log("Service is not running")
            return True
    
    def restart_service(self, port: Optional[int] = None) -> bool:
        """Restart the CMS service"""
        self.log("Restarting service...")
        self.stop_service()
        time.sleep(2)
        return self.start_service(port)
    
    def get_service_status(self) -> Dict:
        """Get service status"""
        status = {
            "running": False,
            "port": self.service_port,
            "metrics_port": self.metrics_port,
            "health": "unknown",
            "pid": None
        }
        
        if self.process and self.process.poll() is None:
            status["running"] = True
            status["pid"] = self.process.pid
        
        # Check health endpoint
        try:
            response = requests.get(f"{self.base_url}/health", timeout=5)
            if response.status_code == 200:
                status["health"] = "healthy"
            else:
                status["health"] = "unhealthy"
        except requests.exceptions.RequestException:
            status["health"] = "unreachable"
        
        return status
    
    def test_api(self) -> bool:
        """Test the API endpoints"""
        self.log("Testing API endpoints...")
        
        endpoints = [
            ("/", "Root endpoint"),
            ("/health", "Health check"),
            ("/api/v1/health", "Detailed health"),
            ("/api/v1/metrics/info", "Metrics info"),
        ]
        
        success_count = 0
        
        for endpoint, description in endpoints:
            try:
                response = requests.get(f"{self.base_url}{endpoint}", timeout=5)
                if response.status_code == 200:
                    self.log(f"✅ {description}: OK")
                    success_count += 1
                else:
                    self.log(f"❌ {description}: HTTP {response.status_code}", "ERROR")
            except requests.exceptions.RequestException as e:
                self.log(f"❌ {description}: {e}", "ERROR")
        
        # Test login endpoint
        try:
            login_data = {
                "username": "admin",
                "password": "admin123"
            }
            response = requests.post(
                f"{self.base_url}/api/v1/auth/login",
                json=login_data,
                timeout=5
            )
            if response.status_code == 200:
                data = response.json()
                if data.get("success"):
                    self.log("✅ Admin login: OK")
                    success_count += 1
                else:
                    self.log("❌ Admin login: Failed", "ERROR")
            else:
                self.log(f"❌ Admin login: HTTP {response.status_code}", "ERROR")
        except requests.exceptions.RequestException as e:
            self.log(f"❌ Admin login: {e}", "ERROR")
        
        self.log(f"API Test Results: {success_count}/{len(endpoints) + 1} passed")
        return success_count == len(endpoints) + 1
    
    def show_logs(self, lines: int = 50):
        """Show service logs"""
        if self.process and self.process.poll() is None:
            self.log(f"Service logs (last {lines} lines):")
            print("-" * 50)
            # Note: In a real implementation, you'd want to capture and display logs
            print("Logs would be displayed here...")
        else:
            self.log("Service is not running")
    
    def build_service(self) -> bool:
        """Build the service"""
        self.log("Building service...")
        os.chdir(self.service_path)
        
        try:
            result = subprocess.run(
                ["cargo", "build", "--release"],
                capture_output=True,
                text=True
            )
            
            if result.returncode == 0:
                self.log("Build successful!")
                return True
            else:
                self.log(f"Build failed: {result.stderr}", "ERROR")
                return False
        except Exception as e:
            self.log(f"Build error: {e}", "ERROR")
            return False

def main():
    parser = argparse.ArgumentParser(description="CMS Service Manager")
    parser.add_argument("command", choices=[
        "start", "stop", "restart", "status", "test", "logs", "build"
    ], help="Command to execute")
    parser.add_argument("--port", type=int, help="Service port (default: 8083)")
    parser.add_argument("--foreground", action="store_true", help="Run in foreground")
    parser.add_argument("--lines", type=int, default=50, help="Number of log lines to show")
    
    args = parser.parse_args()
    
    manager = CMSServiceManager()
    
    if args.command == "start":
        success = manager.start_service(
            port=args.port,
            background=not args.foreground
        )
        sys.exit(0 if success else 1)
    
    elif args.command == "stop":
        success = manager.stop_service()
        sys.exit(0 if success else 1)
    
    elif args.command == "restart":
        success = manager.restart_service(port=args.port)
        sys.exit(0 if success else 1)
    
    elif args.command == "status":
        status = manager.get_service_status()
        print(json.dumps(status, indent=2))
    
    elif args.command == "test":
        success = manager.test_api()
        sys.exit(0 if success else 1)
    
    elif args.command == "logs":
        manager.show_logs(lines=args.lines)
    
    elif args.command == "build":
        success = manager.build_service()
        sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
