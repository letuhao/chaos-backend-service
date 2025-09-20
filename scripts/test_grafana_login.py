#!/usr/bin/env python3
"""
Test Grafana Login
Try different common passwords to find the correct one
"""

import requests
import time

def test_login(username, password):
    """Test login with given credentials"""
    try:
        response = requests.get("http://localhost:3001/api/org", 
                              auth=(username, password), timeout=5)
        if response.status_code == 200:
            return True, response.json()
        else:
            return False, f"Status: {response.status_code}"
    except Exception as e:
        return False, str(e)

def main():
    """Test different login combinations"""
    print("🔐 Testing Grafana Login Credentials")
    print("=" * 40)
    
    # Common password combinations
    passwords_to_try = [
        "admin",
        "admin123", 
        "Ab123456",
        "password",
        "123456",
        "grafana",
        "admin123456"
    ]
    
    username = "admin"
    
    for password in passwords_to_try:
        print(f"🔍 Trying: {username} / {password}")
        success, result = test_login(username, password)
        
        if success:
            print(f"✅ SUCCESS! Login works with: {username} / {password}")
            print(f"📊 Org info: {result}")
            return password
        else:
            print(f"❌ Failed: {result}")
    
    print("\n❌ None of the common passwords worked!")
    print("💡 You may need to:")
    print("   1. Check Grafana logs for the actual password")
    print("   2. Reset the admin password")
    print("   3. Check if Grafana is using a different auth method")
    
    return None

if __name__ == "__main__":
    main()
