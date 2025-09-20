#!/usr/bin/env python3
"""
Generate Test Data for Monitoring
Creates some test traffic to populate the monitoring dashboards
"""

import requests
import time
import random
import threading
from datetime import datetime

class TestDataGenerator:
    def __init__(self):
        self.cms_url = "http://localhost:8083"
        self.api_gateway_url = "http://localhost:8080"
        self.running = False
        
    def generate_cms_traffic(self):
        """Generate test traffic for CMS service"""
        print("🚀 Generating CMS test traffic...")
        
        endpoints = [
            "/health",
            "/api/v1/health", 
            "/api/v1/metrics/info",
            "/api/v1/auth/login"
        ]
        
        while self.running:
            try:
                # Random endpoint
                endpoint = random.choice(endpoints)
                url = f"{self.cms_url}{endpoint}"
                
                # Random delay between requests
                delay = random.uniform(0.1, 2.0)
                time.sleep(delay)
                
                # Make request
                if endpoint == "/api/v1/auth/login":
                    # Login request with JSON body
                    response = requests.post(url, 
                        json={"username": "admin", "password": "admin123"},
                        timeout=5)
                else:
                    # GET request
                    response = requests.get(url, timeout=5)
                
                if response.status_code == 200:
                    print(f"✅ {endpoint} - {response.status_code}")
                else:
                    print(f"⚠️ {endpoint} - {response.status_code}")
                    
            except Exception as e:
                print(f"❌ Error: {e}")
                time.sleep(1)
    
    def generate_api_gateway_traffic(self):
        """Generate test traffic for API Gateway"""
        print("🚀 Generating API Gateway test traffic...")
        
        endpoints = [
            "/health",
            "/api/v1/status"
        ]
        
        while self.running:
            try:
                endpoint = random.choice(endpoints)
                url = f"{self.api_gateway_url}{endpoint}"
                
                delay = random.uniform(0.5, 3.0)
                time.sleep(delay)
                
                response = requests.get(url, timeout=5)
                
                if response.status_code == 200:
                    print(f"✅ Gateway {endpoint} - {response.status_code}")
                else:
                    print(f"⚠️ Gateway {endpoint} - {response.status_code}")
                    
            except Exception as e:
                print(f"❌ Gateway Error: {e}")
                time.sleep(1)
    
    def start_traffic_generation(self, duration_minutes=5):
        """Start generating test traffic for specified duration"""
        print(f"🎯 Starting test traffic generation for {duration_minutes} minutes...")
        print("Press Ctrl+C to stop early")
        
        self.running = True
        
        # Start traffic generation threads
        cms_thread = threading.Thread(target=self.generate_cms_traffic)
        gateway_thread = threading.Thread(target=self.generate_api_gateway_traffic)
        
        cms_thread.daemon = True
        gateway_thread.daemon = True
        
        cms_thread.start()
        gateway_thread.start()
        
        try:
            # Run for specified duration
            time.sleep(duration_minutes * 60)
        except KeyboardInterrupt:
            print("\n🛑 Stopping traffic generation...")
        finally:
            self.running = False
            print("✅ Traffic generation stopped")
    
    def check_services(self):
        """Check if services are running"""
        print("🔍 Checking services...")
        
        services = [
            ("CMS Service", f"{self.cms_url}/health"),
            ("API Gateway", f"{self.api_gateway_url}/health"),
            ("Prometheus", "http://localhost:9091"),
            ("Grafana", "http://localhost:3001")
        ]
        
        for name, url in services:
            try:
                response = requests.get(url, timeout=5)
                if response.status_code == 200:
                    print(f"✅ {name}: Running")
                else:
                    print(f"⚠️ {name}: Status {response.status_code}")
            except Exception as e:
                print(f"❌ {name}: Not accessible - {e}")

def main():
    """Main function"""
    generator = TestDataGenerator()
    
    print("🧪 Chaos World Test Data Generator")
    print("=" * 40)
    
    # Check services first
    generator.check_services()
    print()
    
    # Ask user for duration
    try:
        duration = input("Enter duration in minutes (default 5): ").strip()
        duration = int(duration) if duration else 5
    except ValueError:
        duration = 5
    
    # Start traffic generation
    generator.start_traffic_generation(duration)
    
    print("\n📊 Check your Grafana dashboards now!")
    print("📈 Grafana: http://localhost:3001")
    print("📊 Prometheus: http://localhost:9091")

if __name__ == "__main__":
    main()
