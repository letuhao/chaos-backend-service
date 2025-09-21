#!/usr/bin/env python3
"""
Stop Chaos World Backend Services
A Python script to stop all Chaos World services
"""

import os
import sys
import subprocess
import time

class ServiceManager:
    def __init__(self):
        self.services = [
            "ChaosWorld-API-Gateway",
            "ChaosWorld-Backend", 
            "ChaosWorld-CMS",
            "ChaosWorld-UserManagement"
        ]
    
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
    
    def stop_service(self, service_name: str) -> bool:
        """Stop a single service"""
        try:
            result = subprocess.run(
                ["sc", "stop", service_name],
                capture_output=True,
                text=True,
                check=True
            )
            self.log(f"✅ {service_name} stopped")
            return True
        except subprocess.CalledProcessError as e:
            self.log(f"❌ Failed to stop {service_name}: {e.stderr}", "ERROR")
            return False
    
    def stop_all_services(self):
        """Stop all Chaos World services"""
        self.log("Stopping Chaos World Backend Services...")
        
        if not self.check_admin():
            self.log("This script must be run as Administrator!", "ERROR")
            self.log("Right-click and select 'Run as administrator'", "ERROR")
            return False
        
        success_count = 0
        for service in self.services:
            if self.stop_service(service):
                success_count += 1
        
        if success_count > 0:
            self.log("", "SUCCESS")
            self.log("Services stopped successfully!", "SUCCESS")
            return True
        else:
            self.log("No services were stopped successfully", "ERROR")
            return False

def main():
    manager = ServiceManager()
    
    if len(sys.argv) > 1 and sys.argv[1] == "help":
        print("Usage: python stop_services.py")
        print("This script stops all Chaos World services")
        print("Must be run as Administrator")
        return
    
    success = manager.stop_all_services()
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
