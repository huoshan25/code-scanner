#!/bin/bash
set -e

# 版本号从 package.json 中读取
VERSION=$(node -p "require('./package.json').version")
echo "Building version ${VERSION}"

# 创建 bin 目录
mkdir -p bin

# 构建函数
build() {
    local TARGET=$1
    local OUTPUT=$2

    echo "Building for ${TARGET}..."
    cross cargo build --release --target "${TARGET}"

    # 移动二进制文件到 bin 目录
    cp "target/${TARGET}/release/code-scanner${3}" "bin/${OUTPUT}"

    # 在 Unix 系统上设置执行权限
    if [[ "${OUTPUT}" != *".exe" ]]; then
        chmod +x "bin/${OUTPUT}"
    fi
}

# macOS builds
build "x86_64-apple-darwin" "code-scanner-macos-x86_64" ""
build "aarch64-apple-darwin" "code-scanner-macos-aarch64" ""

# Linux builds
build "x86_64-unknown-linux-gnu" "code-scanner-linux-x86_64" ""
build "aarch64-unknown-linux-gnu" "code-scanner-linux-aarch64" ""

# Windows builds
build "x86_64-pc-windows-msvc" "code-scanner.exe" ".exe"

echo "Build completed successfully!"