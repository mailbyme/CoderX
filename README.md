# CoderX
## AI-Powered Coding Assistant
**Pure Rust · Zero Dependencies · Secure · Private · Blazing Fast**

> A terminal-native AI coding assistant built entirely in Rust, with no third-party dependencies. Ship as closed-source binary only.

[中文](#coderx-中文) | [English](#coderx-english)

---

## CoderX（中文）
### AI 驱动的终端编程助手
**纯 Rust 实现 · 零第三方依赖 · 安全隔离 · 本地优先 · 极速响应**

CoderX 是一款**完全用 Rust 从零开发的终端 AI 编程助手**，无任何外部库依赖、无遥测、不上传数据、全程本地可控。

### ✨ 版本与功能

#### 🆓 CoderX Free（永久免费）
##### 🌍 所有AI模型支持
- Anthropic（Claude 3.5 Sonnet/Opus/Haiku）
- OpenAI（GPT-4o/GPT-4 Turbo/GPT-3.5）
- AWS Bedrock
- Google Vertex AI（Gemini）
- Meta（Llama）
- Mistral AI
- 阿里通义千问
- 百度文心一言
- 腾讯混元
- 智谱 GLM
- DeepSeek
- 零一万物 Yi
- Cohere
- 小米 AI
- 自定义私有 API

##### 🎨 完整UI组件库
- 专业组件库（Box/Table/List/Spinner）
- 多主题系统（内置主题 + 自定义主题）
- 语法高亮
- 高级交互效果

##### 📦 完整插件系统（完全免费）
- 插件加载与管理
- 社区插件生态
- 自由开发与分享

##### 🎯 5个基础内置 Abilities
- Commit - 自动 Git 提交
- Review - 基础代码审查
- Explain - 代码解释
- Test - 生成基础测试
- Refactor - 简单重构建议

##### 🛠️ 基础工具与功能
- Bash 命令执行
- File Read/Write
- 代码搜索（Grep）
- 基础 Git 集成
- 本地会话保存/加载
- 基础安全沙箱

---

#### 💼 CoderX Pro（专业版）
##### 🧠 CoderX Recall（回忆系统）
- 长期项目记忆
- 对话关键信息自动提取
- 后台记忆整理与压缩
- 会话历史永久保存

##### 📝 CoderX Blueprint（蓝图规划）
- 任务智能拆解
- 自动生成执行计划
- 无依赖任务并行执行
- 步骤依赖管理

##### 👀 CoderX Inspect（代码检视）
- 完整代码审查（正确性/安全/性能/风格）
- 安全扫描（密钥检测/注入风险）
- 性能分析
- 代码质量报告

##### 🔍 CoderX Index（项目索引）
- 全局项目索引
- 智能代码搜索
- 项目类型自动识别
- 文件模糊定位

##### ⏰ CoderX Pipeline（任务流水线）
- 任务队列与优先级管理
- 定时调度
- 后台静默执行
- 任务状态跟踪

##### ⚙️ 完整高级功能
- 细粒度权限控制
- 完整安全沙箱
- 高级危险检测
- 多项目配置隔离
- 配置导入/导出
- 配置模板库

##### ✨ 无限自定义 Abilities
- 创建任意自定义能力
- 自由迭代优化
- Ability 版本管理
- 导入导出分享

---

### 🚀 快速开始（二进制版）
CoderX **仅提供预编译二进制包，无需编译环境，开箱即用**。

1. 前往 `https://github.com/你的用户名/CoderX/releases` 下载对应系统版本
2. 设置 API Key
```bash
# Anthropic（默认）
export ANTHROPIC_API_KEY="sk-ant-..."

# OpenAI
export OPENAI_API_KEY="sk-..."
```
3. 运行
```bash
# Linux/macOS
./coderx

# Windows
coderx.exe
```

### 📚 常用命令
```
/help          查看帮助
/clear         清空终端
/model         切换模型
/provider      切换服务商
/init          初始化项目
/review        代码审查
/lang          中英文切换
/config        查看配置
/set-key       设置 API 密钥
/save          保存会话
/history       会话历史
/load          加载会话
/git-status    Git 状态
/commit        Git 提交
/push          Git 推送
/pull          Git 拉取
/exit          退出
```

### 🔧 配置方式
- **环境变量**（临时）
- **配置文件**（永久）：`~/.config/coderx/config.json`
```json
{
  "general": {
    "language": "zh",
    "model": "claude-3-5-sonnet-20241022",
    "provider": "anthropic"
  },
  "providers": {
    "anthropic": { "api_key": "..." }
  }
}
```

### 📄 许可证
- 文档：**MIT**
- 软件：**闭源，仅二进制分发**

---

## CoderX (English)
### AI-Powered Terminal Coding Assistant
**Pure Rust · Zero Dependencies · Secure · Private · Blazing Fast**

CoderX is a **terminal-native AI coding assistant built entirely in Rust from scratch**.
No third-party dependencies, no telemetry, no data collection—everything stays local.

### ✨ Editions & Features

#### 🆓 CoderX Free (Permanent Free)
##### 🌍 All 16+ AI Providers
- Anthropic (Claude 3.5 Sonnet/Opus/Haiku)
- OpenAI (GPT-4o/GPT-4 Turbo/GPT-3.5)
- AWS Bedrock
- Google Vertex AI (Gemini)
- Meta (Llama)
- Mistral AI
- Qwen (Alibaba)
- Ernie (Baidu)
- Hunyuan (Tencent)
- GLM (Zhipu AI)
- DeepSeek
- Yi (01.AI)
- Cohere
- Xiaomi AI
- Custom Private API

##### 🎨 Full UI Component Library
- Professional components (Box/Table/List/Spinner)
- Multi-theme system (built-in + custom themes)
- Syntax highlighting
- Advanced interactions

##### 📦 Full Plugin System (100% Free)
- Plugin loading & management
- Community plugin ecosystem
- Free to develop & share

##### 🎯 5 Built-in Abilities
- Commit - Auto git commit
- Review - Basic code review
- Explain - Code explanation
- Test - Basic test generation
- Refactor - Simple refactor suggestions

##### 🛠️ Basic Tools
- Bash execution
- File Read/Write
- Grep search
- Basic Git integration
- Local session save/load
- Basic sandbox

---

#### 💼 CoderX Pro
##### 🧠 CoderX Recall
- Long-term project memory
- Auto-extract key info from conversations
- Background memory consolidation
- Permanent session history

##### 📝 CoderX Blueprint
- Smart task decomposition
- Automatic execution planning
- Parallel execution for independent tasks
- Step dependency management

##### 👀 CoderX Inspect
- Complete code review (correctness/security/performance/style)
- Security scanning (secret detection/injection risks)
- Performance analysis
- Code quality report

##### 🔍 CoderX Index
- Global project indexing
- Smart code search
- Project type auto-detection
- Fuzzy file locating

##### ⏰ CoderX Pipeline
- Task queue & priority management
- Scheduled execution
- Background silent execution
- Task status tracking

##### ⚙️ Complete Advanced Features
- Granular permission control
- Full security sandbox
- Advanced threat detection
- Multi-project config isolation
- Config import/export
- Config template library

##### ✨ Unlimited Custom Abilities
- Create any custom abilities
- Free iteration & optimization
- Ability versioning
- Import/export & sharing

---

### 🚀 Quick Start (Binary Only)
CoderX is **distributed as pre-built binaries—no build environment required**.

1. Download your OS version from `https://github.com/your-username/CoderX/releases`
2. Set your API Key
```bash
# Anthropic (default)
export ANTHROPIC_API_KEY="sk-ant-..."

# OpenAI
export OPENAI_API_KEY="sk-..."
```
3. Run
```bash
# Linux/macOS
./coderx

# Windows
coderx.exe
```

### 📚 Commands
```
/help          Show help
/clear         Clear terminal
/model         Set AI model
/provider      Switch AI provider
/init          Initialize project
/review        Code review
/lang          Switch language
/config        Show config
/set-key       Set API key
/save          Save session
/history       Session history
/load          Load session
/git-status    Git status
/commit        Git commit
/push          Git push
/pull          Git pull
/exit          Exit
```

### 🔧 Configuration
- **Environment variables** (temporary)
- **Config file** (persistent): `~/.config/coderx/config.json`
```json
{
  "general": {
    "language": "en",
    "model": "claude-3-5-sonnet-20241022",
    "provider": "anthropic"
  },
  "providers": {
    "anthropic": { "api_key": "..." }
  }
}
```

### 📄 License
- Documentation: **MIT**
- Software: **Closed-source, binary-only distribution**
