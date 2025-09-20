#!/usr/bin/env python3
"""
Clean up actor-core examples by removing outdated ones and fixing working ones
"""

import os
import subprocess
import sys
from pathlib import Path

def run_command(command, check=True):
    """Run a command and return the result"""
    try:
        result = subprocess.run(command, shell=True, capture_output=True, text=True, check=check)
        return result.returncode == 0, result.stdout, result.stderr
    except subprocess.CalledProcessError as e:
        return False, e.stdout, e.stderr

def check_example(example_name):
    """Check if an example compiles"""
    print(f"Checking {example_name}...")
    success, stdout, stderr = run_command(f'cargo check --example {example_name} -p actor-core', check=False)
    return success, stderr

def main():
    examples_dir = Path("crates/actor-core/examples")
    
    # Examples that are clearly outdated and should be removed
    outdated_examples = [
        "deprecation_example.rs",
        "exhaustion_cli.rs", 
        "exhaustion_system_example.rs",
        "performance_workflow_example.rs",
        "detailed_performance_analysis.rs",
        "correct_performance_comparison.rs",
        "legacy_subsystems/subsystem_registration.rs",
        "legacy_subsystems/system_resource_manager.rs", 
        "legacy_subsystems/validator.rs",
        "legacy_resource_managers/enhanced_hybrid_resource_manager.rs",
        "legacy_resource_managers/magic_resource_manager.rs",
        "legacy_resource_managers/resource_manager.rs",
        "legacy_resource_managers/rpg_resource_manager.rs",
        "legacy_resource_managers/README.md",
        "rpg_resource_manager_refactored.rs",
        "validation_example.rs",
        "observability_example.rs",
        "observability_integration.rs",
        "redis_connection_example.rs",
        "resource_manager_example.rs",
        "subsystem_example.rs",
        "runtime_registry_example.rs",
        "simple_condition_integration.rs",
        "complete_refactor_example.rs",
        "builder_pattern_example.rs",
        "config_usage_example.rs",
        "configuration_hub_example.rs",
        "mongodb_config_demo.rs",
        "mongodb_simple_test.rs",
        "production_condition_configs.yaml",
        "magic_resource_config.yaml",
        "rpg_resource_config.yaml"
    ]
    
    print("🗑️  Removing outdated examples...")
    for example in outdated_examples:
        example_path = examples_dir / example
        if example_path.exists():
            print(f"   Removing {example}")
            if example_path.is_file():
                example_path.unlink()
            elif example_path.is_dir():
                import shutil
                shutil.rmtree(example_path)
        else:
            print(f"   {example} not found, skipping")
    
    print("\n✅ Cleanup completed!")
    print("\nRemaining examples:")
    
    # List remaining examples
    remaining_examples = []
    for item in examples_dir.rglob("*.rs"):
        if item.is_file():
            relative_path = item.relative_to(examples_dir)
            remaining_examples.append(str(relative_path).replace("\\", "/"))
    
    for example in sorted(remaining_examples):
        print(f"   {example}")
    
    print(f"\nTotal remaining examples: {len(remaining_examples)}")

if __name__ == "__main__":
    main()
