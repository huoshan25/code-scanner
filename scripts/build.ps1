# PowerShell 构建脚本
Write-Host "Starting build process..."

# 创建 bin 目录
New-Item -ItemType Directory -Force -Path bin

# 构建函数
function Build-Target {
    param (
        [string]$Target,
        [string]$Output,
        [string]$Extension = ""
    )

    Write-Host "Building for ${Target}..."

    # 直接使用 cargo 而不是 cross
    cargo build --release

    # 移动二进制文件到 bin 目录
    $sourcePath = "target/release/code-scanner${Extension}"
    $targetPath = "bin/${Output}"

    if (Test-Path $sourcePath) {
        Copy-Item $sourcePath $targetPath -Force
        Write-Host "Successfully copied binary to ${targetPath}"
    } else {
        Write-Host "Error: Could not find binary at ${sourcePath}"
        exit 1
    }
}

# Windows build
Build-Target -Target "x86_64-pc-windows-msvc" -Output "code-scanner.exe" -Extension ".exe"

Write-Host "Build completed successfully!"