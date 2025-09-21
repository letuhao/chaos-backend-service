#!/usr/bin/env python3
"""
User Management Service Manager
A Python script to manage the User Management service individually
"""

import os
import sys
import subprocess
import time
import requests
import argparse
from pathlib import Path

class UserManagementServiceManager:
    def __init__(self):
        self.service_name = "ChaosWorld-UserManagement"
        self.service_url = "http://localhost:8082"
        self.project_root = Path(__file__).parent.parent
        self.service_dir = r"C:\ChaosWorld\services"
        self.log_dir = r"C:\ChaosWorld\logs"
        self.nssm_path = r"C:\ProgramData\chocolatey\bin\nssm.exe"
        
    def log(self, message: str, level: str = "INFO"):
        """Log a message with timestamp"""
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
    
    def check_service_exists(self) -> bool:
        """Check if the service is installed"""
        try:
            result = subprocess.run(
                ["sc", "query", self.service_name],
                capture_output=True,
                text=True,
                check=True
            )
            return "RUNNING" in result.stdout or "STOPPED" in result.stdout
        except subprocess.CalledProcessError:
            return False
    
    def get_service_status(self) -> str:
        """Get the current status of the service"""
        try:
            result = subprocess.run(
                ["sc", "query", self.service_name],
                capture_output=True,
                text=True,
                check=True
            )
            if "RUNNING" in result.stdout:
                return "RUNNING"
            elif "STOPPED" in result.stdout:
                return "STOPPED"
            else:
                return "UNKNOWN"
        except subprocess.CalledProcessError:
            return "NOT_INSTALLED"
    
    def start_service(self) -> bool:
        """Start the User Management service"""
        if not self.check_admin():
            self.log("This command requires Administrator privileges!", "ERROR")
            return False
        
        if not self.check_service_exists():
            self.log(f"Service {self.service_name} is not installed!", "ERROR")
            self.log("Run 'python install_services.py' to install all services", "INFO")
            return False
        
        try:
            result = subprocess.run(
                ["sc", "start", self.service_name],
                capture_output=True,
                text=True,
                check=True
            )
            self.log(f"✅ {self.service_name} started successfully")
            return True
        except subprocess.CalledProcessError as e:
            self.log(f"Failed to start {self.service_name}: {e.stderr}", "ERROR")
            return False
    
    def stop_service(self) -> bool:
        """Stop the User Management service"""
        if not self.check_admin():
            self.log("This command requires Administrator privileges!", "ERROR")
            return False
        
        try:
            result = subprocess.run(
                ["sc", "stop", self.service_name],
                capture_output=True,
                text=True,
                check=True
            )
            self.log(f"✅ {self.service_name} stopped successfully")
            return True
        except subprocess.CalledProcessError as e:
            self.log(f"Failed to stop {self.service_name}: {e.stderr}", "ERROR")
            return False
    
    def restart_service(self) -> bool:
        """Restart the User Management service"""
        self.log(f"Restarting {self.service_name}...")
        if self.stop_service():
            time.sleep(2)
            return self.start_service()
        return False
    
    def status_service(self) -> bool:
        """Check the status of the User Management service"""
        status = self.get_service_status()
        self.log(f"Service Status: {status}")
        
        if status == "RUNNING":
            # Test HTTP endpoint
            try:
                response = requests.get(f"{self.service_url}/health", timeout=5)
                if response.status_code == 200:
                    self.log("✅ Service is running and responding to HTTP requests")
                    return True
                else:
                    self.log(f"⚠️ Service is running but HTTP check failed: {response.status_code}", "WARNING")
                    return False
            except requests.exceptions.RequestException as e:
                self.log(f"⚠️ Service is running but not responding to HTTP requests: {e}", "WARNING")
                return False
        else:
            self.log(f"❌ Service is not running (Status: {status})", "ERROR")
            return False
    
    def test_endpoints(self) -> bool:
        """Test User Management API endpoints"""
        self.log("Testing User Management API endpoints...")
        
        endpoints = [
            ("/health", "Health Check"),
            ("/", "Root Endpoint"),
        ]
        
        success_count = 0
        for endpoint, description in endpoints:
            try:
                response = requests.get(f"{self.service_url}{endpoint}", timeout=5)
                if response.status_code == 200:
                    self.log(f"✅ {description}: OK")
                    success_count += 1
                else:
                    self.log(f"❌ {description}: HTTP {response.status_code}", "ERROR")
            except requests.exceptions.RequestException as e:
                self.log(f"❌ {description}: {e}", "ERROR")
        
        self.log(f"Test Results: {success_count}/{len(endpoints)} endpoints working")
        return success_count == len(endpoints)
    
    def show_logs(self, lines: int = 50) -> bool:
        """Show recent service logs"""
        log_file = os.path.join(self.log_dir, f"{self.service_name}.log")
        if not os.path.exists(log_file):
            self.log(f"Log file not found: {log_file}", "ERROR")
            return False
        
        try:
            with open(log_file, 'r', encoding='utf-8') as f:
                all_lines = f.readlines()
                recent_lines = all_lines[-lines:] if len(all_lines) > lines else all_lines
                
                self.log(f"Recent {len(recent_lines)} lines from {log_file}:")
                print("-" * 80)
                for line in recent_lines:
                    print(line.rstrip())
                print("-" * 80)
                return True
        except Exception as e:
            self.log(f"Failed to read log file: {e}", "ERROR")
            return False
    
    def build_service(self) -> bool:
        """Build the User Management service"""
        self.log("Building User Management service...")
        
        service_path = self.project_root / "services" / "user-management"
        if not service_path.exists():
            self.log(f"Service directory not found: {service_path}", "ERROR")
            return False
        
        try:
            # Change to service directory and build
            result = subprocess.run(
                ["cargo", "build", "--release", "--bin", "user-management"],
                cwd=service_path,
                capture_output=True,
                text=True,
                check=True
            )
            self.log("✅ User Management service built successfully")
            
            # Copy executable to service directory
            target_exe = self.project_root / "target" / "release" / "user-management.exe"
            if target_exe.exists():
                import shutil
                shutil.copy2(target_exe, os.path.join(self.service_dir, "user-management.exe"))
                self.log("✅ Executable copied to service directory")
            
            return True
        except subprocess.CalledProcessError as e:
            self.log(f"Build failed: {e.stderr}", "ERROR")
            return False
    
    def run_foreground(self, port: int = 8082) -> bool:
        """Run the service in foreground for development"""
        self.log(f"Starting User Management service in foreground on port {port}...")
        
        service_path = self.project_root / "services" / "user-management"
        if not service_path.exists():
            self.log(f"Service directory not found: {service_path}", "ERROR")
            return False
        
        try:
            # Set environment variables
            env = os.environ.copy()
            env["RUST_LOG"] = "info"
            env["USER_MANAGEMENT_PORT"] = str(port)
            env["MONGODB_URL"] = "mongodb://localhost:27017"
            
            # Run the service
            subprocess.run(
                ["cargo", "run", "--bin", "user-management"],
                cwd=service_path,
                env=env
            )
            return True
        except KeyboardInterrupt:
            self.log("Service stopped by user", "INFO")
            return True
        except Exception as e:
            self.log(f"Failed to run service: {e}", "ERROR")
            return False

def main():
    parser = argparse.ArgumentParser(description="User Management Service Manager")
    parser.add_argument("command", choices=[
        "start", "stop", "restart", "status", "test", "logs", "build", "foreground"
    ], help="Command to execute")
    parser.add_argument("--port", type=int, default=8082, help="Port for foreground mode")
    parser.add_argument("--lines", type=int, default=50, help="Number of log lines to show")
    
    args = parser.parse_args()
    
    manager = UserManagementServiceManager()
    
    if args.command == "start":
        success = manager.start_service()
    elif args.command == "stop":
        success = manager.stop_service()
    elif args.command == "restart":
        success = manager.restart_service()
    elif args.command == "status":
        success = manager.status_service()
    elif args.command == "test":
        success = manager.test_endpoints()
    elif args.command == "logs":
        success = manager.show_logs(args.lines)
    elif args.command == "build":
        success = manager.build_service()
    elif args.command == "foreground":
        success = manager.run_foreground(args.port)
    else:
        parser.print_help()
        success = False
    
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
