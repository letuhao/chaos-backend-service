# Update MongoDB runtime_flags to use port 8081
Write-Host "Updating MongoDB runtime_flags to use port 8081..." -ForegroundColor Green

# Connect to MongoDB and update the document
$mongoScript = @"
use chaos_game;
db.runtime_flags.updateOne(
    { "_id": "runtime_config" },
    { `$set: { "server_port": 8081 } }
);
print("Updated server_port to 8081");
"@

# Execute the MongoDB script
$mongoScript | mongosh

Write-Host "MongoDB update completed!" -ForegroundColor Green
Write-Host "You can now restart the Chaos Backend service." -ForegroundColor Yellow
