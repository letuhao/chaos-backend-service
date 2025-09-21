#!/usr/bin/env python3
"""
Simple Deploy Script
A simple deployment script that uses service_utils for stop/build/copy/start operations
"""

import sys
import time
from pathlib import Path
from typing import List

# Add the scripts directory to the path so we can import service_utils
sys.path.insert(0, str(Path(__file__).parent))

from service_utils import ServiceUtils

class SimpleDeployer:
    def __init__(self):
        self.utils = ServiceUtils()
    
    def log(self, message: str, level: str = "INFO"):
        """Log a message with timestamp"""
        import time
        timestamp = time.strftime("%H:%M:%S")
        print(f"[{timestamp}] {level}: {message}")
    
    def deploy_all(self, skip_build: bool = False, service_ids: List[str] = None) -> bool:
        """Deploy all services or specific services"""
        if service_ids:
            self.log(f"🚀 Starting Simple Deployment for: {', '.join(service_ids)}")
        else:
            self.log("🚀 Starting Simple Deployment for all services")
        print("=" * 50)
        
        start_time = time.time()
        
        # Step 1: Stop services
        self.log("🛑 Step 1: Stopping services...")
        if not self.utils.stop_all_services(service_ids):
            self.log("❌ Failed to stop services. Aborting deployment.", "ERROR")
            return False
        
        # Wait for services to fully stop
        self.log("⏳ Waiting for services to fully stop...")
        time.sleep(3)
        
        # Step 2: Build services (optional)
        if not skip_build:
            self.log("🔨 Step 2: Building services...")
            if not self.utils.build_services(service_ids):
                self.log("❌ Failed to build services. Aborting deployment.", "ERROR")
                return False
        else:
            self.log("⏭️  Skipping build step (--skip-build specified)")
        
        # Step 3: Copy config files
        self.log("📋 Step 3: Copying configuration files...")
        if not self.utils.copy_config_files(service_ids):
            self.log("❌ Failed to copy config files. Aborting deployment.", "ERROR")
            return False
        
        # Step 4: Copy services
        self.log("📦 Step 4: Copying services...")
        if not self.utils.copy_services(service_ids):
            self.log("❌ Failed to copy services. Aborting deployment.", "ERROR")
            return False
        
        # Step 5: Start services
        self.log("🚀 Step 5: Starting services...")
        if not self.utils.start_all_services(service_ids):
            self.log("❌ Failed to start services. Aborting deployment.", "ERROR")
            return False
        
        # Step 6: Wait for services to become healthy
        if not self.utils.wait_for_services(service_ids, timeout=30):
            self.log("⚠️  Some services may not be fully healthy yet", "WARNING")
        
        # Calculate deployment time
        end_time = time.time()
        deployment_time = end_time - start_time
        
        self.log("✅ Deployment completed!")
        self.log(f"⏱️  Total deployment time: {deployment_time:.2f} seconds")
        
        # Show final status for deployed services only
        self.log("📊 Final service status:")
        status = self.utils.get_service_status()
        
        # If no specific services were deployed, show all services
        if service_ids is None:
            service_ids = list(self.utils.services.keys())
        
        for service_id in service_ids:
            if service_id in status:
                is_healthy = status[service_id]
                config = self.utils.services[service_id]
                status_icon = "✅" if is_healthy else "❌"
                self.log(f"  {status_icon} {config['name']} (port {config['port']})")
            else:
                config = self.utils.services[service_id]
                self.log(f"  ❓ {config['name']} (port {config['port']}) - Status unknown")
        
        return True
    
    def quick_deploy(self) -> bool:
        """Quick deploy - skip build step"""
        return self.deploy_all(skip_build=True)
    
    def full_deploy(self) -> bool:
        """Full deploy - including build step"""
        return self.deploy_all(skip_build=False)
    
    def stop_all(self, service_ids: List[str] = None) -> bool:
        """Stop all services or specific services"""
        if service_ids:
            self.log(f"🛑 Stopping services: {', '.join(service_ids)}...")
        else:
            self.log("🛑 Stopping all services...")
        return self.utils.stop_all_services(service_ids)
    
    def start_all(self, service_ids: List[str] = None) -> bool:
        """Start all services or specific services"""
        if service_ids:
            self.log(f"🚀 Starting services: {', '.join(service_ids)}...")
        else:
            self.log("🚀 Starting all services...")
        if not self.utils.start_all_services(service_ids):
            return False
        
        # Wait for services to become healthy
        return self.utils.wait_for_services(service_ids, timeout=30)
    
    def build_all(self, service_ids: List[str] = None) -> bool:
        """Build all services or specific services"""
        if service_ids:
            self.log(f"🔨 Building services: {', '.join(service_ids)}...")
        else:
            self.log("🔨 Building all services...")
        return self.utils.build_services(service_ids)
    
    def copy_all(self, service_ids: List[str] = None) -> bool:
        """Copy all services or specific services"""
        if service_ids:
            self.log(f"📦 Copying services: {', '.join(service_ids)}...")
        else:
            self.log("📦 Copying all services...")
        return self.utils.copy_services(service_ids)
    
    def status(self) -> bool:
        """Show service status"""
        self.log("📊 Service Status:")
        print("=" * 30)
        
        status = self.utils.get_service_status()
        for service_id, is_healthy in status.items():
            config = self.utils.services[service_id]
            status_icon = "✅" if is_healthy else "❌"
            self.log(f"  {status_icon} {config['name']} (port {config['port']})")
        
        return True

    def restart_all(self, service_ids: List[str] = None) -> bool:
        """Restart all services or specific services"""
        if service_ids:
            self.log(f"🔄 Restarting services: {', '.join(service_ids)}")
        else:
            self.log("🔄 Restarting all services...")
        
        # Stop services
        if not self.utils.stop_all_services():
            self.log("❌ Failed to stop services", "ERROR")
            return False
        
        # Wait for services to fully stop
        self.log("⏳ Waiting for services to fully stop...")
        time.sleep(3)
        
        # Start services
        if not self.utils.start_all_services(service_ids):
            self.log("❌ Failed to start services", "ERROR")
            return False
        
        # Wait for services to become healthy
        if not self.utils.wait_for_services(service_ids, timeout=30):
            self.log("⚠️  Some services may not be fully healthy yet", "WARNING")
        
        return True

    def test_service(self, service_id: str) -> bool:
        """Test deploy a single service with full process"""
        if service_id not in self.utils.services:
            self.log(f"❌ Unknown service: {service_id}", "ERROR")
            return False
        
        config = self.utils.services[service_id]
        self.log(f"🧪 Testing deployment of {config['name']}...")
        print("=" * 50)
        
        # Step 1: Stop the specific service
        self.log(f"🛑 Step 1: Stopping {config['name']}...")
        process = self.utils.find_process_by_port(config["port"])
        if process:
            if not self.utils.stop_service_by_port(config["port"], config['name']):
                self.log(f"❌ Failed to stop {config['name']}", "ERROR")
                return False
        else:
            self.log(f"ℹ️  {config['name']} was not running")
        
        # Step 2: Build the service
        self.log(f"🔨 Step 2: Building {config['name']}...")
        if not self.utils.build_services([service_id]):
            self.log(f"❌ Failed to build {config['name']}", "ERROR")
            return False
        
        # Step 3: Copy config files for the specific service
        self.log(f"📋 Step 3: Copying configuration files for {config['name']}...")
        if not self.utils.copy_config_files([service_id]):
            self.log(f"❌ Failed to copy config files", "ERROR")
            return False
        
        # Step 4: Copy the service
        self.log(f"📦 Step 4: Copying {config['name']}...")
        if not self.utils.copy_services([service_id]):
            self.log(f"❌ Failed to copy {config['name']}", "ERROR")
            return False
        
        # Step 5: Start the service
        self.log(f"🚀 Step 5: Starting {config['name']}...")
        if not self.utils.start_service(service_id):
            self.log(f"❌ Failed to start {config['name']}", "ERROR")
            return False
        
        # Step 6: Wait for service to become healthy
        self.log(f"⏳ Step 6: Waiting for {config['name']} to become healthy...")
        # Wait a bit for the service to start
        time.sleep(3)
        # Check if the specific service is healthy
        is_healthy = self.utils.check_service_health(service_id)
        if not is_healthy:
            self.log(f"⚠️  {config['name']} may not be fully healthy yet", "WARNING")
        
        # Step 7: Check final status
        self.log(f"📊 Step 7: Checking final status...")
        is_healthy = self.utils.check_service_health(service_id)
        status_icon = "✅" if is_healthy else "❌"
        self.log(f"  {status_icon} {config['name']} (port {config['port']}) - {'Healthy' if is_healthy else 'Unhealthy'}")
        
        return is_healthy

