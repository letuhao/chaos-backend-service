#!/usr/bin/env python3
"""
Environment Check Script
Verifies that all required dependencies are available.
"""

import sys
import subprocess

def check_python_version():
    """Check Python version"""
    print(f"Python version: {sys.version}")
    print(f"Python executable: {sys.executable}")
    print()

def check_dependencies():
    """Check if required dependencies are installed"""
    dependencies = ['requests', 'yaml']
    
    for dep in dependencies:
        try:
            if dep == 'yaml':
                import yaml
                print(f"✅ {dep} (PyYAML) is available")
            else:
                __import__(dep)
                print(f"✅ {dep} is available")
        except ImportError as e:
            print(f"❌ {dep} is not available: {e}")
            print(f"   Install with: pip install {dep if dep != 'yaml' else 'PyYAML'}")

def check_pip_packages():
    """Check installed packages via pip"""
    try:
        result = subprocess.run([sys.executable, '-m', 'pip', 'list'], 
                              capture_output=True, text=True)
        if result.returncode == 0:
            print("\nInstalled packages:")
            for line in result.stdout.split('\n'):
                if 'requests' in line.lower() or 'yaml' in line.lower():
                    print(f"  {line}")
        else:
            print("❌ Could not list pip packages")
    except Exception as e:
        print(f"❌ Error checking pip packages: {e}")

if __name__ == "__main__":
    print("🔍 Checking Python Environment...")
    print("=" * 40)
    
    check_python_version()
    check_dependencies()
    check_pip_packages()
    
    print("\n" + "=" * 40)
    print("Environment check complete!")
