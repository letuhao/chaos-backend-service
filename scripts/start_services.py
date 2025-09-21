#!/usr/bin/env python3
"""
Start Chaos World Backend Services
A Python script to start all Chaos World services
"""

import os
import sys
import subprocess
import time
import requests

class ServiceManager:
    def __init__(self):
        self.services = [
            "ChaosWorld-API-Gateway",
            "ChaosWorld-Backend", 
            "ChaosWorld-CMS",
            "ChaosWorld-UserManagement"
        ]
        self.service_urls = {
            "ChaosWorld-API-Gateway": "http://localhost:8080",
            "ChaosWorld-Backend": "http://localhost:8081",
            "ChaosWorld-CMS": "http://localhost:8083",
            "ChaosWorld-UserManagement": "http://localhost:8082"
        }
    
    def log(self, message: str, level: str = "INFO"):
        """Log a message with timestamp"""
        import time
        timestamp = time.strftime("%H:%M:%S")
        print(f"[{timestamp}] {level}: {message}")
    
    def check_admin(self) -> bool:
        """Check if running as administrator"""
        try:
            import ctypes
            return ctypes.windll.shell32.IsUserAnAdmin()
        except:
            return False
    
    def start_service(self, service_name: str) -> bool:
        """Start a single service"""
        try:
            result = subprocess.run(
                ["sc", "start", service_name],
                capture_output=True,
                text=True,
                check=True
            )
            self.log(f"✅ {service_name} started")
            return True
        except subprocess.CalledProcessError as e:
            self.log(f"❌ Failed to start {service_name}: {e.stderr}", "ERROR")
            return False
    
    def check_service_health(self, service_name: str) -> bool:
        """Check if service is responding"""
        if service_name not in self.service_urls:
            return True
        
        url = self.service_urls[service_name]
        try:
            response = requests.get(f"{url}/health", timeout=5)
            return response.status_code == 200
        except:
            return False
    
    def start_all_services(self):
        """Start all Chaos World services"""
        self.log("Starting Chaos World Backend Services...")
        
        if not self.check_admin():
            self.log("This script must be run as Administrator!", "ERROR")
            self.log("Right-click and select 'Run as administrator'", "ERROR")
            return False
        
        success_count = 0
        for service in self.services:
            if self.start_service(service):
                success_count += 1
        
        if success_count > 0:
            self.log("", "SUCCESS")
            self.log("Waiting for services to initialize...", "INFO")
            time.sleep(10)
            
            # Check service health
            self.log("Checking service health...", "INFO")
            for service in self.services:
                if self.check_service_health(service):
                    self.log(f"✅ {service} is healthy")
                else:
                    self.log(f"⚠️  {service} may not be responding yet", "WARNING")
            
            self.log("", "SUCCESS")
            self.log("Services started successfully!", "SUCCESS")
            self.log("", "SUCCESS")
            self.log("Service URLs:", "SUCCESS")
            self.log("- API Gateway: http://localhost:8080", "SUCCESS")
            self.log("- Chaos Backend: http://localhost:8081", "SUCCESS")
            self.log("- User Management: http://localhost:8082", "SUCCESS")
            self.log("- CMS Service: http://localhost:8083", "SUCCESS")
            return True
        else:
            self.log("No services were started successfully", "ERROR")
            return False

def main():
    manager = ServiceManager()
    
    if len(sys.argv) > 1 and sys.argv[1] == "help":
        print("Usage: python start_services.py")
        print("This script starts all Chaos World services")
        print("Must be run as Administrator")
        return
    
    success = manager.start_all_services()
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
