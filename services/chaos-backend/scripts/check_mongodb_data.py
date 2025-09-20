#!/usr/bin/env python3
"""
Script to check MongoDB data structure and organization
"""

import pymongo
from pprint import pprint
import json

def check_mongodb_data():
    """Check MongoDB data structure and organization"""
    
    # Connect to MongoDB
    client = pymongo.MongoClient("mongodb://localhost:27017")
    db = client["chaos_game"]
    
    print("=" * 60)
    print("🔍 MONGODB DATA STRUCTURE ANALYSIS")
    print("=" * 60)
    
    # List all collections
    print("\n📋 COLLECTIONS IN DATABASE:")
    collections = db.list_collection_names()
    for collection_name in collections:
        count = db[collection_name].count_documents({})
        print(f"  - {collection_name}: {count} documents")
    
    print("\n" + "=" * 60)
    
    # Check runtime_flags collection
    print("\n🚩 RUNTIME FLAGS COLLECTION:")
    runtime_flags = db["runtime_flags"]
    flags_docs = list(runtime_flags.find())
    
    for doc in flags_docs:
        print(f"\n📄 Document ID: {doc.get('_id')}")
        for key, value in doc.items():
            if key != '_id':
                print(f"  {key}: {value} ({type(value).__name__})")
    
    print("\n" + "=" * 60)
    
    # Check configurations collection (if exists)
    config_collections = [name for name in collections if 'config' in name.lower()]
    
    if config_collections:
        print(f"\n⚙️  CONFIGURATION COLLECTIONS:")
        for collection_name in config_collections:
            print(f"\n📁 Collection: {collection_name}")
            collection = db[collection_name]
            
            # Get sample documents
            sample_docs = list(collection.find().limit(5))
            
            if sample_docs:
                print(f"  Total documents: {collection.count_documents({})}")
                print(f"  Sample documents:")
                
                for i, doc in enumerate(sample_docs):
                    print(f"\n  📄 Document {i+1}:")
                    print(f"    ID: {doc.get('_id')}")
                    
                    # Show key fields
                    for key, value in doc.items():
                        if key != '_id':
                            if isinstance(value, dict):
                                print(f"    {key}: {type(value).__name__} with {len(value)} keys")
                            elif isinstance(value, list):
                                print(f"    {key}: {type(value).__name__} with {len(value)} items")
                            else:
                                print(f"    {key}: {value} ({type(value).__name__})")
            else:
                print("  No documents found")
    else:
        print("\n⚠️  No configuration collections found")
    
    print("\n" + "=" * 60)
    
    # Check for any actor-related collections
    actor_collections = [name for name in collections if 'actor' in name.lower()]
    
    if actor_collections:
        print(f"\n👤 ACTOR COLLECTIONS:")
        for collection_name in actor_collections:
            print(f"\n📁 Collection: {collection_name}")
            collection = db[collection_name]
            count = collection.count_documents({})
            print(f"  Total documents: {count}")
            
            if count > 0:
                sample = collection.find_one()
                print(f"  Sample document structure:")
                for key, value in sample.items():
                    if key != '_id':
                        print(f"    {key}: {type(value).__name__}")
    
    print("\n" + "=" * 60)
    
    # Check for any resource-related collections
    resource_collections = [name for name in collections if 'resource' in name.lower()]
    
    if resource_collections:
        print(f"\n💎 RESOURCE COLLECTIONS:")
        for collection_name in resource_collections:
            print(f"\n📁 Collection: {collection_name}")
            collection = db[collection_name]
            count = collection.count_documents({})
            print(f"  Total documents: {count}")
            
            if count > 0:
                sample = collection.find_one()
                print(f"  Sample document structure:")
                for key, value in sample.items():
                    if key != '_id':
                        print(f"    {key}: {type(value).__name__}")
    
    print("\n" + "=" * 60)
    
    # Summary
    print("\n📊 SUMMARY:")
    print(f"  Total collections: {len(collections)}")
    print(f"  Runtime flags: {len(flags_docs)} documents")
    
    total_config_docs = 0
    for collection_name in config_collections:
        total_config_docs += db[collection_name].count_documents({})
    
    print(f"  Configuration documents: {total_config_docs}")
    
    print("\n✅ MongoDB data structure analysis completed!")
    
    client.close()

if __name__ == "__main__":
    check_mongodb_data()
