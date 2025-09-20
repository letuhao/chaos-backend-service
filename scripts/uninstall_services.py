#!/usr/bin/env python3
"""
Uninstall Chaos World Backend Services
A Python script to uninstall all Chaos World services
"""

import os
import sys
import subprocess
import shutil

class ServiceManager:
    def __init__(self):
        self.services = [
            "ChaosWorld-API-Gateway",
            "ChaosWorld-Backend", 
            "ChaosWorld-CMS"
        ]
        self.nssm_path = r"C:\ProgramData\chocolatey\bin\nssm.exe"
        self.service_dir = r"C:\ChaosWorld\services"
        self.log_dir = r"C:\ChaosWorld\logs"
    
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
                text=True
            )
            self.log(f"Stopped {service_name}")
            return True
        except subprocess.CalledProcessError as e:
            self.log(f"Could not stop {service_name}: {e.stderr}", "WARNING")
            return False
    
    def uninstall_service(self, service_name: str) -> bool:
        """Uninstall a single service using NSSM"""
        try:
            result = subprocess.run(
                [self.nssm_path, "remove", service_name, "confirm"],
                capture_output=True,
                text=True,
                check=True
            )
            self.log(f"✅ {service_name} uninstalled")
            return True
        except subprocess.CalledProcessError as e:
            self.log(f"❌ Failed to uninstall {service_name}: {e.stderr}", "ERROR")
            return False
    
    def cleanup_directories(self):
        """Clean up service directories"""
        try:
            if os.path.exists(self.service_dir):
                shutil.rmtree(self.service_dir)
                self.log(f"Removed service directory: {self.service_dir}")
            
            if os.path.exists(self.log_dir):
                shutil.rmtree(self.log_dir)
                self.log(f"Removed log directory: {self.log_dir}")
            
            return True
        except Exception as e:
            self.log(f"Failed to cleanup directories: {e}", "ERROR")
            return False
    
    def uninstall_all_services(self):
        """Uninstall all Chaos World services"""
        self.log("Uninstalling Chaos World Backend Services...")
        
        if not self.check_admin():
            self.log("This script must be run as Administrator!", "ERROR")
            self.log("Right-click and select 'Run as administrator'", "ERROR")
            return False
        
        # Stop all services first
        self.log("Stopping services...")
        for service in self.services:
            self.stop_service(service)
        
        # Wait a moment for services to stop
        import time
        time.sleep(3)
        
        # Uninstall services
        success_count = 0
        for service in self.services:
            if self.uninstall_service(service):
                success_count += 1
        
        # Cleanup directories
        self.log("Cleaning up directories...")
        self.cleanup_directories()
        
        if success_count == len(self.services):
            self.log("", "SUCCESS")
            self.log("All services uninstalled successfully!", "SUCCESS")
            self.log("", "SUCCESS")
            self.log("Cleanup completed:", "SUCCESS")
            self.log(f"- Removed service directory: {self.service_dir}", "SUCCESS")
            self.log(f"- Removed log directory: {self.log_dir}", "SUCCESS")
            return True
        else:
            self.log(f"Only {success_count}/{len(self.services)} services uninstalled successfully", "ERROR")
            return False

def main():
    manager = ServiceManager()
    
    if len(sys.argv) > 1 and sys.argv[1] == "help":
        print("Usage: python uninstall_services.py")
        print("This script uninstalls all Chaos World services")
        print("Must be run as Administrator")
        return
    
    success = manager.uninstall_all_services()
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