def main():
    """Main function"""
    deployer = SimpleDeployer()
    
    if len(sys.argv) > 1:
        command = sys.argv[1].lower()
        
        if command == "help":
            print("🚀 Simple Deploy Script")
            print("=" * 30)
            print("Usage:")
            print("  python simple_deploy.py [command] [services...]")
            print()
            print("Commands:")
            print("  help           - Show this help message")
            print("  stop           - Stop all services")
            print("  start          - Start all services")
            print("  build          - Build all services")
            print("  copy           - Copy all services")
            print("  status         - Show service status")
            print("  restart        - Restart all services")
            print("  quick          - Quick deploy (skip build)")
            print("  full           - Full deploy (with build)")
            print("  deploy         - Same as 'full' (default)")
            print("  test           - Test individual service deployment")
            print()
            print("Available Services:")
            for service_id, config in deployer.utils.services.items():
                print(f"  {service_id:<20} - {config['name']} (port {config['port']})")
            print()
            print("Examples:")
            print("  python simple_deploy.py quick                    # Quick deploy all services")
            print("  python simple_deploy.py full                     # Full deploy all services")
            print("  python simple_deploy.py quick api-gateway        # Deploy only API Gateway")
            print("  python simple_deploy.py full api-gateway user-management  # Deploy specific services")
            print("  python simple_deploy.py restart api-gateway      # Restart only API Gateway")
            print("  python simple_deploy.py test user-management     # Test deploy User Management")
            print("  python simple_deploy.py status                   # Check service status")
            return
        
        # Parse service arguments
        service_ids = None
        if len(sys.argv) > 2:
            service_ids = sys.argv[2:]
            # Validate service IDs
            valid_services = set(deployer.utils.services.keys())
            invalid_services = set(service_ids) - valid_services
            if invalid_services:
                print(f"❌ Invalid services: {', '.join(invalid_services)}")
                print(f"Valid services: {', '.join(valid_services)}")
                sys.exit(1)
        
        if command == "stop":
            success = deployer.stop_all(service_ids)
        elif command == "start":
            success = deployer.start_all(service_ids)
        elif command == "build":
            success = deployer.build_all(service_ids)
        elif command == "copy":
            success = deployer.copy_all(service_ids)
        elif command == "status":
            success = deployer.status()
        elif command == "restart":
            success = deployer.restart_all(service_ids)
        elif command == "test":
            if not service_ids or len(service_ids) != 1:
                print("❌ Test command requires exactly one service")
                print("Usage: python simple_deploy.py test <service-name>")
                sys.exit(1)
            success = deployer.test_service(service_ids[0])
        elif command == "quick":
            success = deployer.deploy_all(skip_build=True, service_ids=service_ids)
        elif command == "full" or command == "deploy":
            success = deployer.deploy_all(skip_build=False, service_ids=service_ids)
        else:
            print(f"❌ Unknown command: {command}")
            print("Use 'python simple_deploy.py help' for usage information")
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
