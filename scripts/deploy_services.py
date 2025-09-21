#!/usr/bin/env python3
"""
Deploy Chaos World Backend Services
A Python script to orchestrate the deployment process:
1. Stop all services
2. Copy executables (build and install)
3. Start all services
"""

import os
import sys
import subprocess
import time
from pathlib import Path

class ServiceDeployer:
    def __init__(self):
        self.script_dir = Path(__file__).parent
        self.project_root = self.script_dir.parent
        
        # Paths to the existing scripts
        self.stop_script = self.script_dir / "stop_services.py"
        self.install_script = self.script_dir / "install_services.py"
        self.copy_script = self.script_dir / "copy_services.py"
        self.start_script = self.script_dir / "start_services.py"
        self.build_script = self.script_dir / "build_services.py"
        
        # Verify scripts exist
        self.verify_scripts()
    
    def verify_scripts(self):
        """Verify that all required scripts exist"""
        scripts = [self.stop_script, self.copy_script, self.start_script, self.build_script]
        for script in scripts:
            if not script.exists():
                self.log(f"❌ Required script not found: {script}", "ERROR")
                sys.exit(1)
    
    def log(self, message: str, level: str = "INFO"):
        """Log a message with timestamp"""
        timestamp = time.strftime("%H:%M:%S")
        print(f"[{timestamp}] {level}: {message}")
    
    def run_script(self, script_path: Path, args: list = None) -> bool:
        """Run a Python script and return success status"""
        try:
            cmd = [sys.executable, str(script_path)]
            if args:
                cmd.extend(args)
            
            self.log(f"Running: {' '.join(cmd)}")
            result = subprocess.run(cmd, cwd=self.project_root, capture_output=True, text=True)
            
            if result.returncode == 0:
                self.log(f"✅ Script completed successfully: {script_path.name}")
                if result.stdout:
                    print(result.stdout)
                return True
            else:
                self.log(f"❌ Script failed: {script_path.name}", "ERROR")
                if result.stderr:
                    print(f"Error: {result.stderr}")
                return False
                
        except Exception as e:
            self.log(f"❌ Exception running script {script_path.name}: {e}", "ERROR")
            return False
    
    def check_admin(self) -> bool:
        """Check if running as administrator"""
        try:
            import ctypes
            return ctypes.windll.shell32.IsUserAnAdmin()
        except:
            return False
    
    def stop_services(self) -> bool:
        """Stop all services using stop_services.py"""
        self.log("🛑 Step 1: Stopping all services...")
        print("=" * 50)
        
        return self.run_script(self.stop_script)
    
    def build_services(self) -> bool:
        """Build all services using build_services.py"""
        self.log("🔨 Step 2: Building all services...")
        print("=" * 50)
        
        return self.run_script(self.build_script)
    
    def copy_services(self) -> bool:
        """Copy services using copy_services.py"""
        self.log("📦 Step 3: Copying services...")
        print("=" * 50)
        
        return self.run_script(self.copy_script)
    
    def start_services(self) -> bool:
        """Start all services using start_services.py"""
        self.log("🚀 Step 4: Starting all services...")
        print("=" * 50)
        
        return self.run_script(self.start_script)
    
    def deploy_all(self, skip_build: bool = False) -> bool:
        """Deploy all services"""
        self.log("🚀 Starting Chaos World Services Deployment")
        print("=" * 60)
        
        # Check admin privileges once at the beginning
        if not self.check_admin():
            self.log("⚠️  Warning: Not running as Administrator", "WARNING")
            self.log("Some operations may fail. Consider running as Administrator for full functionality.")
            print()
        
        start_time = time.time()
        
        # Step 1: Stop services
        if not self.stop_services():
            self.log("❌ Failed to stop services. Aborting deployment.", "ERROR")
            return False
        
        # Wait a moment for services to fully stop
        self.log("⏳ Waiting for services to fully stop...")
        time.sleep(3)
        
        # Step 2: Build services (optional)
        if not skip_build:
            if not self.build_services():
                self.log("❌ Failed to build services. Aborting deployment.", "ERROR")
                return False
        else:
            self.log("⏭️  Skipping build step (--skip-build specified)")
        
        # Step 3: Copy services
        if not self.copy_services():
            self.log("❌ Failed to copy services. Aborting deployment.", "ERROR")
            return False
        
        # Step 4: Start services
        if not self.start_services():
            self.log("❌ Failed to start services. Aborting deployment.", "ERROR")
            return False
        
        # Calculate deployment time
        end_time = time.time()
        deployment_time = end_time - start_time
        
        self.log("✅ Deployment completed successfully!")
        self.log(f"⏱️  Total deployment time: {deployment_time:.2f} seconds")
        
        return True
    
    def quick_deploy(self) -> bool:
        """Quick deploy - skip build step"""
        return self.deploy_all(skip_build=True)
    
    def full_deploy(self) -> bool:
        """Full deploy - including build step"""
        return self.deploy_all(skip_build=False)

def main():
    """Main function"""
    deployer = ServiceDeployer()
    
    if len(sys.argv) > 1:
        command = sys.argv[1].lower()
        
        if command == "help":
            print("🚀 Chaos World Services Deployer")
            print("=" * 40)
            print("Usage:")
            print("  python deploy_services.py [command]")
            print()
            print("Commands:")
            print("  help           - Show this help message")
            print("  stop           - Stop all services only")
            print("  build          - Build all services only")
            print("  copy           - Copy services only")
            print("  install        - Install/copy services only")
            print("  start          - Start all services only")
            print("  quick          - Quick deploy (skip build)")
            print("  full           - Full deploy (with build)")
            print("  deploy         - Same as 'full' (default)")
            print()
            print("Examples:")
            print("  python deploy_services.py quick    # Quick deploy without building")
            print("  python deploy_services.py full     # Full deploy with building")
            print("  python deploy_services.py stop     # Stop services only")
            return
        
        elif command == "stop":
            success = deployer.stop_services()
        elif command == "build":
            success = deployer.build_services()
        elif command == "copy":
            success = deployer.copy_services()
        elif command == "install":
            success = deployer.install_services()
        elif command == "start":
            success = deployer.start_services()
        elif command == "quick":
            success = deployer.quick_deploy()
        elif command == "full" or command == "deploy":
            success = deployer.full_deploy()
        else:
            print(f"❌ Unknown command: {command}")
            print("Use 'python deploy_services.py help' for usage information")
            sys.exit(1)
    else:
        # Default to full deploy
        success = deployer.full_deploy()
    
    if success:
        print("\n🎉 Operation completed successfully!")
        sys.exit(0)
    else:
        print("\n💥 Operation failed!")
        sys.exit(1)

if __name__ == "__main__":
    main()
