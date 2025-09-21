#!/usr/bin/env python3
"""
Copy Chaos World Backend Services
A Python script to copy built executables to the service directory
"""

import os
import sys
import shutil
from pathlib import Path

class ServiceCopier:
    def __init__(self):
        self.project_root = Path(__file__).parent.parent
        self.service_dir = Path("C:/ChaosWorld/services")
        self.target_dir = self.project_root / "target" / "release"
        
        # Service executables to copy
        self.services = {
            "api-gateway.exe": "api-gateway.exe",
            "chaos-backend.exe": "chaos-backend.exe", 
            "content-management-service.exe": "content-management-service.exe",
            "user-management.exe": "user-management.exe"
        }
    
    def log(self, message: str, level: str = "INFO"):
        """Log a message with timestamp"""
        import time
        timestamp = time.strftime("%H:%M:%S")
        print(f"[{timestamp}] {level}: {message}")
    
    def copy_executables(self) -> bool:
        """Copy all service executables"""
        self.log("📦 Copying service executables...")
        
        # Check if target directory exists
        if not self.target_dir.exists():
            self.log(f"❌ Target directory not found: {self.target_dir}", "ERROR")
            return False
        
        # Check if service directory exists
        if not self.service_dir.exists():
            self.log(f"❌ Service directory not found: {self.service_dir}", "ERROR")
            self.log("Please run install_services.py first to create the service directory", "ERROR")
            return False
        
        success = True
        
        for source_name, dest_name in self.services.items():
            source_path = self.target_dir / source_name
            dest_path = self.service_dir / dest_name
            
            if source_path.exists():
                try:
                    # Copy the file
                    shutil.copy2(source_path, dest_path)
                    self.log(f"✅ Copied {source_name} -> {dest_name}")
                except Exception as e:
                    self.log(f"❌ Failed to copy {source_name}: {e}", "ERROR")
                    success = False
            else:
                self.log(f"❌ Source file not found: {source_path}", "ERROR")
                success = False
        
        return success
    
    def verify_copies(self) -> bool:
        """Verify that all files were copied successfully"""
        self.log("🔍 Verifying copied files...")
        
        all_good = True
        
        for source_name, dest_name in self.services.items():
            dest_path = self.service_dir / dest_name
            
            if dest_path.exists():
                size = dest_path.stat().st_size
                self.log(f"✅ {dest_name} - {size:,} bytes")
            else:
                self.log(f"❌ {dest_name} - NOT FOUND", "ERROR")
                all_good = False
        
        return all_good

def main():
    """Main function"""
    copier = ServiceCopier()
    
    if len(sys.argv) > 1 and sys.argv[1] == "help":
        print("📦 Chaos World Services Copier")
        print("=" * 40)
        print("Usage:")
        print("  python copy_services.py [command]")
        print()
        print("Commands:")
        print("  help    - Show this help message")
        print("  copy    - Copy executables (default)")
        print("  verify  - Verify copied files")
        print()
        print("This script copies built executables from target/release/")
        print("to C:/ChaosWorld/services/ without requiring admin privileges.")
        return
    
    command = sys.argv[1] if len(sys.argv) > 1 else "copy"
    
    if command == "copy":
        success = copier.copy_executables()
        if success:
            copier.verify_copies()
    elif command == "verify":
        success = copier.verify_copies()
    else:
        print(f"❌ Unknown command: {command}")
        print("Use 'python copy_services.py help' for usage information")
        sys.exit(1)
    
    if success:
        print("\n🎉 Operation completed successfully!")
        sys.exit(0)
    else:
        print("\n💥 Operation failed!")
        sys.exit(1)

if __name__ == "__main__":
    main()
