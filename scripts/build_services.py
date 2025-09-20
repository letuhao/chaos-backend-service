#!/usr/bin/env python3
"""
Chaos World Services Build Script
Builds and copies all service executables to the target directory
"""

import os
import sys
import subprocess
import shutil
from pathlib import Path
from typing import List, Dict, Optional
import logging

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('build_services.log'),
        logging.StreamHandler(sys.stdout)
    ]
)
logger = logging.getLogger(__name__)

class ServiceBuilder:
    def __init__(self, workspace_root: Path):
        self.workspace_root = workspace_root
        self.target_dir = workspace_root / "target" / "release"
        self.services_dir = workspace_root / "services"
        
        # Service configurations
        self.services = {
            "api-gateway": {
                "path": self.services_dir / "api-gateway",
                "binary": "api-gateway.exe",
                "cargo_args": ["--bin", "api-gateway"]
            },
            "chaos-backend": {
                "path": self.services_dir / "chaos-backend",
                "binary": "chaos-backend.exe", 
                "cargo_args": ["--bin", "chaos-backend"]
            },
            "content-management-service": {
                "path": self.services_dir / "content-management-service",
                "binary": "content-management-service.exe",
                "cargo_args": ["--bin", "content-management-service"]
            }
        }
    
    def check_prerequisites(self) -> bool:
        """Check if required tools are available"""
        logger.info("🔍 Checking prerequisites...")
        
        # Check if cargo is available
        try:
            result = subprocess.run(["cargo", "--version"], capture_output=True, text=True)
            if result.returncode != 0:
                logger.error("❌ Cargo not found. Please install Rust toolchain.")
                return False
            logger.info(f"✅ Cargo found: {result.stdout.strip()}")
        except FileNotFoundError:
            logger.error("❌ Cargo not found. Please install Rust toolchain.")
            return False
        
        # Check if target directory exists
        if not self.target_dir.exists():
            logger.info("📁 Creating target directory...")
            self.target_dir.mkdir(parents=True, exist_ok=True)
        
        return True
    
    def build_service(self, service_name: str, service_config: Dict) -> bool:
        """Build a single service"""
        logger.info(f"🔨 Building {service_name}...")
        
        service_path = service_config["path"]
        if not service_path.exists():
            logger.error(f"❌ Service directory not found: {service_path}")
            return False
        
        # Change to service directory
        original_cwd = os.getcwd()
        try:
            os.chdir(service_path)
            
            # Build the service
            cmd = ["cargo", "build", "--release"] + service_config["cargo_args"]
            logger.info(f"Running: {' '.join(cmd)}")
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            if result.returncode != 0:
                logger.error(f"❌ Failed to build {service_name}")
                logger.error(f"Error: {result.stderr}")
                return False
            
            logger.info(f"✅ {service_name} built successfully")
            return True
            
        except Exception as e:
            logger.error(f"❌ Error building {service_name}: {e}")
            return False
        finally:
            os.chdir(original_cwd)
    
    def copy_executable(self, service_name: str, service_config: Dict) -> bool:
        """Verify executable exists in target directory"""
        logger.info(f"📋 Verifying {service_name} executable...")
        
        binary_name = service_config["binary"]
        target_path = self.target_dir / binary_name
        
        if not target_path.exists():
            logger.error(f"❌ Executable not found: {target_path}")
            return False
        
        # Check file size to ensure it's not empty
        file_size = target_path.stat().st_size
        if file_size == 0:
            logger.error(f"❌ Executable is empty: {target_path}")
            return False
        
        logger.info(f"✅ {binary_name} found and ready ({file_size} bytes)")
        return True
    
    def build_all_services(self) -> bool:
        """Build all services"""
        logger.info("🚀 Starting build process for all services...")
        
        if not self.check_prerequisites():
            return False
        
        success_count = 0
        total_count = len(self.services)
        
        for service_name, service_config in self.services.items():
            logger.info(f"\n{'='*50}")
            logger.info(f"Building {service_name.upper()}")
            logger.info(f"{'='*50}")
            
            if self.build_service(service_name, service_config):
                if self.copy_executable(service_name, service_config):
                    success_count += 1
                    logger.info(f"✅ {service_name} completed successfully")
                else:
                    logger.error(f"❌ {service_name} copy failed")
            else:
                logger.error(f"❌ {service_name} build failed")
        
        logger.info(f"\n{'='*50}")
        logger.info(f"BUILD SUMMARY: {success_count}/{total_count} services built successfully")
        logger.info(f"{'='*50}")
        
        return success_count == total_count
    
    def clean_build(self) -> bool:
        """Clean build artifacts"""
        logger.info("🧹 Cleaning build artifacts...")
        
        try:
            # Clean cargo build cache
            cmd = ["cargo", "clean"]
            result = subprocess.run(cmd, capture_output=True, text=True)
            if result.returncode != 0:
                logger.warning(f"⚠️ Cargo clean failed: {result.stderr}")
                return False
            
            # Remove target directory
            if self.target_dir.exists():
                shutil.rmtree(self.target_dir)
                logger.info("✅ Target directory cleaned")
            
            logger.info("✅ Build artifacts cleaned successfully")
            return True
            
        except Exception as e:
            logger.error(f"❌ Error cleaning build artifacts: {e}")
            return False
    
    def verify_builds(self) -> bool:
        """Verify all built executables exist and are executable"""
        logger.info("🔍 Verifying built executables...")
        
        all_valid = True
        for service_name, service_config in self.services.items():
            binary_name = service_config["binary"]
            binary_path = self.target_dir / binary_name
            
            if not binary_path.exists():
                logger.error(f"❌ {binary_name} not found")
                all_valid = False
            else:
                # Check if file is executable (Windows)
                if binary_path.suffix == '.exe':
                    logger.info(f"✅ {binary_name} found and ready")
                else:
                    logger.warning(f"⚠️ {binary_name} found but may not be executable")
        
        return all_valid

