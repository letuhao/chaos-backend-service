#!/usr/bin/env python3
"""
Update Runtime Flags Script
This script allows you to update runtime flags in MongoDB without restarting the server.
"""

import pymongo
import json
import sys
from datetime import datetime

def update_runtime_flags():
    """Update runtime flags in MongoDB"""
    
    if len(sys.argv) < 3:
        print("Usage: python update_flags.py <flag_name> <value>")
        print("Example: python update_flags.py server_port 9090")
        print("Example: python update_flags.py max_connections 2000")
        print("Example: python update_flags.py enable_mongodb_sync false")
        sys.exit(1)
    
    flag_name = sys.argv[1]
    flag_value = sys.argv[2]
    
    # Parse value type
    if flag_value.lower() in ['true', 'false']:
        flag_value = flag_value.lower() == 'true'
    elif flag_value.isdigit():
        flag_value = int(flag_value)
    elif flag_value.replace('.', '').isdigit():
        flag_value = float(flag_value)
    
    # MongoDB connection
    client = pymongo.MongoClient("mongodb://localhost:27017")
    db = client["chaos_game"]
    
    print(f"🔗 Connected to MongoDB")
    print(f"🔄 Updating flag: {flag_name} = {flag_value}")
    
    # Update the flag
    result = db.runtime_flags.update_one(
        {"_id": "runtime_config"},
        {
            "$set": {
                flag_name: flag_value,
                "updated_at": datetime.utcnow()
            }
        }
    )
    
    if result.modified_count > 0:
        print(f"✅ Successfully updated {flag_name} to {flag_value}")
        print("🔄 Server will pick up the change on next config sync")
    else:
        print(f"❌ Failed to update {flag_name}")
        sys.exit(1)

def list_flags():
    """List all current runtime flags"""
    
    client = pymongo.MongoClient("mongodb://localhost:27017")
    db = client["chaos_game"]
    
    print("🔗 Connected to MongoDB")
    print("📋 Current runtime flags:")
    
    flags = db.runtime_flags.find_one({"_id": "runtime_config"})
    if flags:
        for key, value in flags.items():
            if key not in ["_id", "created_at", "updated_at"]:
                print(f"   {key}: {value}")
    else:
        print("❌ No runtime flags found")

if __name__ == "__main__":
    if len(sys.argv) == 1:
        list_flags()
    else:
        update_runtime_flags()
