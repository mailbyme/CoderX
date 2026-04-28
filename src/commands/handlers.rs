use std::collections::HashMap;
use crate::state::session::SharedSessionState;
use crate::state::message_store::SharedMessageStore;
use crate::i18n::{Language, translate, translate_fmt};
use crate::i18n::{
    COMMAND_HELP, COMMAND_CLEAR, COMMAND_MODEL, COMMAND_PROVIDER, COMMAND_INIT, 
    COMMAND_REVIEW, COMMAND_EXIT, UNKNOWN_COMMAND, MODEL_SET, MODEL_CURRENT,
    PROVIDER_SET, PROVIDER_CURRENT, PROVIDER_INVALID, INIT_START, INIT_CHECK_DIR,
    INIT_LOAD_CONTEXT, INIT_READY, REVIEW_CONTEXT, EXIT_MESSAGE
};

type CommandHandler = fn(&[String], &SharedSessionState, &SharedMessageStore, Language) -> String;

pub struct CommandHandlers {
    registry: HashMap<String, CommandHandler>,
}

impl CommandHandlers {
    pub fn new() -> Self {
        let mut registry: HashMap<String, CommandHandler> = HashMap::new();
        registry.insert("/help".to_string(), Self::handle_help);
        registry.insert("/clear".to_string(), Self::handle_clear);
        registry.insert("/model".to_string(), Self::handle_model);
        registry.insert("/provider".to_string(), Self::handle_provider);
        registry.insert("/init".to_string(), Self::handle_init);
        registry.insert("/review".to_string(), Self::handle_review);
        registry.insert("/exit".to_string(), Self::handle_exit);
        registry.insert("/lang".to_string(), Self::handle_lang);
        registry.insert("/tools".to_string(), Self::handle_tools);
        registry.insert("/帮助".to_string(), Self::handle_help);
        registry.insert("/清空".to_string(), Self::handle_clear);
        registry.insert("/模型".to_string(), Self::handle_model);
        registry.insert("/提供商".to_string(), Self::handle_provider);
        registry.insert("/初始化".to_string(), Self::handle_init);
        registry.insert("/回顾".to_string(), Self::handle_review);
        registry.insert("/退出".to_string(), Self::handle_exit);
        registry.insert("/语言".to_string(), Self::handle_lang);
        registry.insert("/工具".to_string(), Self::handle_tools);
        Self { registry }
    }

    pub fn execute(
        &self,
        cmd_name: &str,
        args: &[String],
        session: &SharedSessionState,
        messages: &SharedMessageStore,
        language: Language,
    ) -> String {
        if let Some(handler) = self.registry.get(cmd_name) {
            handler(args, session, messages, language)
        } else {
            translate_fmt(&UNKNOWN_COMMAND, language, &[cmd_name])
        }
    }

    fn handle_help(_args: &[String], _session: &SharedSessionState, _messages: &SharedMessageStore, lang: Language) -> String {
        if lang == Language::Chinese {
            format!("\
\n\
/help          - 显示帮助信息\n\
/clear         - 清空终端\n\
/lang <en/zh>  - 切换语言 (英文/中文)\n\
/model <name>  - 设置 AI 模型\n\
/provider <name> - 设置 API 提供商\n\
/tools         - 列出可用工具\n\
/init          - 初始化项目\n\
/review        - 查看对话上下文\n\
/config        - 显示当前配置\n\
/set-key <provider> <key> - 设置 API 密钥\n\
/save          - 保存当前会话\n\
/history       - 列出保存的会话\n\
/load <id>     - 加载会话\n\
/delete-history <id> - 删除会话\n\
/git-status    - 显示 git 状态\n\
/git-log [n]   - 显示 git 日志\n\
/commit <msg>  - 提交更改\n\
/push          - 推送到远程仓库\n\
/pull          - 从远程仓库拉取\n\
/exit          - 退出 CoderX\n\
\n"
            )
        } else {
            format!("\
\n\
/help          - Show this help message\n\
/clear         - Clear the terminal\n\
/lang <en/zh>  - Switch language (English/Chinese)\n\
/model <name>  - Set the AI model\n\
/provider <name> - Set API provider\n\
/tools         - List available tools\n\
/init          - Initialize project context\n\
/review        - Review conversation context\n\
/config        - Show current configuration\n\
/set-key <provider> <key> - Set API key\n\
/save          - Save current session\n\
/history       - List saved sessions\n\
/load <id>     - Load session\n\
/delete-history <id> - Delete session\n\
/git-status    - Show git status\n\
/git-log [n]   - Show git log\n\
/commit <msg>  - Commit changes\n\
/push          - Push to remote repo\n\
/pull          - Pull from remote repo\n\
/exit          - Exit CoderX\n\
\n"
            )
        }
    }