def main():
    """Main function"""
    print("🚀 Chaos World Services Build Script")
    print("=" * 50)
    
    # Get workspace root
    script_dir = Path(__file__).parent
    workspace_root = script_dir.parent
    
    if not workspace_root.exists():
        logger.error(f"❌ Workspace root not found: {workspace_root}")
        sys.exit(1)
    
    logger.info(f"📁 Workspace root: {workspace_root}")
    
    # Create builder
    builder = ServiceBuilder(workspace_root)
    
    # Parse command line arguments
    if len(sys.argv) > 1:
        command = sys.argv[1].lower()
        
        if command == "clean":
            if builder.clean_build():
                logger.info("✅ Clean completed successfully")
                sys.exit(0)
            else:
                logger.error("❌ Clean failed")
                sys.exit(1)
        
        elif command == "verify":
            if builder.verify_builds():
                logger.info("✅ All builds verified successfully")
                sys.exit(0)
            else:
                logger.error("❌ Build verification failed")
                sys.exit(1)
        
        elif command == "help":
            print("Usage: python build_services.py [command]")
            print("Commands:")
            print("  (no command) - Build all services")
            print("  clean        - Clean build artifacts")
            print("  verify       - Verify built executables")
            print("  help         - Show this help message")
            sys.exit(0)
        
        else:
            logger.error(f"❌ Unknown command: {command}")
            logger.info("Use 'python build_services.py help' for usage information")
            sys.exit(1)
    
    # Build all services
    if builder.build_all_services():
        logger.info("🎉 All services built successfully!")
        
        # Verify builds
        if builder.verify_builds():
            logger.info("✅ Build verification passed")
            sys.exit(0)
        else:
            logger.error("❌ Build verification failed")
            sys.exit(1)
    else:
        logger.error("❌ Build process failed")
        sys.exit(1)

if __name__ == "__main__":
    main()
