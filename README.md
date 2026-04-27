# CoderX

> AI-Powered Coding Assistant

CoderX is a terminal-native AI coding assistant built entirely in Rust with zero third-party dependencies.

## Features

- 🎯 **AI Code Assistance**: Powered by Anthropic, OpenAI, AWS Bedrock, Google Vertex AI, and Anthropic Foundry
- 🛠️ **Built-in Tools**: Bash execution, file operations, code search
- ⚡ **Fast & Lightweight**: Native performance, minimal memory footprint
- 🔐 **Privacy First**: No telemetry, all data stays local
- 📱 **Cross-Platform**: Windows, macOS, and Linux support

## Quick Start

```bash
# Set your API key
export ANTHROPIC_API_KEY="your-api-key"

# Run CoderX
cargo run --release
```

## Commands

| Command | Description |
|---------|-------------|
| `/help` | Show available commands |
| `/clear` | Clear the terminal |
| `/model <name>` | Set AI model |
| `/provider <name>` | Set API provider |
| `/init` | Initialize project |
| `/review` | Review context |
| `/exit` | Exit CoderX |

## API Providers

- **Anthropic** (default) - Set `ANTHROPIC_API_KEY`
- **OpenAI** - Set `CLAUDE_CODE_USE_OPENAI=1` and `OPENAI_API_KEY`
- **AWS Bedrock** - Set `CLAUDE_CODE_USE_BEDROCK=1`
- **Google Vertex** - Set `CLAUDE_CODE_USE_VERTEX=1`
- **Anthropic Foundry** - Set `CLAUDE_CODE_USE_FOUNDRY=1`

## Building

```bash
# Build for your platform
cargo build --release

# Cross-compile for Windows
cargo build --release --target x86_64-pc-windows-msvc
```

## License

MIT
