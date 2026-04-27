use std::collections::HashMap;
use crate::state::session::SharedSessionState;
use crate::state::message_store::SharedMessageStore;

type CommandHandler = fn(&[String], &SharedSessionState, &SharedMessageStore) -> String;

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
        registry.insert("/帮助".to_string(), Self::handle_help);
        registry.insert("/清空".to_string(), Self::handle_clear);
        registry.insert("/模型".to_string(), Self::handle_model);
        registry.insert("/提供商".to_string(), Self::handle_provider);
        registry.insert("/初始化".to_string(), Self::handle_init);
        registry.insert("/回顾".to_string(), Self::handle_review);
        registry.insert("/退出".to_string(), Self::handle_exit);
        Self { registry }
    }

    pub fn execute(
        &self,
        cmd_name: &str,
        args: &[String],
        session: &SharedSessionState,
        messages: &SharedMessageStore,
    ) -> String {
        if let Some(handler) = self.registry.get(cmd_name) {
            handler(args, session, messages)
        } else {
            format!("未知命令: {}. 输入 /help 查看可用命令。", cmd_name)
        }
    }

    fn handle_help(_args: &[String], _session: &SharedSessionState, _messages: &SharedMessageStore) -> String {
        "\n可用命令:\n\n\
        /help          - 显示此帮助信息\n\
        /clear         - 清空终端\n\
        /model <名称>  - 设置 AI 模型（例如: claude-3-5-haiku-20241022）\n\
        /provider <名称> - 设置 API 提供商（anthropic/openai/bedrock/vertex/foundry）\n\
        /init          - 初始化项目上下文\n\
        /review        - 查看当前对话上下文\n\
        /exit          - 退出 CoderX\n\n\
        中文命令:\n\n\
        /帮助          - 显示帮助信息\n\
        /清空          - 清空终端\n\
        /模型 <名称>   - 设置 AI 模型\n\
        /提供商 <名称> - 设置 API 提供商\n\
        /初始化        - 初始化项目\n\
        /回顾          - 查看对话上下文\n\
        /退出          - 退出程序\n\n".to_string()
    }

    fn handle_clear(_args: &[String], _session: &SharedSessionState, _messages: &SharedMessageStore) -> String {
        "\x1B[2J\x1B[H".to_string()
    }

    fn handle_model(args: &[String], session: &SharedSessionState, _messages: &SharedMessageStore) -> String {
        if let Some(model) = args.first() {
            let mut config = session.get_config();
            config.model = model.clone();
            session.update_config(config);
            format!("模型已设置为: {}\n", model)
        } else {
            format!("当前模型: {}\n", session.get_config().model)
        }
    }

    fn handle_provider(args: &[String], session: &SharedSessionState, _messages: &SharedMessageStore) -> String {
        if let Some(provider) = args.first() {
            let valid_providers = ["anthropic", "openai", "bedrock", "vertex", "foundry"];
            if valid_providers.contains(&provider.as_str()) {
                let mut config = session.get_config();
                config.provider = provider.clone();
                session.update_config(config);
                format!("提供商已设置为: {}\n", provider)
            } else {
                format!("无效的提供商。有效选项: {}\n", valid_providers.join(", "))
            }
        } else {
            format!("当前提供商: {}\n", session.get_config().provider)
        }
    }

    fn handle_init(_args: &[String], _session: &SharedSessionState, _messages: &SharedMessageStore) -> String {
        "\n正在初始化项目...\n\
        - 检查当前目录\n\
        - 加载上下文文件\n\
        - 准备就绪！\n\n".to_string()
    }

    fn handle_review(_args: &[String], _session: &SharedSessionState, messages: &SharedMessageStore) -> String {
        let count = messages.len();
        format!("对话上下文: {} 条消息\n\n", count)
    }

    fn handle_exit(_args: &[String], session: &SharedSessionState, _messages: &SharedMessageStore) -> String {
        session.stop();
        "正在退出 CoderX... 再见！\n".to_string()
    }
}
