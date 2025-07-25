# Nya - 跨平台 AI 聊天命令行工具

Nya 是一个用 Rust 编写的简单而强大的命令行工具，可让您直接从终端与 AI 聊天模型进行交互。

## 特性

- **跨平台支持**: 可在 Windows、macOS 和 Linux 上原生运行。
- **直接查询**: 快速向 AI 发送单个问题并获得答复。
- **交互模式**: 进入一个连续的对话会话。
- **配置管理**: 轻松设置和切换 API 密钥、端点和模型。
- **智能分页**: 自动为长响应启用分页器，以改善可读性。
- **格式支持**: 支持纯文本和 Markdown 输出。
- **管道支持**: 支持从标准输入（stdin）通过管道接收数据，实现强大的命令行工作流。

## 安装与使用

确保您的系统上安装了 Rust 和 Cargo。

### 1. 构建项目

首先，克隆仓库并进入项目目录。然后，您可以使用 Cargo 构建项目：

```bash
# 构建调试版本 (用于开发)
cargo build

# 构建优化后的发布版本 (用于生产)
cargo build --release
```

可执行文件将位于 `target/debug/nya` 或 `target/release/nya`。

### 2. 安装到系统

如果您想在系统的任何地方都能使用 `nya` 命令，可以将其安装。

```bash
cargo install --path .
```

这个命令会从当前目录的源代码编译项目，并将生成的可执行文件复制到您的 Cargo 安装路径中（通常是 `~/.cargo/bin/`）。安装后，您就可以直接调用 `nya` 命令。

**注意：** 如果安装后无法直接调用 `nya` 命令，请确保 `~/.cargo/bin` 目录已添加到您 shell 的 `PATH` 环境变量中。

## 用法

### 1. 配置 API

在使用之前，您需要配置您的 API 凭据。

```bash
nya config set --api-key "您的_API_密钥" --api-url "https://api.example.com/v1" --model "gpt-4"
```

您可以省略任何您不想更新的参数。要查看当前配置：

```bash
nya config show
```

### 2. 发送单个问题

直接从命令行提问：

```bash
nya "Rust 中最好的错误处理库是什么？"
```

### 3. 使用交互模式

要开始一个持续的对话，请使用 `-i` 或 `--interactive` 标志：

```bash
nya -i
```

然后，您可以在提示符下键入消息。输入 `exit` 退出会话。

### 4. 管道支持 (Pipe Support)

**示例 1: 分析文件列表**

```bash
ls -l | nya "请总结一下这些文件的特点"
```

**示例 2: 解释代码**

```bash
cat src/main.rs | nya "请解释一下这段 Rust 代码是做什么的"
```

## 命令和选项

### 主要选项

- `[MESSAGE]`: 要发送给 AI 的消息。
- `-i`, `--interactive`: 启用交互式对话模式。
- `-P`, `--no-pager`: 禁用对长输出的分页显示。
- `-f`, `--format <FORMAT>`: 设置输出格式 (`plain` 或 `markdown`)。默认为 `plain`。

### 子命令

- `config`: 管理应用程序配置。
  - `show`: 显示当前配置。
  - `set`: 设置 API 密钥、API 地址和模型。
    - `-k`, `--api-key <KEY>`: 您的 API 密钥。
    - `-u`, `--api-url <URL>`: API 的端点 URL。
    - `-m`, `--model <MODEL>`: 要使用的模型名称。

## 许可证

该项目根据 [LICENSE](LICENSE) 文件中的条款获得许可。

## 备注

本 README 文件由 Gemini 生成。
