#!/usr/bin/env node
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

function getBinaryName() {
    const platform = process.platform;
    const arch = process.arch;

    let binaryName = 'code-scanner';

    // 平台后缀
    switch (platform) {
        case 'win32':
            binaryName += '.exe';
            break;
        case 'darwin':
            binaryName += '-macos';
            break;
        case 'linux':
            binaryName += '-linux';
            break;
        default:
            throw new Error(`Unsupported platform: ${platform}`);
    }

    // 架构后缀
    switch (arch) {
        case 'x64':
            binaryName += '-x86_64';
            break;
        case 'arm64':
            binaryName += '-aarch64';
            break;
        default:
            throw new Error(`Unsupported architecture: ${arch}`);
    }

    return binaryName;
}

function getBinaryPath() {
    return path.join(__dirname, 'bin', getBinaryName());
}

if (process.argv.includes('--install')) {
    const binaryPath = getBinaryPath();
    if (process.platform !== 'win32') {
        try {
            fs.chmodSync(binaryPath, '755');
            console.log(`Successfully set execute permissions for ${binaryPath}`);
        } catch (error) {
            console.error(`Failed to set execute permissions: ${error.message}`);
            process.exit(1);
        }
    }
    process.exit(0);
}

const binary = getBinaryPath();
const args = process.argv.slice(2);

try {
    const child = spawn(binary, args, {
        stdio: 'inherit',
        windowsHide: true
    });

    child.on('error', (err) => {
        console.error(`Failed to start binary: ${err.message}`);
        process.exit(1);
    });

    child.on('exit', (code) => {
        process.exit(code || 0);
    });
} catch (error) {
    console.error(`Failed to execute binary: ${error.message}`);
    process.exit(1);
}