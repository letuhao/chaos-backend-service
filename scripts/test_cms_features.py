#!/usr/bin/env python3
"""
Test CMS Service Features
Comprehensive testing of all CMS service endpoints
"""

import requests
import json
import time
from typing import Dict, Any

class CMSTester:
    def __init__(self, base_url: str = "http://localhost:8083"):
        self.base_url = base_url
        self.session = requests.Session()
        self.token = None
    
    def test_endpoint(self, method: str, endpoint: str, data: Dict = None, headers: Dict = None, expected_status: int = 200) -> Dict[str, Any]:
        """Test a single endpoint"""
        url = f"{self.base_url}{endpoint}"
        
        try:
            if method.upper() == "GET":
                response = self.session.get(url, headers=headers, timeout=10)
            elif method.upper() == "POST":
                response = self.session.post(url, json=data, headers=headers, timeout=10)
            else:
                return {"error": f"Unsupported method: {method}"}
            
            result = {
                "endpoint": endpoint,
                "method": method,
                "status_code": response.status_code,
                "success": response.status_code == expected_status,
                "response": response.json() if response.headers.get('content-type', '').startswith('application/json') else response.text
            }
            
            return result
            
        except Exception as e:
            return {
                "endpoint": endpoint,
                "method": method,
                "status_code": 0,
                "success": False,
                "error": str(e)
            }
    
    def test_health_endpoints(self):
        """Test health and status endpoints"""
        print("🏥 Testing Health Endpoints")
        print("-" * 40)
        
        # Test root endpoint
        result = self.test_endpoint("GET", "/")
        print(f"GET / - Status: {result['status_code']} - {'✅' if result['success'] else '❌'}")
        if result['success']:
            print(f"  Response: {result['response']['data']}")
        
        # Test health endpoint
        result = self.test_endpoint("GET", "/health")
        print(f"GET /health - Status: {result['status_code']} - {'✅' if result['success'] else '❌'}")
        if result['success']:
            print(f"  Response: {result['response']['data']}")
        
        print()
    
    def test_auth_endpoints(self):
        """Test authentication endpoints"""
        print("🔐 Testing Authentication Endpoints")
        print("-" * 40)
        
        # Test login
        login_data = {"username": "admin", "password": "admin123"}
        result = self.test_endpoint("POST", "/api/v1/auth/login", login_data)
        print(f"POST /api/v1/auth/login - Status: {result['status_code']} - {'✅' if result['success'] else '❌'}")
        
        if result['success'] and 'data' in result['response']:
            self.token = result['response']['data']['token']
            print(f"  Token received: {self.token[:50]}...")
            print(f"  Expires in: {result['response']['data']['expires_in']} seconds")
        else:
            print(f"  Error: {result.get('error', 'Unknown error')}")
        
        # Test me endpoint
        if self.token:
            headers = {"Authorization": f"Bearer {self.token}"}
            result = self.test_endpoint("GET", "/api/v1/auth/me", headers=headers)
            print(f"GET /api/v1/auth/me - Status: {result['status_code']} - {'✅' if result['success'] else '❌'}")
            if result['success']:
                print(f"  User: {result['response']['data']['username']} ({result['response']['data']['role']})")
        
        print()
    
    def test_monitoring_endpoints(self):
        """Test monitoring endpoints"""
        print("📊 Testing Monitoring Endpoints")
        print("-" * 40)
        
        # Test health endpoint (monitoring)
        result = self.test_endpoint("GET", "/api/v1/health")
        print(f"GET /api/v1/health - Status: {result['status_code']} - {'✅' if result['success'] else '❌'}")
        if result['success']:
            print(f"  Response: {result['response']}")
        
        # Test metrics info endpoint
        result = self.test_endpoint("GET", "/api/v1/metrics/info")
        print(f"GET /api/v1/metrics/info - Status: {result['status_code']} - {'✅' if result['success'] else '❌'}")
        if result['success']:
            print(f"  Response: {result['response']}")
        
        print()
    
    def test_protected_endpoints(self):
        """Test protected endpoints"""
        print("🔒 Testing Protected Endpoints")
        print("-" * 40)
        
        if not self.token:
            print("❌ No token available, skipping protected endpoint tests")
            return
        
        headers = {"Authorization": f"Bearer {self.token}"}
        
        # Test admin endpoint
        result = self.test_endpoint("GET", "/api/v1/admin", headers=headers)
        print(f"GET /api/v1/admin - Status: {result['status_code']} - {'✅' if result['success'] else '❌'}")
        if result['success']:
            print(f"  Response: {result['response']['data']['message']}")
        else:
            print(f"  Error: {result.get('error', 'Unknown error')}")
        
        print()
    
    def test_metrics_server(self):
        """Test the dedicated metrics server"""
        print("📈 Testing Metrics Server")
        print("-" * 40)
        
        try:
            response = requests.get("http://localhost:9090/metrics", timeout=10)
            if response.status_code == 200:
                print("✅ Metrics server is responding")
                metrics_text = response.text
                
                # Count different metric types
                lines = metrics_text.split('\n')
                help_lines = [line for line in lines if line.startswith('# HELP')]
                metric_lines = [line for line in lines if not line.startswith('#') and line.strip()]
                
                print(f"  Found {len(help_lines)} metric types")
                print(f"  Found {len(metric_lines)} metric values")
                
                # Show some example metrics
                print("  Sample metrics:")
                for line in metric_lines[:5]:
                    if line.strip():
                        print(f"    {line}")
            else:
                print(f"❌ Metrics server returned status {response.status_code}")
        except Exception as e:
            print(f"❌ Metrics server test failed: {e}")
        
        print()
    
    def run_all_tests(self):
        """Run all tests"""
        print("🧪 CMS Service Feature Testing")
        print("=" * 50)
        print()
        
        self.test_health_endpoints()
        self.test_auth_endpoints()
        self.test_monitoring_endpoints()
        self.test_protected_endpoints()
        self.test_metrics_server()
        
        print("🎉 Testing completed!")

def main():
    """Main function"""
    tester = CMSTester()
    tester.run_all_tests()

if __name__ == "__main__":
    main()