    fn handle_clear(_args: &[String], _session: &SharedSessionState, _messages: &SharedMessageStore, _lang: Language) -> String {
        "\x1B[2J\x1B[H".to_string()
    }

    fn handle_model(args: &[String], session: &SharedSessionState, _messages: &SharedMessageStore, lang: Language) -> String {
        if let Some(model) = args.first() {
            let mut config = session.get_config();
            config.model = model.clone();
            session.update_config(config);
            translate_fmt(&MODEL_SET, lang, &[model])
        } else {
            let current_model = &session.get_config().model;
            translate_fmt(&MODEL_CURRENT, lang, &[current_model])
        }
    }

    fn handle_provider(args: &[String], session: &SharedSessionState, _messages: &SharedMessageStore, lang: Language) -> String {
        if let Some(provider) = args.first() {
            let valid_providers = ["anthropic", "openai", "bedrock", "vertex", "foundry"];
            if valid_providers.contains(&provider.as_str()) {
                let mut config = session.get_config();
                config.provider = provider.clone();
                session.update_config(config);
                translate_fmt(&PROVIDER_SET, lang, &[provider])
            } else {
                translate_fmt(&PROVIDER_INVALID, lang, &[&valid_providers.join(", ")])
            }
        } else {
            let current_provider = &session.get_config().provider;
            translate_fmt(&PROVIDER_CURRENT, lang, &[current_provider])
        }
    }

    fn handle_init(_args: &[String], _session: &SharedSessionState, _messages: &SharedMessageStore, lang: Language) -> String {
        format!("\n{}\n{}\n{}\n{}\n\n",
            translate(&INIT_START, lang),
            translate(&INIT_CHECK_DIR, lang),
            translate(&INIT_LOAD_CONTEXT, lang),
            translate(&INIT_READY, lang)
        )
    }

    fn handle_review(_args: &[String], _session: &SharedSessionState, messages: &SharedMessageStore, lang: Language) -> String {
        let count = messages.len().to_string();
        translate_fmt(&REVIEW_CONTEXT, lang, &[&count])
    }

    fn handle_exit(_args: &[String], session: &SharedSessionState, _messages: &SharedMessageStore, lang: Language) -> String {
        session.stop();
        translate(&EXIT_MESSAGE, lang).to_string()
    }

    fn handle_lang(args: &[String], session: &SharedSessionState, _messages: &SharedMessageStore, _lang: Language) -> String {
        if let Some(lang_str) = args.first() {
            let new_lang = Language::from_str(lang_str);
            let mut config = session.get_config();
            config.language = new_lang;
            session.update_config(config);
            format!("Language set to: {}\n", new_lang.display_name())
        } else {
            let current_lang = session.get_config().language;
            format!("Current language: {}\nAvailable: English, Chinese\n", current_lang.display_name())
        }
    }

    fn handle_tools(_args: &[String], _session: &SharedSessionState, _messages: &SharedMessageStore, lang: Language) -> String {
        let header = if lang == Language::Chinese {
            "\n可用工具:\n"
        } else {
            "\nAvailable tools:\n"
        };
        
        let mut result = header.to_string();
        result.push_str("  bash    - Execute shell commands\n");
        result.push_str("  read    - Read file contents\n");
        result.push_str("  write   - Write content to file\n");
        result.push_str("  grep    - Search for patterns in files\n");
        result.push_str("  search  - Alias for grep\n\n");
        
        if lang == Language::Chinese {
            result = header.to_string();
            result.push_str("  bash    - 执行 shell 命令\n");
            result.push_str("  read    - 读取文件内容\n");
            result.push_str("  write   - 写入文件内容\n");
            result.push_str("  grep    - 搜索文件中的模式\n");
            result.push_str("  search  - grep 的别名\n\n");
        }
        
        result
    }
}

impl Default for CommandHandlers {
    fn default() -> Self {
        Self::new()
    }
}
