# Manual NSSM Installation

## Step 1: Download NSSM
1. Go to https://nssm.cc/download
2. Download `nssm-2.24.zip`
3. Extract the zip file

## Step 2: Choose Architecture
- For 64-bit Windows: Use `nssm-2.24\win64\nssm.exe`
- For 32-bit Windows: Use `nssm-2.24\win32\nssm.exe`

## Step 3: Install NSSM
1. Copy `nssm.exe` to `C:\Windows\System32\` (requires admin)
2. Or copy to any folder and add that folder to your PATH

## Step 4: Verify Installation
Open Command Prompt as Administrator and run:
```cmd
nssm version
```

## Alternative: Portable Installation
1. Create folder `C:\ChaosWorld\bin\`
2. Copy `nssm.exe` to `C:\ChaosWorld\bin\`
3. Add `C:\ChaosWorld\bin\` to your PATH environment variable
