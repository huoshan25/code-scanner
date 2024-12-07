# Code Scanner

一个用 Rust 编写的代码注释扫描工具，用于扫描项目中的特定注释（如 TODO、FIXME 等）并生成报告。

## 功能特性

- 支持扫描指定目录下的所有文件
- 可配置的注释类型和模式
- 可忽略特定目录和文件类型
- 支持 Markdown 和 HTML 格式的报告输出
- 实时显示扫描进度
- 支持通过配置文件或命令行参数配置

## 安装
确保你已安装 Rust 开发环境，然后克隆此仓库：

```bash
git clone https://github.com/yourusername/code-scanner.git
cd code-scanner
cargo build --release
```
## 使用方法

### 基本用法
```bash
# 使用配置文件扫描目录
code-scanner ./path/to/scan -c code-scanner.yaml

# 指定输出格式和文件
code-scanner ./path/to/scan -f html -o report.html
```

### 配置文件示例 (code-scanner.yaml)

```bash
# 忽略的目录
ignore:
  - node_modules
  - target
  - .git

# 要扫描的文件类型
fileTypes:
  - .rs
  - .js
  - .py

# 注释类型配置
comments:
  - type_name: "TODO"
    patterns:
      - "TODO:"
      - "TODO("
  - type_name: "FIXME"
    patterns:
      - "FIXME:"
      - "FIXME("

# 输出配置
output:
  format: "html"  # 或 "md"
  path: "./report.html"
```

## 项目结构

```mipsasm
code-scanner/
├── src/
│   ├── cli/                    # 命令行接口
│   │   ├── args.rs            # 命令行参数处理
│   │   └── commands.rs        # 命令执行逻辑
│   │
│   ├── core/                   # 核心功能
│   │   ├── scanner.rs         # 文件扫描
│   │   ├── parser.rs          # 注释解析
│   │   └── types.rs           # 公共类型定义
│   │
│   ├── config/                 # 配置相关
│   │   └── loader.rs          # 配置加载和验证
│   │
│   ├── reporters/             # 报告生成器
│   │   ├── markdown.rs        # Markdown 报告
│   │   └── html.rs           # HTML 报告
│   │
│   ├── progress/              # 进度显示
│   │   └── mod.rs
│   │
│   └── main.rs                # 程序入口
│
└── templates/                  # 报告模板
    └── report.html            # HTML 报告模板
```

## 命令行选项

```bash
USAGE:
    code-scanner [OPTIONS] [DIRECTORY]

ARGS:
    <DIRECTORY>    要扫描的目录 [default: .]

OPTIONS:
    -c, --config <FILE>     配置文件路径
    -f, --format <FORMAT>   输出格式 (md/html)
    -o, --output <FILE>     输出文件路径
    -h, --help             显示帮助信息
    -V, --version          显示版本信息
```


## 开发

### 依赖项

- clap: 命令行参数解析
- serde: 配置文件序列化/反序列化
- handlebars: HTML 模板渲染
- walkdir: 文件系统遍历
- anyhow: 错误处理

### 构建和测试

```bash
# 构建项目
cargo build

# 运行测试
cargo test

# 构建发布版本
cargo build --release
```

## 许可证
MIT License

## 贡献
欢迎提交 Issue 和 Pull Request！