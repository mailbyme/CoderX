#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Chinese,
}

impl Language {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "zh" | "zh-cn" | "chinese" | "中文" => Language::Chinese,
            _ => Language::English,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Language::English => "en",
            Language::Chinese => "zh",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Language::English => "English",
            Language::Chinese => "中文",
        }
    }
}

pub struct Translation {
    en: &'static str,
    zh: &'static str,
}

impl Translation {
    pub const fn new(en: &'static str, zh: &'static str) -> Self {
        Self { en, zh }
    }

    pub fn translate(&self, lang: Language) -> &'static str {
        match lang {
            Language::English => self.en,
            Language::Chinese => self.zh,
        }
    }
}

macro_rules! tr {
    ($en:expr, $zh:expr) => {
        Translation::new($en, $zh)
    };
}

pub const WELCOME_TITLE: Translation = tr!(
    "AI-Powered Coding Assistant",
    "AI 驱动的编码助手"
);

pub const WELCOME_HINT: Translation = tr!(
    "Type /help for available commands",
    "输入 /help 查看可用命令"
);

pub const THINKING: Translation = tr!(
    "Thinking...",
    "思考中..."
);

pub const USER: Translation = tr!(
    "[USER]",
    "[用户]"
);

pub const SYSTEM: Translation = tr!(
    "[SYS]",
    "[系统]"
);

pub const TOOL: Translation = tr!(
    "[TOOL]",
    "[工具]"
);

pub const ERROR: Translation = tr!(
    "[ERROR]",
    "[错误]"
);

pub const UNKNOWN: Translation = tr!(
    "[UNKNOWN]",
    "[未知]"
);

pub const COMMAND_HELP: Translation = tr!(
    "/help          - Show this help message",
    "/help          - 显示帮助信息"
);

pub const COMMAND_CLEAR: Translation = tr!(
    "/clear         - Clear the terminal",
    "/clear         - 清空终端"
);

pub const COMMAND_MODEL: Translation = tr!(
    "/model <name>  - Set the AI model",
    "/model <名称>  - 设置 AI 模型"
);

pub const COMMAND_PROVIDER: Translation = tr!(
    "/provider <name> - Set API provider",
    "/provider <名称> - 设置 API 提供商"
);

pub const COMMAND_INIT: Translation = tr!(
    "/init          - Initialize project context",
    "/init          - 初始化项目"
);

pub const COMMAND_REVIEW: Translation = tr!(
    "/review        - Review conversation context",
    "/review        - 查看对话上下文"
);

pub const COMMAND_EXIT: Translation = tr!(
    "/exit          - Exit CoderX",
    "/exit          - 退出 CoderX"
);

pub const UNKNOWN_COMMAND: Translation = tr!(
    "Unknown command: {}. Type /help for available commands.",
    "未知命令: {}. 输入 /help 查看可用命令。"
);

pub const MODEL_SET: Translation = tr!(
    "Model set to: {}",
    "模型已设置为: {}"
);

pub const MODEL_CURRENT: Translation = tr!(
    "Current model: {}",
    "当前模型: {}"
);

pub const PROVIDER_SET: Translation = tr!(
    "Provider set to: {}",
    "提供商已设置为: {}"
);

pub const PROVIDER_CURRENT: Translation = tr!(
    "Current provider: {}",
    "当前提供商: {}"
);

pub const PROVIDER_INVALID: Translation = tr!(
    "Invalid provider. Valid options: {}",
    "无效的提供商。有效选项: {}"
);

pub const INIT_START: Translation = tr!(
    "Initializing project...",
    "正在初始化项目..."
);

pub const INIT_CHECK_DIR: Translation = tr!(
    "- Checking current directory",
    "- 检查当前目录"
);

pub const INIT_LOAD_CONTEXT: Translation = tr!(
    "- Loading context files",
    "- 加载上下文文件"
);

pub const INIT_READY: Translation = tr!(
    "- Ready!",
    "- 准备就绪！"
);

pub const REVIEW_CONTEXT: Translation = tr!(
    "Conversation context: {} messages",
    "对话上下文: {} 条消息"
);

pub const EXIT_MESSAGE: Translation = tr!(
    "Exiting CoderX... Goodbye!",
    "正在退出 CoderX... 再见！"
);

pub fn translate(trans: &Translation, lang: Language) -> &'static str {
    trans.translate(lang)
}

pub fn translate_fmt(trans: &Translation, lang: Language, args: &[&str]) -> String {
    let template = trans.translate(lang);
    let mut result = template.to_string();
    for (i, arg) in args.iter().enumerate() {
        result = result.replace(&format!("{{{}}}", i), arg);
    }
    result
}
