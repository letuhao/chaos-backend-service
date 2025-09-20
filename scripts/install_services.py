#!/usr/bin/env python3
"""
Install Chaos World Backend Services
A Python script to install all Chaos World services using NSSM
"""

import os
import sys
import subprocess
import shutil
from pathlib import Path

class ServiceInstaller:
    def __init__(self):
        self.nssm_path = r"C:\ProgramData\chocolatey\bin\nssm.exe"
        self.service_dir = r"C:\ChaosWorld\services"
        self.log_dir = r"C:\ChaosWorld\logs"
        self.project_root = Path(__file__).parent.parent
        
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
    
    def check_nssm(self) -> bool:
        """Check if NSSM is installed"""
        return os.path.exists(self.nssm_path)
    
    def check_service_files(self) -> bool:
        """Check if service executables exist"""
        # Check if executables exist in target directory first
        target_dir = self.project_root / "target" / "release"
        api_gateway_target = target_dir / "api-gateway.exe"
        chaos_backend_target = target_dir / "chaos-backend.exe"
        cms_service_target = target_dir / "content-management-service.exe"
        
        if not api_gateway_target.exists():
            self.log(f"API Gateway not found at {api_gateway_target}", "ERROR")
            self.log("Run 'python build_services.py' to build all services", "INFO")
            return False
        
        if not chaos_backend_target.exists():
            self.log(f"Chaos Backend not found at {chaos_backend_target}", "ERROR")
            self.log("Run 'python build_services.py' to build all services", "INFO")
            return False
        
        if not cms_service_target.exists():
            self.log(f"CMS Service not found at {cms_service_target}", "ERROR")
            self.log("Run 'python build_services.py' to build all services", "INFO")
            return False
        
        # Copy executables to service directory
        self.log("Copying service executables to service directory...")
        try:
            shutil.copy2(api_gateway_target, os.path.join(self.service_dir, "api-gateway.exe"))
            shutil.copy2(chaos_backend_target, os.path.join(self.service_dir, "chaos-backend.exe"))
            shutil.copy2(cms_service_target, os.path.join(self.service_dir, "content-management-service.exe"))
            self.log("Service executables copied successfully")
        except Exception as e:
            self.log(f"Failed to copy service executables: {e}", "ERROR")
            return False
        
        return True
    
    def create_directories(self):
        """Create necessary directories"""
        os.makedirs(self.service_dir, exist_ok=True)
        os.makedirs(self.log_dir, exist_ok=True)
        self.log(f"Created directories: {self.service_dir}, {self.log_dir}")
    
    def copy_executables(self):
        """Copy service executables to service directory"""
        # This method is now handled by check_service_files()
        # Keeping for backward compatibility but it should not be called
        self.log("Note: copy_executables() is deprecated, using check_service_files() instead", "INFO")
        return True
    
    def run_nssm_command(self, service_name: str, command: str, *args) -> bool:
        """Run NSSM command"""
        cmd = [self.nssm_path, command, service_name] + list(args)
        try:
            result = subprocess.run(cmd, capture_output=True, text=True, check=True)
            return True
        except subprocess.CalledProcessError as e:
            self.log(f"NSSM command failed: {' '.join(cmd)}", "ERROR")
            self.log(f"Error: {e.stderr}", "ERROR")
            return False
    
    def install_service(self, service_name: str, exe_path: str, display_name: str, description: str, app_parameters: str = "", environment_vars: dict = None):
        """Install a single service"""
        self.log(f"Installing {service_name}...")
        
        # Install service
        if not self.run_nssm_command(service_name, "install", exe_path):
            return False
        
        # Configure service
        configs = [
            ("AppDirectory", self.service_dir),
            ("AppStdout", os.path.join(self.log_dir, f"{service_name}.log")),
            ("AppStderr", os.path.join(self.log_dir, f"{service_name}-error.log")),
            ("AppRotateFiles", "1"),
            ("AppRotateOnline", "1"),
            ("AppRotateBytes", "1048576"),
            ("Start", "SERVICE_AUTO_START"),
            ("DisplayName", display_name),
            ("Description", description)
        ]
        
        # Add app parameters if provided
        if app_parameters:
            configs.append(("AppParameters", app_parameters))
        
        for key, value in configs:
            if not self.run_nssm_command(service_name, "set", key, value):
                return False
        
        # Add environment variables if provided
        if environment_vars:
            for env_key, env_value in environment_vars.items():
                if not self.run_nssm_command(service_name, "set", "AppEnvironmentExtra", f"{env_key}={env_value}"):
                    return False
        
        self.log(f"✅ {service_name} installed successfully")
        return True
    
    def install_all_services(self):
        """Install all Chaos World services"""
        self.log("Installing Chaos World Backend Services...")
        
        # Check prerequisites
        if not self.check_admin():
            self.log("This script must be run as Administrator!", "ERROR")
            self.log("Right-click and select 'Run as administrator'", "ERROR")
            return False
        
        if not self.check_nssm():
            self.log(f"NSSM not found at {self.nssm_path}", "ERROR")
            self.log("Please run install_nssm.bat first", "ERROR")
            return False
        
        # Create directories
        self.create_directories()
        
        # Check and copy service files
        if not self.check_service_files():
            return False
        
        # Install services
        services = [
            {
                "name": "ChaosWorld-API-Gateway",
                "exe": os.path.join(self.service_dir, "api-gateway.exe"),
                "display_name": "Chaos World API Gateway",
                "description": "API Gateway for Chaos World Game Backend",
                "app_parameters": "--port 8080",
                "environment_vars": {
                    "RUST_LOG": "info",
                    "API_GATEWAY_PORT": "8080"
                }
            },
            {
                "name": "ChaosWorld-Backend",
                "exe": os.path.join(self.service_dir, "chaos-backend.exe"),
                "display_name": "Chaos World Backend",
                "description": "Main Game Backend for Chaos World",
                "app_parameters": "--port 8081",
                "environment_vars": {
                    "RUST_LOG": "info",
                    "CHAOS_BACKEND_PORT": "8081"
                }
            },
            {
                "name": "ChaosWorld-CMS",
                "exe": os.path.join(self.service_dir, "content-management-service.exe"),
                "display_name": "Chaos World CMS",
                "description": "Content Management Service for Chaos World",
                "app_parameters": "--port 8083",
                "environment_vars": {
                    "RUST_LOG": "info",
                    "CMS_PORT": "8083",
                    "APP_ENV": "prod"
                }
            }
        ]
        
        success_count = 0
        for service in services:
            if self.install_service(
                service["name"],
                service["exe"],
                service["display_name"],
                service["description"],
                service["app_parameters"],
                service["environment_vars"]
            ):
                success_count += 1
        
        if success_count == len(services):
            self.log("", "SUCCESS")
            self.log("Services installed successfully!", "SUCCESS")
            self.log("", "SUCCESS")
            self.log("Services created:", "SUCCESS")
            for service in services:
                self.log(f"- {service['name']}", "SUCCESS")
            self.log("", "SUCCESS")
            self.log(f"Logs will be saved to: {self.log_dir}", "SUCCESS")
            self.log("", "SUCCESS")
            self.log("Next steps:", "SUCCESS")
            self.log("1. Run start_services.py to start the services", "SUCCESS")
            self.log("2. Check Windows Services (services.msc) to see the services", "SUCCESS")
            self.log("3. Run stop_services.py to stop the services", "SUCCESS")
            self.log("4. Run uninstall_services.py to remove the services", "SUCCESS")
            return True
        else:
            self.log(f"Only {success_count}/{len(services)} services installed successfully", "ERROR")
            return False

def main():
    installer = ServiceInstaller()
    
    if len(sys.argv) > 1 and sys.argv[1] == "help":
        print("Usage: python install_services.py")
        print("This script installs all Chaos World services using NSSM")
        print("Must be run as Administrator")
        return
    
    success = installer.install_all_services()
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
