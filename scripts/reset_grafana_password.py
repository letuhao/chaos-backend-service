#!/usr/bin/env python3
"""
Reset Grafana Admin Password
Reset the admin password to a known value
"""

import subprocess
import time
import requests
from pathlib import Path

def reset_grafana_password():
    """Reset Grafana admin password"""
    print("🔧 Resetting Grafana admin password...")
    
    grafana_path = r"C:\ProgramData\chocolatey\lib\grafana\tools\grafana-11.5.8"
    grafana_cli = Path(grafana_path) / "bin" / "grafana-cli.exe"
    
    if not grafana_cli.exists():
        print(f"❌ Grafana CLI not found: {grafana_cli}")
        return False
    
    try:
        # Stop Grafana first
        print("🛑 Stopping Grafana...")
        subprocess.run(["taskkill", "/F", "/IM", "grafana-server.exe"], 
                      capture_output=True, check=False)
        time.sleep(3)
        
        # Reset admin password
        print("🔑 Resetting admin password to 'admin123'...")
        cmd = [
            str(grafana_cli),
            "admin",
            "reset-admin-password",
            "admin123"
        ]
        
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
        
        if result.returncode == 0:
            print("✅ Password reset successful!")
            print("🔑 New credentials: admin / admin123")
            return True
        else:
            print(f"❌ Password reset failed: {result.stderr}")
            return False
            
    except Exception as e:
        print(f"❌ Error resetting password: {e}")
        return False

def start_grafana():
    """Start Grafana with the new password"""
    print("🚀 Starting Grafana...")
    
    grafana_exe = Path(r"C:\ProgramData\chocolatey\lib\grafana\tools\grafana-11.5.8\bin\grafana-server.exe")
    custom_config = Path("C:/ChaosWorld/monitoring/grafana/grafana.ini")
    
    cmd = [
        str(grafana_exe),
        f"--config={custom_config}",
        f"--homepath={grafana_exe.parent.parent}"
    ]
    
    try:
        subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        time.sleep(10)
        
        # Test if Grafana is running
        response = requests.get("http://localhost:3001", timeout=10)
        if response.status_code == 200:
            print("✅ Grafana started successfully!")
            return True
        else:
            print(f"❌ Grafana not responding: {response.status_code}")
            return False
            
    except Exception as e:
        print(f"❌ Error starting Grafana: {e}")
        return False

def test_new_credentials():
    """Test the new credentials"""
    print("🧪 Testing new credentials...")
    
    try:
        response = requests.get("http://localhost:3001/api/org", 
                              auth=("admin", "admin123"), timeout=10)
        if response.status_code == 200:
            print("✅ Login successful with admin / admin123")
            return True
        else:
            print(f"❌ Login failed: {response.status_code}")
            return False
    except Exception as e:
        print(f"❌ Login test error: {e}")
        return False

def main():
    """Main function"""
    print("🔧 Grafana Password Reset")
    print("=" * 30)
    
    # Reset password
    if not reset_grafana_password():
        print("❌ Failed to reset password")
        return
    
    # Start Grafana
    if not start_grafana():
        print("❌ Failed to start Grafana")
        return
    
    # Test credentials
    if test_new_credentials():
        print("\n🎉 Success! You can now login to Grafana with:")
        print("📈 URL: http://localhost:3001")
        print("🔑 Username: admin")
        print("🔑 Password: admin123")
    else:
        print("\n⚠️ Password reset completed but login test failed")
        print("Try logging in manually at http://localhost:3001")

if __name__ == "__main__":
    main()
