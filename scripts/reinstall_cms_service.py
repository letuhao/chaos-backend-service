#!/usr/bin/env python3
"""
Reinstall CMS Service with Correct Configuration
This script reinstalls the CMS service with the proper port and environment variables
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

def reinstall_cms_service():
    """Reinstall CMS service with correct configuration"""
    print("🔧 Reinstalling CMS Service with Correct Configuration")
    print("=" * 60)
    
    if not check_admin():
        print("❌ This script must be run as Administrator!")
        print("Right-click PowerShell/Command Prompt and select 'Run as administrator'")
        return False
    
    print("✅ Running as Administrator")
    
    try:
        # Stop the service if it's running
        print("🛑 Stopping CMS service...")
        subprocess.run(["nssm", "stop", "ChaosWorld-CMS"], capture_output=True)
        
        # Remove the service
        print("🗑️ Removing CMS service...")
        subprocess.run(["nssm", "remove", "ChaosWorld-CMS", "confirm"], capture_output=True)
        
        # Wait a moment
        import time
        time.sleep(2)
        
        # Reinstall the service with correct configuration
        print("🔧 Reinstalling CMS service...")
        
        # Install service
        exe_path = r"C:\ChaosWorld\services\content-management-service.exe"
        result = subprocess.run([
            "nssm", "install", "ChaosWorld-CMS", exe_path
        ], capture_output=True, text=True)
        
        if result.returncode != 0:
            print(f"❌ Failed to install service: {result.stderr}")
            return False
        
        print("✅ Service installed")
        
        # Configure service
        configs = [
            ("AppDirectory", r"C:\ChaosWorld\services"),
            ("AppParameters", "--port 8083"),
            ("AppStdout", r"C:\ChaosWorld\logs\ChaosWorld-CMS.log"),
            ("AppStderr", r"C:\ChaosWorld\logs\ChaosWorld-CMS-error.log"),
            ("AppRotateFiles", "1"),
            ("AppRotateOnline", "1"),
            ("AppRotateBytes", "1048576"),
            ("Start", "SERVICE_AUTO_START"),
            ("DisplayName", "Chaos World CMS"),
            ("Description", "Content Management Service for Chaos World")
        ]
        
        for key, value in configs:
            result = subprocess.run([
                "nssm", "set", "ChaosWorld-CMS", key, value
            ], capture_output=True, text=True)
            
            if result.returncode != 0:
                print(f"❌ Failed to set {key}: {result.stderr}")
                return False
        
        print("✅ Service configuration set")
        
        # Set environment variables
        env_vars = [
            "RUST_LOG=info",
            "CMS_PORT=8083",
            "APP_ENV=prod"
        ]
        
        for env_var in env_vars:
            result = subprocess.run([
                "nssm", "set", "ChaosWorld-CMS", "AppEnvironmentExtra", env_var
            ], capture_output=True, text=True)
            
            if result.returncode != 0:
                print(f"❌ Failed to set environment variable {env_var}: {result.stderr}")
                return False
        
        print("✅ Environment variables set")
        
        # Start the service
        print("🚀 Starting CMS service...")
        result = subprocess.run([
            "nssm", "start", "ChaosWorld-CMS"
        ], capture_output=True, text=True)
        
        if result.returncode != 0:
            print(f"❌ Failed to start service: {result.stderr}")
            return False
        
        print("✅ CMS service started")
        
        # Wait for service to initialize
        print("⏳ Waiting for service to initialize...")
        time.sleep(5)
        
        # Test the service
        print("🧪 Testing CMS service...")
        try:
            import requests
            response = requests.get("http://localhost:8083/health", timeout=10)
            if response.status_code == 200:
                print("✅ CMS service is responding on port 8083!")
                return True
            else:
                print(f"⚠️ CMS service responded with status {response.status_code}")
                print(f"Response: {response.text}")
                return False
        except Exception as e:
            print(f"❌ CMS service test failed: {e}")
            print("Check the service logs at C:\\ChaosWorld\\logs\\ChaosWorld-CMS.log")
            return False
            
    except Exception as e:
        print(f"❌ Error reinstalling CMS service: {e}")
        return False

def main():
    """Main function"""
    print("🚀 CMS Service Reinstaller")
    print("=" * 60)
    
    if reinstall_cms_service():
        print("\n🎉 CMS service reinstalled successfully!")
        print("The service should now be running on port 8083")
        print("Check status with: python check_services.py")
    else:
        print("\n❌ Failed to reinstall CMS service")
        sys.exit(1)

if __name__ == "__main__":
    main()
