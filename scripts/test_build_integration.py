#!/usr/bin/env python3
"""
Test script to verify build integration works correctly
This script tests the build and file checking logic without requiring admin privileges
"""

import os
import sys
from pathlib import Path

def test_build_integration():
    """Test that build integration works correctly"""
    print("🧪 Testing Build Integration")
    print("=" * 40)
    
    # Get workspace root
    script_dir = Path(__file__).parent
    workspace_root = script_dir.parent
    target_dir = workspace_root / "target" / "release"
    
    print(f"📁 Workspace root: {workspace_root}")
    print(f"📁 Target directory: {target_dir}")
    
    # Check if executables exist
    executables = [
        "api-gateway.exe",
        "chaos-backend.exe", 
        "content-management-service.exe"
    ]
    
    all_found = True
    for exe in executables:
        exe_path = target_dir / exe
        if exe_path.exists():
            size = exe_path.stat().st_size
            print(f"✅ {exe} found ({size:,} bytes)")
        else:
            print(f"❌ {exe} not found")
            all_found = False
    
    if all_found:
        print("\n🎉 All executables found! Build integration is working correctly.")
        print("You can now run 'python install_services.py' as Administrator to install services.")
        return True
    else:
        print("\n❌ Some executables are missing. Run 'python build_services.py' first.")
        return False

if __name__ == "__main__":
    success = test_build_integration()
    sys.exit(0 if success else 1)