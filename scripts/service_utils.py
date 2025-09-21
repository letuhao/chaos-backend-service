#!/usr/bin/env python3
"""
Service Utilities
A utility module for managing Chaos World services without requiring admin privileges
"""

import os
import sys
import subprocess
import time
import shutil
import psutil
from pathlib import Path
from typing import List, Dict, Optional

class ServiceUtils:
    """Utility class for managing services without admin privileges"""
    
    def __init__(self):
        self.project_root = Path(__file__).parent.parent
        self.service_dir = Path("C:/ChaosWorld/services")
        self.target_dir = self.project_root / "target" / "release"
        
        # Service configurations
        self.services = {
            "api-gateway": {
                "exe": "api-gateway.exe",
                "port": 8080,
                "name": "API Gateway"
            },
            "user-management": {
                "exe": "user-management.exe", 
                "port": 8082,
                "name": "User Management"
            },
            "chaos-backend": {
                "exe": "chaos-backend.exe",
                "port": 8081, 
                "name": "Chaos Backend"
            },
            "content-management-service": {
                "exe": "content-management-service.exe",
                "port": 8083,
                "name": "Content Management Service"
            }
        }
    
    def log(self, message: str, level: str = "INFO"):
        """Log a message with timestamp"""
        timestamp = time.strftime("%H:%M:%S")
        print(f"[{timestamp}] {level}: {message}")
    
    def find_process_by_port(self, port: int) -> Optional[psutil.Process]:
        """Find process running on a specific port"""
        try:
            for conn in psutil.net_connections(kind='inet'):
                if conn.laddr.port == port and conn.status == 'LISTEN':
                    try:
                        return psutil.Process(conn.pid)
                    except (psutil.NoSuchProcess, psutil.AccessDenied):
                        continue
        except Exception as e:
            self.log(f"Error finding process on port {port}: {e}", "WARNING")
        return None
    
    def find_process_by_name(self, exe_name: str) -> List[psutil.Process]:
        """Find processes by executable name"""
        processes = []
        try:
            for proc in psutil.process_iter(['pid', 'name', 'exe']):
                try:
                    if proc.info['name'] == exe_name or (proc.info['exe'] and proc.info['exe'].endswith(exe_name)):
                        processes.append(proc)
                except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.ZombieProcess):
                    continue
        except Exception as e:
            self.log(f"Error finding processes by name {exe_name}: {e}", "WARNING")
        return processes
    
    def stop_service_by_port(self, port: int, service_name: str) -> bool:
        """Stop service running on a specific port"""
        process = self.find_process_by_port(port)
        if process:
            try:
                self.log(f"Stopping {service_name} (PID: {process.pid}) on port {port}")
                process.terminate()
                
                # Wait for graceful shutdown
                try:
                    process.wait(timeout=10)
                    self.log(f"✅ {service_name} stopped gracefully")
                    return True
                except psutil.TimeoutExpired:
                    self.log(f"⚠️  {service_name} didn't stop gracefully, forcing kill", "WARNING")
                    process.kill()
                    process.wait(timeout=5)
                    self.log(f"✅ {service_name} force stopped")
                    return True
            except Exception as e:
                self.log(f"❌ Error stopping {service_name}: {e}", "ERROR")
                return False
        else:
            self.log(f"ℹ️  {service_name} not running on port {port}")
            return True
    
    def stop_service_by_name(self, exe_name: str, service_name: str) -> bool:
        """Stop service by executable name"""
        processes = self.find_process_by_name(exe_name)
        if not processes:
            self.log(f"ℹ️  {service_name} not running")
            return True
        
        success = True
        for process in processes:
            try:
                self.log(f"Stopping {service_name} (PID: {process.pid})")
                process.terminate()
                
                # Wait for graceful shutdown
                try:
                    process.wait(timeout=10)
                    self.log(f"✅ {service_name} stopped gracefully")
                except psutil.TimeoutExpired:
                    self.log(f"⚠️  {service_name} didn't stop gracefully, forcing kill", "WARNING")
                    process.kill()
                    process.wait(timeout=5)
                    self.log(f"✅ {service_name} force stopped")
            except Exception as e:
                self.log(f"❌ Error stopping {service_name}: {e}", "ERROR")
                success = False
        
        return success
    
    def stop_all_services(self, service_ids: List[str] = None) -> bool:
        """Stop all Chaos World services or specific services"""
        if service_ids:
            self.log(f"🛑 Stopping services: {', '.join(service_ids)}...")
        else:
            self.log("🛑 Stopping all Chaos World services...")
        print("=" * 50)
        
        success = True
        
        # If no specific services requested, stop all
        if service_ids is None:
            service_ids = list(self.services.keys())
        
        # Stop by port first (more reliable)
        for service_id in service_ids:
            if service_id in self.services:
                config = self.services[service_id]
                if not self.stop_service_by_port(config["port"], config["name"]):
                    success = False
        
        # Also stop by executable name as backup
        for service_id in service_ids:
            if service_id in self.services:
                config = self.services[service_id]
                if not self.stop_service_by_name(config["exe"], config["name"]):
                    success = False
        
        # Wait a moment for all processes to fully stop
        time.sleep(2)
        
        return success
    
    def build_services(self, service_ids: List[str] = None) -> bool:
        """Build all services using cargo"""
        self.log("🔨 Building all services...")
        print("=" * 50)
        
        if not self.target_dir.exists():
            self.log(f"❌ Target directory not found: {self.target_dir}", "ERROR")
            return False
        
        # If no specific services requested, build all
        if service_ids is None:
            service_ids = list(self.services.keys())
        
        success = True
        
        for service_id in service_ids:
            if service_id not in self.services:
                self.log(f"❌ Unknown service: {service_id}", "ERROR")
                success = False
                continue
                
            config = self.services[service_id]
            self.log(f"Building {config['name']}...")
            try:
                # Build the service
                result = subprocess.run(
                    ["cargo", "build", "--release", "--bin", service_id],
                    cwd=self.project_root,
                    capture_output=True,
                    text=True,
                    timeout=300  # 5 minute timeout
                )
                
                if result.returncode == 0:
                    self.log(f"✅ {config['name']} built successfully")
                else:
                    self.log(f"❌ {config['name']} build failed", "ERROR")
                    if result.stderr:
                        print(f"Error: {result.stderr}")
                    success = False
                    
            except subprocess.TimeoutExpired:
                self.log(f"❌ {config['name']} build timed out", "ERROR")
                success = False
            except Exception as e:
                self.log(f"❌ Error building {config['name']}: {e}", "ERROR")
                success = False
        
        return success
    
    def copy_config_files(self, service_ids: List[str] = None) -> bool:
        """Copy configuration files to service directory"""
        self.log("📋 Copying configuration files...")
        print("=" * 50)
        
        configs_dir = self.project_root / "services"
        target_configs_dir = self.service_dir / "configs"
        
        if not configs_dir.exists():
            self.log(f"❌ Source configs directory not found: {configs_dir}", "ERROR")
            return False
        
        if not target_configs_dir.exists():
            try:
                target_configs_dir.mkdir(parents=True, exist_ok=True)
                self.log(f"✅ Created configs directory: {target_configs_dir}")
            except Exception as e:
                self.log(f"❌ Failed to create configs directory: {e}", "ERROR")
                return False
        
        success = True
        
        # If no specific services requested, copy all
        if service_ids is None:
            service_ids = list(self.services.keys())
        
        # Copy config files for each service
        for service_id in service_ids:
            if service_id not in self.services:
                self.log(f"❌ Unknown service: {service_id}", "ERROR")
                success = False
                continue
                
            config = self.services[service_id]
            source_config_dir = configs_dir / service_id / "configs"
            target_service_config_dir = target_configs_dir / service_id
            
            if source_config_dir.exists():
                try:
                    if target_service_config_dir.exists():
                        shutil.rmtree(target_service_config_dir)
                    shutil.copytree(source_config_dir, target_service_config_dir)
                    self.log(f"✅ Copied configs for {config['name']}")
                except Exception as e:
                    self.log(f"❌ Failed to copy configs for {config['name']}: {e}", "ERROR")
                    success = False
            else:
                self.log(f"⚠️  No configs found for {config['name']} at {source_config_dir}", "WARNING")
        
        return success
    
    def copy_services(self, service_ids: List[str] = None) -> bool:
        """Copy built executables to service directory"""
        self.log("📦 Copying service executables...")
        print("=" * 50)
        
        if not self.target_dir.exists():
            self.log(f"❌ Target directory not found: {self.target_dir}", "ERROR")
            return False
        
        if not self.service_dir.exists():
            self.log(f"❌ Service directory not found: {self.service_dir}", "ERROR")
            self.log("Please run install_services.py first to create the service directory", "ERROR")
            return False
        
        # If no specific services requested, copy all
        if service_ids is None:
            service_ids = list(self.services.keys())
        
        success = True
        
        for service_id in service_ids:
            if service_id not in self.services:
                self.log(f"❌ Unknown service: {service_id}", "ERROR")
                success = False
                continue
                
            config = self.services[service_id]
            source_path = self.target_dir / config["exe"]
            dest_path = self.service_dir / config["exe"]
            
            if source_path.exists():
                try:
                    # Copy the file
                    shutil.copy2(source_path, dest_path)
                    size = dest_path.stat().st_size
                    self.log(f"✅ Copied {config['name']} ({size:,} bytes)")
                except Exception as e:
                    self.log(f"❌ Failed to copy {config['name']}: {e}", "ERROR")
                    success = False
            else:
                self.log(f"❌ Source file not found: {source_path}", "ERROR")
                success = False
        
        return success
    
    def start_service(self, service_id: str) -> bool:
        """Start a specific service"""
        config = self.services.get(service_id)
        if not config:
            self.log(f"❌ Unknown service: {service_id}", "ERROR")
            return False
        
        exe_path = self.service_dir / config["exe"]
        if not exe_path.exists():
            self.log(f"❌ Executable not found: {exe_path}", "ERROR")
            return False
        
        try:
            self.log(f"Starting {config['name']}...")
            
            # Start the service (services now load config from files)
            cmd = [str(exe_path)]
            
            # Set environment variables for services that need them
            env = os.environ.copy()
            if service_id == "api-gateway":
                env["CONFIG_PATH"] = "configs/api-gateway/api-gateway.yaml"
            elif service_id == "user-management":
                env["CONFIG_PATH"] = "configs/user-management/user-management.yaml"
            elif service_id == "chaos-backend":
                env["CONFIG_PATH"] = "configs/chaos-backend/chaos-backend.yaml"
            elif service_id == "content-management-service":
                env["CONFIG_PATH"] = "configs/content-management-service/content-management-service.yaml"
            
            process = subprocess.Popen(
                cmd,
                cwd=str(self.service_dir),
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                env=env,
                creationflags=subprocess.CREATE_NEW_PROCESS_GROUP if os.name == 'nt' else 0
            )
            
            # Wait a moment for startup
            time.sleep(2)
            
            # Check if process is still running
            if process.poll() is None:
                self.log(f"✅ {config['name']} started (PID: {process.pid})")
                return True
            else:
                stdout, stderr = process.communicate()
                self.log(f"❌ {config['name']} failed to start", "ERROR")
                if stderr:
                    print(f"Error: {stderr.decode()}")
                return False
                
        except Exception as e:
            self.log(f"❌ Error starting {config['name']}: {e}", "ERROR")
            return False
    
    def start_all_services(self, service_ids: List[str] = None) -> bool:
        """Start all Chaos World services or specific services"""
        if service_ids:
            self.log(f"🚀 Starting services: {', '.join(service_ids)}...")
        else:
            self.log("🚀 Starting all Chaos World services...")
        print("=" * 50)
        
        success = True
        
        # If no specific services requested, start all in order
        if service_ids is None:
            start_order = ["user-management", "chaos-backend", "content-management-service", "api-gateway"]
        else:
            start_order = service_ids
        
        for service_id in start_order:
            if not self.start_service(service_id):
                success = False
                break  # Stop if any service fails to start
            time.sleep(1)  # Small delay between starts
        
        return success
    
    def check_service_health(self, service_id: str) -> bool:
        """Check if a service is healthy by testing its port"""
        config = self.services.get(service_id)
        if not config:
            return False
        
        try:
            import requests
            response = requests.get(f"http://localhost:{config['port']}/health", timeout=5)
            return response.status_code == 200
        except:
            return False
    
    def get_service_status(self) -> Dict[str, bool]:
        """Get status of all services"""
        status = {}
        for service_id, config in self.services.items():
            status[service_id] = self.check_service_health(service_id)
        return status
    
    def wait_for_services(self, service_ids: List[str] = None, timeout: int = 30) -> bool:
        """Wait for services to become healthy"""
        if service_ids:
            self.log(f"⏳ Waiting for services to become healthy: {', '.join(service_ids)} (timeout: {timeout}s)...")
        else:
            self.log(f"⏳ Waiting for all services to become healthy (timeout: {timeout}s)...")
        
        start_time = time.time()
        while time.time() - start_time < timeout:
            status = self.get_service_status()
            
            if service_ids:
                # Check only specific services
                healthy_count = sum(1 for service_id in service_ids if status.get(service_id, False))
                total_count = len(service_ids)
            else:
                # Check all services
                healthy_count = sum(status.values())
                total_count = len(status)
            
            if healthy_count == total_count:
                self.log("✅ All services are healthy!")
                return True
            
            self.log(f"Health check: {healthy_count}/{total_count} services healthy")
            time.sleep(2)
        
        self.log("❌ Timeout waiting for services to become healthy", "ERROR")
        return False
