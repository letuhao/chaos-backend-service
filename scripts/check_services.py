#!/usr/bin/env python3
"""
Check Chaos World Backend Services Status
A Python script to check the status of all Chaos World services
"""

import os
import sys
import subprocess
import requests
import json
from datetime import datetime

class ServiceChecker:
    def __init__(self):
        self.services = [
            {
                "name": "ChaosWorld-API-Gateway",
                "url": "http://localhost:8080",
                "port": 8080
            },
            {
                "name": "ChaosWorld-Backend", 
                "url": "http://localhost:8081",
                "port": 8081
            },
            {
                "name": "ChaosWorld-CMS",
                "url": "http://localhost:8083",
                "port": 8083
            }
        ]
    
    def log(self, message: str, level: str = "INFO"):
        """Log a message with timestamp"""
        timestamp = datetime.now().strftime("%H:%M:%S")
        print(f"[{timestamp}] {level}: {message}")
    
    def check_service_status(self, service_name: str) -> dict:
        """Check Windows service status"""
        try:
            result = subprocess.run(
                ["sc", "query", service_name],
                capture_output=True,
                text=True,
                check=True
            )
            
            lines = result.stdout.split('\n')
            status = "UNKNOWN"
            for line in lines:
                if "STATE" in line:
                    if "RUNNING" in line:
                        status = "RUNNING"
                    elif "STOPPED" in line:
                        status = "STOPPED"
                    elif "PAUSED" in line:
                        status = "PAUSED"
                    break
            
            return {
                "name": service_name,
                "status": status,
                "error": None
            }
        except subprocess.CalledProcessError as e:
            return {
                "name": service_name,
                "status": "NOT_FOUND",
                "error": e.stderr
            }
    
    def check_service_health(self, service: dict) -> dict:
        """Check if service is responding to HTTP requests"""
        try:
            response = requests.get(f"{service['url']}/health", timeout=5)
            if response.status_code == 200:
                return {
                    "healthy": True,
                    "response_time": response.elapsed.total_seconds(),
                    "status_code": response.status_code,
                    "error": None
                }
            else:
                return {
                    "healthy": False,
                    "response_time": response.elapsed.total_seconds(),
                    "status_code": response.status_code,
                    "error": f"HTTP {response.status_code}"
                }
        except requests.exceptions.ConnectionError:
            return {
                "healthy": False,
                "response_time": None,
                "status_code": None,
                "error": "Connection refused"
            }
        except requests.exceptions.Timeout:
            return {
                "healthy": False,
                "response_time": None,
                "status_code": None,
                "error": "Timeout"
            }
        except Exception as e:
            return {
                "healthy": False,
                "response_time": None,
                "status_code": None,
                "error": str(e)
            }
    
    def check_port_usage(self, port: int) -> bool:
        """Check if port is in use"""
        import socket
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            return s.connect_ex(('localhost', port)) == 0
    
    def check_all_services(self):
        """Check all Chaos World services"""
        self.log("Checking Chaos World Backend Services...")
        print("=" * 80)
        
        all_healthy = True
        
        for service in self.services:
            print(f"\n🔍 Checking {service['name']}...")
            
            # Check Windows service status
            service_status = self.check_service_status(service['name'])
            print(f"  Windows Service: {service_status['status']}")
            
            if service_status['error']:
                print(f"  Error: {service_status['error']}")
            
            # Check if port is in use
            port_in_use = self.check_port_usage(service['port'])
            print(f"  Port {service['port']}: {'In Use' if port_in_use else 'Available'}")
            
            # Check HTTP health
            health = self.check_service_health(service)
            if health['healthy']:
                print(f"  ✅ HTTP Health: OK ({health['response_time']:.3f}s)")
            else:
                print(f"  ❌ HTTP Health: {health['error']}")
                all_healthy = False
            
            # Overall status
            if (service_status['status'] == 'RUNNING' and 
                port_in_use and 
                health['healthy']):
                print(f"  🟢 Overall: HEALTHY")
            elif service_status['status'] == 'STOPPED':
                print(f"  🔴 Overall: STOPPED")
            elif service_status['status'] == 'PAUSED':
                print(f"  🟡 Overall: PAUSED")
            else:
                print(f"  🟠 Overall: PARTIAL")
                all_healthy = False
        
        print("\n" + "=" * 80)
        
        if all_healthy:
            self.log("All services are healthy! 🎉", "SUCCESS")
        else:
            self.log("Some services have issues. Check the details above.", "WARNING")
        
        return all_healthy
    
    def show_service_urls(self):
        """Show service URLs for easy access"""
        print("\n🌐 Service URLs:")
        print("-" * 40)
        for service in self.services:
            print(f"  {service['name']}: {service['url']}")
        print(f"  API Gateway Health: {self.services[0]['url']}/health")
        print(f"  Chaos Backend Health: {self.services[1]['url']}/health")
        print(f"  CMS Service Health: {self.services[2]['url']}/health")

def main():
    checker = ServiceChecker()
    
    if len(sys.argv) > 1 and sys.argv[1] == "help":
        print("Usage: python check_services.py")
        print("This script checks the status of all Chaos World services")
        return
    
    success = checker.check_all_services()
    checker.show_service_urls()
    
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
