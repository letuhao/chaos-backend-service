#!/usr/bin/env python3
"""
Simple MongoDB data check using mongo shell commands
"""

import subprocess
import json
import sys

def run_mongo_command(command):
    """Run a MongoDB command and return the result"""
    try:
        result = subprocess.run(
            ["mongosh", "mongodb://localhost:27017/chaos_game", "--eval", command, "--quiet"],
            capture_output=True,
            text=True,
            check=True
        )
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error running MongoDB command: {e}")
        print(f"Error output: {e.stderr}")
        return None
    except FileNotFoundError:
        print("mongosh not found. Please install MongoDB Shell or use MongoDB Compass")
        return None

def check_mongodb_data():
    """Check MongoDB data structure"""
    
    print("=" * 60)
    print("🔍 MONGODB DATA STRUCTURE ANALYSIS")
    print("=" * 60)
    
    # List all collections
    print("\n📋 COLLECTIONS IN DATABASE:")
    collections_result = run_mongo_command("db.runCommand('listCollections').cursor.firstBatch.map(c => c.name)")
    
    if collections_result:
        try:
            collections = json.loads(collections_result)
            for collection in collections:
                count_result = run_mongo_command(f"db.{collection}.countDocuments()")
                if count_result:
                    print(f"  - {collection}: {count_result} documents")
        except json.JSONDecodeError:
            print("  Error parsing collections list")
    else:
        print("  Error getting collections list")
    
    print("\n" + "=" * 60)
    
    # Check runtime_flags collection
    print("\n🚩 RUNTIME FLAGS COLLECTION:")
    flags_result = run_mongo_command("db.runtime_flags.find().toArray()")
    
    if flags_result:
        try:
            flags_docs = json.loads(flags_result)
            for doc in flags_docs:
                print(f"\n📄 Document ID: {doc.get('_id')}")
                for key, value in doc.items():
                    if key != '_id':
                        print(f"  {key}: {value} ({type(value).__name__})")
        except json.JSONDecodeError:
            print("  Error parsing runtime flags")
    else:
        print("  Error getting runtime flags")
    
    print("\n" + "=" * 60)
    
    # Check configurations collection
    print("\n⚙️  CONFIGURATION COLLECTIONS:")
    
    # Look for any collection with 'config' in the name
    config_collections = []
    if collections_result:
        try:
            collections = json.loads(collections_result)
            config_collections = [c for c in collections if 'config' in c.lower()]
        except json.JSONDecodeError:
            pass
    
    if config_collections:
        for collection_name in config_collections:
            print(f"\n📁 Collection: {collection_name}")
            count_result = run_mongo_command(f"db.{collection_name}.countDocuments()")
            if count_result:
                print(f"  Total documents: {count_result}")
                
                # Get sample documents
                sample_result = run_mongo_command(f"db.{collection_name}.find().limit(3).toArray()")
                if sample_result:
                    try:
                        sample_docs = json.loads(sample_result)
                        print(f"  Sample documents:")
                        
                        for i, doc in enumerate(sample_docs):
                            print(f"\n  📄 Document {i+1}:")
                            print(f"    ID: {doc.get('_id')}")
                            
                            for key, value in doc.items():
                                if key != '_id':
                                    if isinstance(value, dict):
                                        print(f"    {key}: Object with {len(value)} keys")
                                    elif isinstance(value, list):
                                        print(f"    {key}: Array with {len(value)} items")
                                    else:
                                        print(f"    {key}: {value} ({type(value).__name__})")
                    except json.JSONDecodeError:
                        print("  Error parsing sample documents")
    else:
        print("  No configuration collections found")
    
    print("\n" + "=" * 60)
    
    # Summary
    print("\n📊 SUMMARY:")
    if collections_result:
        try:
            collections = json.loads(collections_result)
            print(f"  Total collections: {len(collections)}")
        except json.JSONDecodeError:
            print("  Error getting collections count")
    
    print("\n✅ MongoDB data structure analysis completed!")

if __name__ == "__main__":
    check_mongodb_data()
