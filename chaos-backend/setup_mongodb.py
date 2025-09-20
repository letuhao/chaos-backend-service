#!/usr/bin/env python3
"""
MongoDB Setup Script for Chaos Backend Service
This script sets up the MongoDB database with runtime flags and configuration.
"""

import pymongo
import json
from datetime import datetime
import sys

def setup_mongodb():
    """Setup MongoDB with runtime flags and configuration"""
    
    # MongoDB connection
    client = pymongo.MongoClient("mongodb://localhost:27017")
    db = client["chaos_game"]
    
    print("🔗 Connected to MongoDB")
    
    # Create collections
    collections = [
        "runtime_flags",
        "configurations", 
        "actors",
        "game_events",
        "performance_metrics"
    ]
    
    for collection_name in collections:
        if collection_name not in db.list_collection_names():
            db.create_collection(collection_name)
            print(f"📁 Created collection: {collection_name}")
        else:
            print(f"✅ Collection exists: {collection_name}")
    
    # Setup runtime flags
    runtime_flags = {
        "_id": "runtime_config",
        "server_port": 8080,
        "max_connections": 1000,
        "tick_rate": 60,
        "enable_mongodb_sync": True,
        "mongodb_connection": "mongodb://localhost:27017",
        "config_sync_interval": 300,
        "log_level": "info",
        "enable_metrics": True,
        "enable_health_checks": True,
        "world_size": 10000,
        "max_actors": 10000,
        "created_at": datetime.utcnow(),
        "updated_at": datetime.utcnow()
    }
    
    # Insert or update runtime flags
    db.runtime_flags.replace_one(
        {"_id": "runtime_config"}, 
        runtime_flags, 
        upsert=True
    )
    print("🚩 Runtime flags configured")
    
    # Setup default configurations
    default_configs = [
        {
            "_id": "defaults.default_actor_health",
            "category": "defaults",
            "key": "default_actor_health",
            "value": 100,
            "value_type": "integer",
            "source_provider": "mongodb",
            "priority": 1,
            "created_at": datetime.utcnow()
        },
        {
            "_id": "defaults.default_actor_level",
            "category": "defaults", 
            "key": "default_actor_level",
            "value": 1,
            "value_type": "integer",
            "source_provider": "mongodb",
            "priority": 1,
            "created_at": datetime.utcnow()
        },
        {
            "_id": "defaults.default_actor_experience",
            "category": "defaults",
            "key": "default_actor_experience", 
            "value": 0,
            "value_type": "integer",
            "source_provider": "mongodb",
            "priority": 1,
            "created_at": datetime.utcnow()
        },
        {
            "_id": "logging.level",
            "category": "logging",
            "key": "level",
            "value": "info",
            "value_type": "string",
            "source_provider": "mongodb",
            "priority": 1,
            "created_at": datetime.utcnow()
        },
        {
            "_id": "metrics.enabled",
            "category": "metrics",
            "key": "enabled",
            "value": True,
            "value_type": "boolean",
            "source_provider": "mongodb",
            "priority": 1,
            "created_at": datetime.utcnow()
        }
    ]
    
    for config in default_configs:
        db.configurations.replace_one(
            {"_id": config["_id"]},
            config,
            upsert=True
        )
    
    print("⚙️  Default configurations loaded")
    
    # Create indexes
    db.runtime_flags.create_index("_id")
    db.configurations.create_index("category")
    db.configurations.create_index("key")
    db.actors.create_index("id")
    db.actors.create_index("race")
    db.actors.create_index("level")
    db.game_events.create_index("timestamp")
    db.performance_metrics.create_index("timestamp")
    
    print("📊 Database indexes created")
    
    print("\n✅ MongoDB setup completed successfully!")
    print("\n📋 Available runtime flags:")
    for key, value in runtime_flags.items():
        if key not in ["_id", "created_at", "updated_at"]:
            print(f"   {key}: {value}")
    
    print("\n🚀 You can now run the server with:")
    print("   cargo run --features mongodb-storage")

if __name__ == "__main__":
    try:
        setup_mongodb()
    except Exception as e:
        print(f"❌ Error setting up MongoDB: {e}")
        sys.exit(1)
