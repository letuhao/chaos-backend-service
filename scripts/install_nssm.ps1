# PowerShell script to install NSSM
Write-Host "Installing NSSM (Non-Sucking Service Manager)..." -ForegroundColor Yellow

# Create directory
$nssmDir = "C:\ChaosWorld\bin"
if (!(Test-Path $nssmDir)) {
    New-Item -ItemType Directory -Path $nssmDir -Force
}

# Download NSSM
$nssmUrl = "https://nssm.cc/release/nssm-2.24.zip"
$nssmZip = "C:\ChaosWorld\nssm.zip"

Write-Host "Downloading NSSM..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $nssmUrl -OutFile $nssmZip

# Extract NSSM
Write-Host "Extracting NSSM..." -ForegroundColor Cyan
Expand-Archive -Path $nssmZip -DestinationPath "C:\ChaosWorld" -Force

# Copy executable
$nssmExe = "C:\ChaosWorld\nssm-2.24\win64\nssm.exe"
if (Test-Path $nssmExe) {
    Copy-Item $nssmExe -Destination "$nssmDir\nssm.exe" -Force
    Write-Host "NSSM installed to: $nssmDir\nssm.exe" -ForegroundColor Green
} else {
    Write-Host "Error: NSSM executable not found" -ForegroundColor Red
    exit 1
}

# Clean up
Remove-Item "C:\ChaosWorld\nssm-2.24" -Recurse -Force
Remove-Item $nssmZip -Force

# Add to PATH
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($currentPath -notlike "*$nssmDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$nssmDir", "User")
    Write-Host "Added NSSM to PATH" -ForegroundColor Green
}

Write-Host "NSSM installation completed!" -ForegroundColor Green
Write-Host "You can now use: $nssmDir\nssm.exe" -ForegroundColor Cyan
