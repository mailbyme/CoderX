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
        registry.insert("/帮助".to_string(), Self::handle_help);
        registry.insert("/清空".to_string(), Self::handle_clear);
        registry.insert("/模型".to_string(), Self::handle_model);
        registry.insert("/提供商".to_string(), Self::handle_provider);
        registry.insert("/初始化".to_string(), Self::handle_init);
        registry.insert("/回顾".to_string(), Self::handle_review);
        registry.insert("/退出".to_string(), Self::handle_exit);
        registry.insert("/语言".to_string(), Self::handle_lang);
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
        format!("\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n\n",
            translate(&COMMAND_HELP, lang),
            translate(&COMMAND_CLEAR, lang),
            translate(&COMMAND_MODEL, lang),
            translate(&COMMAND_PROVIDER, lang),
            translate(&COMMAND_INIT, lang),
            translate(&COMMAND_REVIEW, lang),
            translate(&COMMAND_EXIT, lang)
        )
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
}
