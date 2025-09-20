# Cleanup old structure after successful migration
Write-Host "🧹 Cleaning up old structure..." -ForegroundColor Yellow

# Remove old chaos-backend directory
if (Test-Path "chaos-backend") {
    Write-Host "  🗑️  Removing old chaos-backend directory..." -ForegroundColor Cyan
    Remove-Item -Path "chaos-backend" -Recurse -Force
    Write-Host "    ✅ Removed old chaos-backend directory" -ForegroundColor Green
} else {
    Write-Host "  ℹ️  Old chaos-backend directory not found" -ForegroundColor Yellow
}

# Remove old services directory (if it exists and is different)
if (Test-Path "services\actor-service") {
    Write-Host "  🗑️  Removing old actor-service directory..." -ForegroundColor Cyan
    Remove-Item -Path "services\actor-service" -Recurse -Force
    Write-Host "    ✅ Removed old actor-service directory" -ForegroundColor Green
}

if (Test-Path "services\combat-service") {
    Write-Host "  🗑️  Removing old combat-service directory..." -ForegroundColor Cyan
    Remove-Item -Path "services\combat-service" -Recurse -Force
    Write-Host "    ✅ Removed old combat-service directory" -ForegroundColor Green
}

if (Test-Path "services\event-service-new") {
    Write-Host "  🗑️  Removing old event-service-new directory..." -ForegroundColor Cyan
    Remove-Item -Path "services\event-service-new" -Recurse -Force
    Write-Host "    ✅ Removed old event-service-new directory" -ForegroundColor Green
}

# Remove temporary scripts
$tempScripts = @("reorganize-structure.ps1", "migrate-code.ps1", "fix-axum-templates.ps1")
foreach ($script in $tempScripts) {
    if (Test-Path $script) {
        Write-Host "  🗑️  Removing temporary script: $script" -ForegroundColor Cyan
        Remove-Item -Path $script -Force
        Write-Host "    ✅ Removed $script" -ForegroundColor Green
    }
}

Write-Host "✅ Cleanup completed!" -ForegroundColor Green
