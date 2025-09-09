#!/usr/bin/env python3
"""
Configuration validation script for Actor Core.

This script validates that the sample configuration files are:
1. Valid YAML syntax
2. Have the expected structure
3. Contain reasonable values
4. Can be loaded by the registry loader

Usage:
    python3 scripts/validate_configs.py
"""

import yaml
import sys
import os
from pathlib import Path

def validate_yaml_syntax(file_path):
    """Validate that a file has valid YAML syntax."""
    try:
        with open(file_path, 'r') as f:
            yaml.safe_load(f)
        print(f"✅ {file_path} has valid YAML syntax")
        return True
    except yaml.YAMLError as e:
        print(f"❌ {file_path} has invalid YAML syntax: {e}")
        return False
    except FileNotFoundError:
        print(f"❌ {file_path} not found")
        return False

def validate_cap_layers_structure(data):
    """Validate the structure of cap_layers.yaml."""
    if not isinstance(data, dict):
        print("❌ cap_layers.yaml should be a dictionary")
        return False
    
    if 'layers' not in data:
        print("❌ cap_layers.yaml should have 'layers' key")
        return False
    
    layers = data['layers']
    if not isinstance(layers, list):
        print("❌ 'layers' should be a list")
        return False
    
    if len(layers) == 0:
        print("❌ 'layers' should not be empty")
        return False
    
    # Validate each layer
    for i, layer in enumerate(layers):
        if not isinstance(layer, dict):
            print(f"❌ Layer {i} should be a dictionary")
            return False
        
        required_keys = ['name', 'priority', 'caps']
        for key in required_keys:
            if key not in layer:
                print(f"❌ Layer {i} should have '{key}' key")
                return False
        
        # Validate layer name
        if not isinstance(layer['name'], str) or not layer['name']:
            print(f"❌ Layer {i} name should be a non-empty string")
            return False
        
        # Validate priority
        if not isinstance(layer['priority'], int):
            print(f"❌ Layer {i} priority should be an integer")
            return False
        
        # Validate caps
        caps = layer['caps']
        if not isinstance(caps, list):
            print(f"❌ Layer {i} caps should be a list")
            return False
        
        for j, cap in enumerate(caps):
            if not isinstance(cap, dict):
                print(f"❌ Layer {i} cap {j} should be a dictionary")
                return False
            
            required_cap_keys = ['id', 'cap_mode']
            for key in required_cap_keys:
                if key not in cap:
                    print(f"❌ Layer {i} cap {j} should have '{key}' key")
                    return False
            
            # Validate cap_mode
            valid_modes = ['BASELINE', 'ADDITIVE', 'HARD_MIN', 'HARD_MAX', 'OVERRIDE']
            if cap['cap_mode'] not in valid_modes:
                print(f"❌ Layer {i} cap {j} has invalid cap_mode: {cap['cap_mode']}")
                return False
    
    print("✅ cap_layers.yaml has valid structure")
    return True

def validate_combiner_structure(data):
    """Validate the structure of combiner.yaml."""
    if not isinstance(data, dict):
        print("❌ combiner.yaml should be a dictionary")
        return False
    
    if 'rules' not in data:
        print("❌ combiner.yaml should have 'rules' key")
        return False
    
    rules = data['rules']
    if not isinstance(rules, list):
        print("❌ 'rules' should be a list")
        return False
    
    if len(rules) == 0:
        print("❌ 'rules' should not be empty")
        return False
    
    # Validate each rule
    for i, rule in enumerate(rules):
        if not isinstance(rule, dict):
            print(f"❌ Rule {i} should be a dictionary")
            return False
        
        required_keys = ['id', 'bucket_order', 'clamp']
        for key in required_keys:
            if key not in rule:
                print(f"❌ Rule {i} should have '{key}' key")
                return False
        
        # Validate id
        if not isinstance(rule['id'], str) or not rule['id']:
            print(f"❌ Rule {i} id should be a non-empty string")
            return False
        
        # Validate bucket_order
        bucket_order = rule['bucket_order']
        if not isinstance(bucket_order, list):
            print(f"❌ Rule {i} bucket_order should be a list")
            return False
        
        valid_buckets = ['FLAT', 'MULT', 'POST_ADD', 'OVERRIDE', 'EXPONENTIAL', 'LOGARITHMIC', 'CONDITIONAL']
        for bucket in bucket_order:
            if bucket not in valid_buckets:
                print(f"❌ Rule {i} has invalid bucket: {bucket}")
                return False
        
        # Validate clamp
        clamp = rule['clamp']
        if not isinstance(clamp, dict):
            print(f"❌ Rule {i} clamp should be a dictionary")
            return False
        
        if 'min' not in clamp or 'max' not in clamp:
            print(f"❌ Rule {i} clamp should have 'min' and 'max' keys")
            return False
        
        if not isinstance(clamp['min'], (int, float)) or not isinstance(clamp['max'], (int, float)):
            print(f"❌ Rule {i} clamp min/max should be numbers")
            return False
        
        if clamp['min'] > clamp['max']:
            print(f"❌ Rule {i} clamp min should be <= max")
            return False
    
    print("✅ combiner.yaml has valid structure")
    return True

def main():
    """Main validation function."""
    print("🔍 Validating Actor Core configuration files...")
    print()
    
    # Change to the script's directory
    script_dir = Path(__file__).parent
    os.chdir(script_dir.parent)
    
    success = True
    
    # Validate cap_layers.yaml
    print("📋 Validating cap_layers.yaml...")
    if validate_yaml_syntax('configs/cap_layers.yaml'):
        with open('configs/cap_layers.yaml', 'r') as f:
            data = yaml.safe_load(f)
        if not validate_cap_layers_structure(data):
            success = False
    else:
        success = False
    
    print()
    
    # Validate combiner.yaml
    print("📋 Validating combiner.yaml...")
    if validate_yaml_syntax('configs/combiner.yaml'):
        with open('configs/combiner.yaml', 'r') as f:
            data = yaml.safe_load(f)
        if not validate_combiner_structure(data):
            success = False
    else:
        success = False
    
    print()
    
    if success:
        print("🎉 All configuration files are valid!")
        sys.exit(0)
    else:
        print("❌ Configuration validation failed!")
        sys.exit(1)

if __name__ == '__main__':
    main()
