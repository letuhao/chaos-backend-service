// MongoDB data structure analysis script
const { MongoClient } = require('mongodb');

async function checkMongoDBData() {
    const client = new MongoClient('mongodb://localhost:27017');
    
    try {
        await client.connect();
        const db = client.db('chaos_game');
        
        console.log('='.repeat(60));
        console.log('🔍 MONGODB DATA STRUCTURE ANALYSIS');
        console.log('='.repeat(60));
        
        // List all collections
        console.log('\n📋 COLLECTIONS IN DATABASE:');
        const collections = await db.listCollections().toArray();
        
        for (const collection of collections) {
            const count = await db.collection(collection.name).countDocuments();
            console.log(`  - ${collection.name}: ${count} documents`);
        }
        
        console.log('\n' + '='.repeat(60));
        
        // Check runtime_flags collection
        console.log('\n🚩 RUNTIME FLAGS COLLECTION:');
        const runtimeFlags = db.collection('runtime_flags');
        const flagsDocs = await runtimeFlags.find().toArray();
        
        for (const doc of flagsDocs) {
            console.log(`\n📄 Document ID: ${doc._id}`);
            for (const [key, value] of Object.entries(doc)) {
                if (key !== '_id') {
                    console.log(`  ${key}: ${value} (${typeof value})`);
                }
            }
        }
        
        console.log('\n' + '='.repeat(60));
        
        // Check configurations collection
        const configCollections = collections.filter(c => 
            c.name.toLowerCase().includes('config')
        );
        
        if (configCollections.length > 0) {
            console.log('\n⚙️  CONFIGURATION COLLECTIONS:');
            
            for (const collection of configCollections) {
                console.log(`\n📁 Collection: ${collection.name}`);
                const coll = db.collection(collection.name);
                const count = await coll.countDocuments();
                console.log(`  Total documents: ${count}`);
                
                if (count > 0) {
                    const sampleDocs = await coll.find().limit(3).toArray();
                    console.log(`  Sample documents:`);
                    
                    for (let i = 0; i < sampleDocs.length; i++) {
                        const doc = sampleDocs[i];
                        console.log(`\n  📄 Document ${i+1}:`);
                        console.log(`    ID: ${doc._id}`);
                        
                        for (const [key, value] of Object.entries(doc)) {
                            if (key !== '_id') {
                                if (typeof value === 'object' && value !== null) {
                                    if (Array.isArray(value)) {
                                        console.log(`    ${key}: Array with ${value.length} items`);
                                    } else {
                                        console.log(`    ${key}: Object with ${Object.keys(value).length} keys`);
                                    }
                                } else {
                                    console.log(`    ${key}: ${value} (${typeof value})`);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            console.log('\n⚠️  No configuration collections found');
        }
        
        console.log('\n' + '='.repeat(60));
        
        // Summary
        console.log('\n📊 SUMMARY:');
        console.log(`  Total collections: ${collections.length}`);
        console.log(`  Runtime flags: ${flagsDocs.length} documents`);
        
        let totalConfigDocs = 0;
        for (const collection of configCollections) {
            const count = await db.collection(collection.name).countDocuments();
            totalConfigDocs += count;
        }
        
        console.log(`  Configuration documents: ${totalConfigDocs}`);
        
        console.log('\n✅ MongoDB data structure analysis completed!');
        
    } catch (error) {
        console.error('❌ Error:', error);
    } finally {
        await client.close();
    }
}

checkMongoDBData();
