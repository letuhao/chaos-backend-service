#!/usr/bin/env python3
"""
Fix CMS Service NSSM Configuration
This script fixes the CMS service configuration to use the correct port
"""

import os
import sys
import subprocess
import ctypes

def check_admin():
    """Check if running as administrator"""
    try:
        return ctypes.windll.shell32.IsUserAnAdmin()
    except:
        return False

def fix_cms_service():
    """Fix CMS service configuration"""
    print("🔧 Fixing CMS Service NSSM Configuration")
    print("=" * 50)
    
    if not check_admin():
        print("❌ This script must be run as Administrator!")
        print("Right-click PowerShell/Command Prompt and select 'Run as administrator'")
        return False
    
    print("✅ Running as Administrator")
    
    # Set the correct parameters for CMS service
    print("🔧 Setting CMS service parameters...")
    
    try:
        # Set port parameter
        result = subprocess.run([
            "nssm", "set", "ChaosWorld-CMS", "AppParameters", "--port", "8083"
        ], capture_output=True, text=True)
        
        if result.returncode != 0:
            print(f"❌ Failed to set AppParameters: {result.stderr}")
            return False
        
        print("✅ AppParameters set to --port 8083")
        
        # Set working directory to ensure config files are found
        result = subprocess.run([
            "nssm", "set", "ChaosWorld-CMS", "AppDirectory", "C:\\ChaosWorld\\services"
        ], capture_output=True, text=True)
        
        if result.returncode != 0:
            print(f"❌ Failed to set AppDirectory: {result.stderr}")
            return False
        
        print("✅ AppDirectory set to C:\\ChaosWorld\\services")
        
        # Set environment variables
        result = subprocess.run([
            "nssm", "set", "ChaosWorld-CMS", "AppEnvironmentExtra", "CMS_PORT=8083"
        ], capture_output=True, text=True)
        
        if result.returncode != 0:
            print(f"❌ Failed to set environment variable: {result.stderr}")
            return False
        
        print("✅ Environment variable CMS_PORT=8083 set")
        
        # Restart the service
        print("🔄 Restarting CMS service...")
        result = subprocess.run([
            "nssm", "restart", "ChaosWorld-CMS"
        ], capture_output=True, text=True)
        
        if result.returncode != 0:
            print(f"❌ Failed to restart service: {result.stderr}")
            return False
        
        print("✅ CMS service restarted")
        
        # Wait a moment for service to start
        import time
        print("⏳ Waiting for service to initialize...")
        time.sleep(5)
        
        # Test the service
        print("🧪 Testing CMS service...")
        try:
            import requests
            response = requests.get("http://localhost:8083/health", timeout=10)
            if response.status_code == 200:
                print("✅ CMS service is responding!")
                return True
            else:
                print(f"⚠️ CMS service responded with status {response.status_code}")
                return False
        except Exception as e:
            print(f"❌ CMS service test failed: {e}")
            return False
            
    except Exception as e:
        print(f"❌ Error fixing CMS service: {e}")
        return False

def main():
    """Main function"""
    print("🚀 CMS Service Configuration Fixer")
    print("=" * 50)
    
    if fix_cms_service():
        print("\n🎉 CMS service configuration fixed successfully!")
        print("The service should now be running on port 8083")
    else:
        print("\n❌ Failed to fix CMS service configuration")
        sys.exit(1)

if __name__ == "__main__":
    main()
