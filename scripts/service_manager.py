#!/usr/bin/env python3
"""
Chaos World Service Manager
A Python-based service management tool for Windows services using NSSM
"""

import subprocess
import sys
import time
import requests
import json
from pathlib import Path

class ServiceManager:
    def __init__(self):
        self.services = {
            'api-gateway': 'ChaosWorld-API-Gateway',
            'chaos-backend': 'ChaosWorld-Backend'
        }
        self.endpoints = {
            'api-gateway': 'http://localhost:8080',
            'chaos-backend': 'http://localhost:8081'
        }
    
    def run_command(self, command, check=True):
        """Run a command and return the result"""
        try:
            result = subprocess.run(command, shell=True, capture_output=True, text=True, check=check)
            return result.returncode == 0, result.stdout, result.stderr
        except subprocess.CalledProcessError as e:
            return False, e.stdout, e.stderr
    
    def check_service_status(self, service_name):
        """Check if a service is running"""
        success, stdout, stderr = self.run_command(f'sc query "{service_name}"', check=False)
        if success:
            return 'RUNNING' in stdout
        return False
    
    def start_service(self, service_name):
        """Start a service"""
        print(f"🚀 Starting {service_name}...")
        success, stdout, stderr = self.run_command(f'nssm start "{service_name}"', check=False)
        if success:
            print(f"✅ {service_name} started successfully")
            return True
        else:
            print(f"❌ Failed to start {service_name}: {stderr}")
            return False
    
    def stop_service(self, service_name):
        """Stop a service"""
        print(f"🛑 Stopping {service_name}...")
        success, stdout, stderr = self.run_command(f'nssm stop "{service_name}"', check=False)
        if success:
            print(f"✅ {service_name} stopped successfully")
            return True
        else:
            print(f"❌ Failed to stop {service_name}: {stderr}")
            return False
    
    def restart_service(self, service_name):
        """Restart a service"""
        print(f"🔄 Restarting {service_name}...")
        self.stop_service(service_name)
        time.sleep(2)
        return self.start_service(service_name)
    
    def test_endpoint(self, service_name, endpoint):
        """Test if an endpoint is responding"""
        try:
            response = requests.get(f"{endpoint}/health", timeout=5)
            if response.status_code == 200:
                print(f"✅ {service_name} is responding on {endpoint}")
                return True
            else:
                print(f"❌ {service_name} returned status {response.status_code}")
                return False
        except requests.exceptions.RequestException as e:
            print(f"❌ {service_name} is not responding: {e}")
            return False
    
    def check_port(self, port):
        """Check if a port is in use"""
        success, stdout, stderr = self.run_command(f'netstat -an | findstr ":{port}"', check=False)
        return success
    
    def status(self):
        """Show comprehensive status of all services"""
        print("=" * 60)
        print("🔍 CHAOS WORLD SERVICES STATUS")
        print("=" * 60)
        
        for service_key, service_name in self.services.items():
            print(f"\n📋 {service_key.upper()}:")
            is_running = self.check_service_status(service_name)
            port = 8080 if service_key == 'api-gateway' else 8081
            port_in_use = self.check_port(port)
            endpoint_responding = self.test_endpoint(service_key, self.endpoints[service_key])
            
            status_icon = "✅" if is_running else "❌"
            port_icon = "✅" if port_in_use else "❌"
            endpoint_icon = "✅" if endpoint_responding else "❌"
            
            print(f"   Service: {status_icon} {'RUNNING' if is_running else 'STOPPED'}")
            print(f"   Port {port}: {port_icon} {'IN USE' if port_in_use else 'NOT IN USE'}")
            print(f"   Endpoint: {endpoint_icon} {'RESPONDING' if endpoint_responding else 'NOT RESPONDING'}")
    
    def start_all(self):
        """Start all services"""
        print("🚀 Starting all Chaos World services...")
        success = True
        for service_key, service_name in self.services.items():
            if not self.start_service(service_name):
                success = False
        return success
    
    def stop_all(self):
        """Stop all services"""
        print("🛑 Stopping all Chaos World services...")
        success = True
        for service_key, service_name in self.services.items():
            if not self.stop_service(service_name):
                success = False
        return success
    
    def restart_all(self):
        """Restart all services"""
        print("🔄 Restarting all Chaos World services...")
        self.stop_all()
        time.sleep(3)
        return self.start_all()

def main():
    manager = ServiceManager()
    
    if len(sys.argv) < 2:
        print("Usage: python service_manager.py <command>")
        print("Commands:")
        print("  status     - Show service status")
        print("  start      - Start all services")
        print("  stop       - Stop all services")
        print("  restart    - Restart all services")
        print("  start-api  - Start API Gateway only")
        print("  start-backend - Start Chaos Backend only")
        print("  stop-api   - Stop API Gateway only")
        print("  stop-backend - Stop Chaos Backend only")
        print("  restart-api - Restart API Gateway only")
        print("  restart-backend - Restart Chaos Backend only")
        sys.exit(1)
    
    command = sys.argv[1].lower()
    
    if command == 'status':
        manager.status()
    elif command == 'start':
        manager.start_all()
    elif command == 'stop':
        manager.stop_all()
    elif command == 'restart':
        manager.restart_all()
    elif command == 'start-api':
        manager.start_service(manager.services['api-gateway'])
    elif command == 'start-backend':
        manager.start_service(manager.services['chaos-backend'])
    elif command == 'stop-api':
        manager.stop_service(manager.services['api-gateway'])
    elif command == 'stop-backend':
        manager.stop_service(manager.services['chaos-backend'])
    elif command == 'restart-api':
        manager.restart_service(manager.services['api-gateway'])
    elif command == 'restart-backend':
        manager.restart_service(manager.services['chaos-backend'])
    else:
        print(f"Unknown command: {command}")
        sys.exit(1)

if __name__ == "__main__":
    main()
