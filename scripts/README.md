# Chaos World Service Management Scripts

This directory contains Python scripts for managing all Chaos World backend services using NSSM (Non-Sucking Service Manager).

## Prerequisites

1. **Python 3.7+** - Required for running the scripts
2. **NSSM** - Install using Chocolatey: `choco install nssm`
3. **Administrator privileges** - Required for service management
4. **Built services** - Services must be built before installation

## Available Scripts

### Service Management

| Script | Description | Usage |
|--------|-------------|-------|
| `build_services.py` | Build all services and copy executables | `python build_services.py` |
| `install_services.py` | Install all services as Windows services | `python install_services.py` |
| `start_services.py` | Start all services | `python start_services.py` |
| `stop_services.py` | Stop all services | `python stop_services.py` |
| `uninstall_services.py` | Uninstall all services | `python uninstall_services.py` |
| `check_services.py` | Check status of all services | `python check_services.py` |

### Individual Service Management

| Script | Description | Usage |
|--------|-------------|-------|
| `cms_service_manager.py` | Manage CMS service individually | `python cms_service_manager.py <command>` |
| `user_management_service_manager.py` | Manage User Management service individually | `python user_management_service_manager.py <command>` |
| `cms_service.bat` | Windows batch wrapper for CMS service | `cms_service.bat <command>` |
| `cms_service.ps1` | PowerShell wrapper for CMS service | `.\cms_service.ps1 <command>` |

### Testing and Verification

| Script | Description | Usage |
|--------|-------------|-------|
| `test_build_integration.py` | Test build integration without admin privileges | `python test_build_integration.py` |

## Quick Start

### 1. Build Services
```bash
# Build all services using the build script
python build_services.py

# Or build manually
cargo build --release

# Verify builds
python build_services.py verify

# Clean builds
python build_services.py clean

# Test build integration (no admin required)
python test_build_integration.py
```

### 2. Install Services
```bash
# Run as Administrator
python install_services.py
```

### 3. Start Services
```bash
# Run as Administrator
python start_services.py
```

### 4. Check Status
```bash
python check_services.py
```

## Service Information

### Installed Services
- **ChaosWorld-API-Gateway** - Port 8080
- **ChaosWorld-Backend** - Port 8081  
- **ChaosWorld-CMS** - Port 8083
- **ChaosWorld-UserManagement** - Port 8082

### Service URLs
- API Gateway: http://localhost:8080
- Chaos Backend: http://localhost:8081
- CMS Service: http://localhost:8083
- User Management: http://localhost:8082

### Health Endpoints
- API Gateway: http://localhost:8080/health
- Chaos Backend: http://localhost:8081/health
- CMS Service: http://localhost:8083/health
- User Management: http://localhost:8082/health

## CMS Service Management

The CMS service can be managed individually using the `cms_service_manager.py` script:

```bash
# Start CMS service
python cms_service_manager.py start

# Start on specific port
python cms_service_manager.py start --port 8084

# Run in foreground
python cms_service_manager.py start --foreground

# Stop service
python cms_service_manager.py stop

# Check status
python cms_service_manager.py status

# Test API endpoints
python cms_service_manager.py test

# Show logs
python cms_service_manager.py logs

# Build service
python cms_service_manager.py build
```

## User Management Service Management

The User Management service can be managed individually using the `user_management_service_manager.py` script:

```bash
# Start User Management service
python user_management_service_manager.py start

# Run in foreground (development mode)
python user_management_service_manager.py foreground

# Run on specific port
python user_management_service_manager.py foreground --port 8084

# Stop service
python user_management_service_manager.py stop

# Restart service
python user_management_service_manager.py restart

# Check status
python user_management_service_manager.py status

# Test API endpoints
python user_management_service_manager.py test

# Show logs
python user_management_service_manager.py logs

# Show more log lines
python user_management_service_manager.py logs --lines 100

# Build service
python user_management_service_manager.py build
```

## Configuration

### Environment Variables
- `CMS_PORT` - CMS service port (default: 8083)
- `USER_MANAGEMENT_PORT` - User Management service port (default: 8082)
- `MONGODB_URL` - MongoDB connection URL (default: mongodb://localhost:27017)
- `RUST_LOG` - Log level (default: info)

### Service Configuration
Services are configured with the following settings:
- **Auto-start**: Services start automatically with Windows
- **Log rotation**: Logs rotate daily and when they reach 1MB
- **Error handling**: Services restart automatically on failure
- **Working directory**: `C:\ChaosWorld\services`
- **Log directory**: `C:\ChaosWorld\logs`

## Troubleshooting

### Common Issues

1. **"This script must be run as Administrator"**
   - Right-click PowerShell/Command Prompt and select "Run as administrator"

2. **"NSSM not found"**
   - Install NSSM: `choco install nssm`
   - Or download from: https://nssm.cc/download

3. **"Service executable not found"**
   - Build the services first: `cargo build --release`

4. **"Port already in use"**
   - Check what's using the port: `netstat -an | findstr :8080`
   - Stop conflicting services or use different ports

5. **"Service not responding"**
   - Check service logs in `C:\ChaosWorld\logs\`
   - Restart the service: `python stop_services.py && python start_services.py`

### Log Files
- Service logs: `C:\ChaosWorld\logs\<service-name>.log`
- Error logs: `C:\ChaosWorld\logs\<service-name>-error.log`

### Windows Services
- Open `services.msc` to see all installed services
- Services are named with "ChaosWorld-" prefix
- Services run under LocalSystem account

## Development

### Running Services in Development Mode
```bash
# Start CMS service in foreground for development
python cms_service_manager.py start --foreground

# Or use cargo directly
cd services/content-management-service
cargo run
```

### Testing API Endpoints
```bash
# Test all services
python check_services.py

# Test CMS service specifically
python cms_service_manager.py test

# Test individual endpoints
curl http://localhost:8080/health
curl http://localhost:8081/health
curl http://localhost:8082/health
curl http://localhost:8083/health
```

## Script Features

- **Cross-platform**: Works on Windows with Python
- **Error handling**: Comprehensive error checking and reporting
- **Logging**: Detailed logging with timestamps
- **Health checks**: HTTP endpoint health verification
- **Port checking**: Verify port availability
- **Service status**: Check Windows service status
- **Cleanup**: Proper cleanup on uninstall
- **Administrator check**: Verify admin privileges
- **Dependency checking**: Verify prerequisites

## Support

For issues or questions:
1. Check the logs in `C:\ChaosWorld\logs\`
2. Run `python check_services.py` for status
3. Verify all prerequisites are installed
4. Ensure services are built with `cargo build --release`
