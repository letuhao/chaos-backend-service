#!/usr/bin/env node
/**
 * Chaos World Service Manager
 * A Node.js-based service management tool for Windows services using NSSM
 */

const { exec, spawn } = require('child_process');
const http = require('http');
const util = require('util');

const execAsync = util.promisify(exec);

class ServiceManager {
    constructor() {
        this.services = {
            'api-gateway': 'ChaosWorld-API-Gateway',
            'chaos-backend': 'ChaosWorld-Backend'
        };
        this.endpoints = {
            'api-gateway': 'http://localhost:8080',
            'chaos-backend': 'http://localhost:8081'
        };
    }

    async runCommand(command, check = true) {
        try {
            const { stdout, stderr } = await execAsync(command);
            return { success: true, stdout, stderr };
        } catch (error) {
            if (check) {
                return { success: false, stdout: error.stdout, stderr: error.stderr };
            }
            return { success: true, stdout: error.stdout, stderr: error.stderr };
        }
    }

    async checkServiceStatus(serviceName) {
        const result = await this.runCommand(`sc query "${serviceName}"`, false);
        if (result.success) {
            return result.stdout.includes('RUNNING');
        }
        return false;
    }

    async startService(serviceName) {
        console.log(`🚀 Starting ${serviceName}...`);
        const result = await this.runCommand(`nssm start "${serviceName}"`, false);
        if (result.success) {
            console.log(`✅ ${serviceName} started successfully`);
            return true;
        } else {
            console.log(`❌ Failed to start ${serviceName}: ${result.stderr}`);
            return false;
        }
    }

    async stopService(serviceName) {
        console.log(`🛑 Stopping ${serviceName}...`);
        const result = await this.runCommand(`nssm stop "${serviceName}"`, false);
        if (result.success) {
            console.log(`✅ ${serviceName} stopped successfully`);
            return true;
        } else {
            console.log(`❌ Failed to stop ${serviceName}: ${result.stderr}`);
            return false;
        }
    }

    async restartService(serviceName) {
        console.log(`🔄 Restarting ${serviceName}...`);
        await this.stopService(serviceName);
        await new Promise(resolve => setTimeout(resolve, 2000));
        return await this.startService(serviceName);
    }

    async testEndpoint(serviceName, endpoint) {
        return new Promise((resolve) => {
            const req = http.get(`${endpoint}/health`, (res) => {
                if (res.statusCode === 200) {
                    console.log(`✅ ${serviceName} is responding on ${endpoint}`);
                    resolve(true);
                } else {
                    console.log(`❌ ${serviceName} returned status ${res.statusCode}`);
                    resolve(false);
                }
            });
            
            req.on('error', (err) => {
                console.log(`❌ ${serviceName} is not responding: ${err.message}`);
                resolve(false);
            });
            
            req.setTimeout(5000, () => {
                console.log(`❌ ${serviceName} request timed out`);
                req.destroy();
                resolve(false);
            });
        });
    }

    async checkPort(port) {
        const result = await this.runCommand(`netstat -an | findstr ":${port}"`, false);
        return result.success;
    }

    async status() {
        console.log('='.repeat(60));
        console.log('🔍 CHAOS WORLD SERVICES STATUS');
        console.log('='.repeat(60));
        
        for (const [serviceKey, serviceName] of Object.entries(this.services)) {
            console.log(`\n📋 ${serviceKey.toUpperCase()}:`);
            const isRunning = await this.checkServiceStatus(serviceName);
            const port = serviceKey === 'api-gateway' ? 8080 : 8081;
            const portInUse = await this.checkPort(port);
            const endpointResponding = await this.testEndpoint(serviceKey, this.endpoints[serviceKey]);
            
            const statusIcon = isRunning ? '✅' : '❌';
            const portIcon = portInUse ? '✅' : '❌';
            const endpointIcon = endpointResponding ? '✅' : '❌';
            
            console.log(`   Service: ${statusIcon} ${isRunning ? 'RUNNING' : 'STOPPED'}`);
            console.log(`   Port ${port}: ${portIcon} ${portInUse ? 'IN USE' : 'NOT IN USE'}`);
            console.log(`   Endpoint: ${endpointIcon} ${endpointResponding ? 'RESPONDING' : 'NOT RESPONDING'}`);
        }
    }

    async startAll() {
        console.log('🚀 Starting all Chaos World services...');
        let success = true;
        for (const [serviceKey, serviceName] of Object.entries(this.services)) {
            if (!(await this.startService(serviceName))) {
                success = false;
            }
        }
        return success;
    }

    async stopAll() {
        console.log('🛑 Stopping all Chaos World services...');
        let success = true;
        for (const [serviceKey, serviceName] of Object.entries(this.services)) {
            if (!(await this.stopService(serviceName))) {
                success = false;
            }
        }
        return success;
    }

    async restartAll() {
        console.log('🔄 Restarting all Chaos World services...');
        await this.stopAll();
        await new Promise(resolve => setTimeout(resolve, 3000));
        return await this.startAll();
    }
}

async function main() {
    const manager = new ServiceManager();
    const command = process.argv[2];

    if (!command) {
        console.log('Usage: node service_manager.js <command>');
        console.log('Commands:');
        console.log('  status     - Show service status');
        console.log('  start      - Start all services');
        console.log('  stop       - Stop all services');
        console.log('  restart    - Restart all services');
        console.log('  start-api  - Start API Gateway only');
        console.log('  start-backend - Start Chaos Backend only');
        console.log('  stop-api   - Stop API Gateway only');
        console.log('  stop-backend - Stop Chaos Backend only');
        console.log('  restart-api - Restart API Gateway only');
        console.log('  restart-backend - Restart Chaos Backend only');
        process.exit(1);
    }

    switch (command) {
        case 'status':
            await manager.status();
            break;
        case 'start':
            await manager.startAll();
            break;
        case 'stop':
            await manager.stopAll();
            break;
        case 'restart':
            await manager.restartAll();
            break;
        case 'start-api':
            await manager.startService(manager.services['api-gateway']);
            break;
        case 'start-backend':
            await manager.startService(manager.services['chaos-backend']);
            break;
        case 'stop-api':
            await manager.stopService(manager.services['api-gateway']);
            break;
        case 'stop-backend':
            await manager.stopService(manager.services['chaos-backend']);
            break;
        case 'restart-api':
            await manager.restartService(manager.services['api-gateway']);
            break;
        case 'restart-backend':
            await manager.restartService(manager.services['chaos-backend']);
            break;
        default:
            console.log(`Unknown command: ${command}`);
            process.exit(1);
    }
}

if (require.main === module) {
    main().catch(console.error);
}
