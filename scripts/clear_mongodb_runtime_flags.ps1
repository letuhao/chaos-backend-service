# Clear MongoDB runtime_flags to force using hardcoded defaults
Write-Host "Clearing MongoDB runtime_flags to use hardcoded defaults..." -ForegroundColor Green

# Connect to MongoDB and clear the runtime_flags collection
$mongoScript = @"
use chaos_game;
db.runtime_flags.deleteMany({});
print("Cleared runtime_flags collection");
"@

# Execute the MongoDB script
$mongoScript | mongosh

Write-Host "MongoDB runtime_flags cleared!" -ForegroundColor Green
Write-Host "The Chaos Backend will now use hardcoded port 8081." -ForegroundColor Yellow
