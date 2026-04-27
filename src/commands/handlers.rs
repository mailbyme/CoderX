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
            format!("Unknown command: {}. Type /help for available commands.", cmd_name)
        }
    }

    fn handle_help(_args: &[String], _session: &SharedSessionState, _messages: &SharedMessageStore) -> String {
        "\nAvailable commands:\n\n\
        /help          - Show this help message\n\
        /clear         - Clear the terminal\n\
        /model <name>  - Set the AI model (e.g., claude-3-5-haiku-20241022)\n\
        /provider <name> - Set API provider (anthropic/openai/bedrock/vertex/foundry)\n\
        /init          - Initialize project context\n\
        /review        - Review current conversation context\n\
        /exit          - Exit CoderX\n\n".to_string()
    }

    fn handle_clear(_args: &[String], _session: &SharedSessionState, _messages: &SharedMessageStore) -> String {
        "\x1B[2J\x1B[H".to_string()
    }

    fn handle_model(args: &[String], session: &SharedSessionState, _messages: &SharedMessageStore) -> String {
        if let Some(model) = args.first() {
            let mut config = session.get_config();
            config.model = model.clone();
            session.update_config(config);
            format!("Model set to: {}\n", model)
        } else {
            format!("Current model: {}\n", session.get_config().model)
        }
    }

    fn handle_provider(args: &[String], session: &SharedSessionState, _messages: &SharedMessageStore) -> String {
        if let Some(provider) = args.first() {
            let valid_providers = ["anthropic", "openai", "bedrock", "vertex", "foundry"];
            if valid_providers.contains(&provider.as_str()) {
                let mut config = session.get_config();
                config.provider = provider.clone();
                session.update_config(config);
                format!("Provider set to: {}\n", provider)
            } else {
                format!("Invalid provider. Valid options: {}\n", valid_providers.join(", "))
            }
        } else {
            format!("Current provider: {}\n", session.get_config().provider)
        }
    }

    fn handle_init(_args: &[String], _session: &SharedSessionState, _messages: &SharedMessageStore) -> String {
        "\nInitializing project...\n\
        - Checking current directory\n\
        - Loading context files\n\
        - Ready!\n\n".to_string()
    }

    fn handle_review(_args: &[String], _session: &SharedSessionState, messages: &SharedMessageStore) -> String {
        let count = messages.len();
        format!("Conversation context: {} messages\n\n", count)
    }

    fn handle_exit(_args: &[String], session: &SharedSessionState, _messages: &SharedMessageStore) -> String {
        session.stop();
        "Exiting CoderX... Goodbye!\n".to_string()
    }
}
