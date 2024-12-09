#!/usr/bin/env node

const {scanCode} = require('../index');
const path = require('path');
const fs = require('fs');

// 默认配置文件名
const DEFAULT_CONFIG_NAME = 'code-scanner.yaml';

async function findConfig() {
    // 首先检查当前目录
    const localConfig = path.join(process.cwd(), DEFAULT_CONFIG_NAME);
    if (fs.existsSync(localConfig)) {
        return localConfig;
    }

    // 如果当前目录没有，可以考虑检查用户主目录
    const homeConfig = path.join(require('os').homedir(), '.code-scanner.yaml');
    if (fs.existsSync(homeConfig)) {
        return homeConfig;
    }

    return null;
}

async function main() {
    try {
        // 检查是否存在配置文件
        const configPath = await findConfig();
        if (!configPath) {
            console.log(`No configuration file found. Creating default ${DEFAULT_CONFIG_NAME}...`);

            // 创建默认配置文件
            const defaultConfig = `ignore:
  - node_modules
  - target
  - dist

fileTypes:
  - ".rs"
  - ".js"
  - ".ts"
  - ".py"

comments:
  - type_name: "TODO"
    patterns:
      - "TODO:"
      - "TODO("
  - type_name: "FIXME"
    patterns:
      - "FIXME:"
      - "FIX:"
  - type_name: "NOTE"
    patterns:
      - "NOTE:"
  - type_name: "HACK"
    patterns:
      - "HACK:"

output:
  format: "html"
  path: "./report.html"
`;
            fs.writeFileSync(DEFAULT_CONFIG_NAME, defaultConfig);
            console.log(`Default configuration file created at ${DEFAULT_CONFIG_NAME}`);
        }

        await scanCode();
    } catch (error) {
        console.error('Error:', error.message);
        process.exit(1);
    }
}

main();