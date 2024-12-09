#!/usr/bin/env node

const { scanCode } = require('../index');

async function main() {
    try {
        await scanCode();
    } catch (error) {
        console.error('Error:', error.message);
        process.exit(1);
    }
}

main();