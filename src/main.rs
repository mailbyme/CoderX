use std::io;

mod terminal;
mod state;
mod commands;
mod tools;
mod providers;
mod infrastructure;
mod utils;
mod i18n;
mod config;

use terminal::Renderer;
use state::{SessionState, MessageStore, Message};
use state::message_store::SharedMessageStore;
use commands::{CommandParser, CommandHandlers};
use providers::Provider;
use i18n::{Language, translate, THINKING};
use tools::ToolRegistry;
use config::Config as AppConfig;

fn create_provider_from_config(config: &AppConfig) -> Box<dyn Provider> {
    match config.provider.current_provider.as_str() {
        "openai" => Box::new(providers::OpenAIProvider::new_with_config(
            config.provider.openai.api_key.clone(),
            config.provider.openai.base_url.clone(),
        )),
        _ => Box::new(providers::AnthropicProvider::new_with_config(
            config.provider.anthropic.api_key.clone(),
            config.provider.anthropic.base_url.clone(),
        )),
    }
}

fn build_context(messages: &SharedMessageStore, prompt: &str) -> String {
    let recent = messages.get_recent(10);
    let mut context = String::new();
    
    for msg in recent {
        context.push_str(&format!("{}: {}\n", msg.role, msg.content));
    }
    context.push_str(&format!("user: {}", prompt));
    context
}

fn process_tool_commands(input: &str, tool_registry: &ToolRegistry) -> Option<String> {
    let input = input.trim();
    
    // Check if input starts with a tool name
    for (tool_name, _) in tool_registry.list() {
        let prefix = format!("{} ", tool_name);
        if input.starts_with(&prefix) || input == tool_name {
            let args = if input.len() > tool_name.len() {
                &input[tool_name.len() + 1..]
            } else {
                ""
            };
            
            match tool_registry.execute(tool_name, args) {
                Ok(result) => return Some(format!("[Tool: {}]\n{}", tool_name, result)),
                Err(e) => return Some(format!("[Tool: {}] Error: {}", tool_name, e)),
            }
        }
    }
    
    None
}

fn main() -> io::Result<()> {
    // Load config from file
    let mut app_config = AppConfig::load()?;
    let language = Language::from_str(&app_config.general.language);
    
    let session = SessionState::new();
    let mut renderer = Renderer::new(language);
    let messages = MessageStore::new(100);
    let command_handlers = CommandHandlers::new();
    let tool_registry = ToolRegistry::new();

    renderer.render_welcome()?;

    while session.is_running() {
        let input = renderer.render_prompt()?;

        match CommandParser::parse(&input) {
            commands::ParseResult::Empty => continue,
            
            commands::ParseResult::Command(cmd_name, args) => {
                let current_lang = Language::from_str(&app_config.general.language);
                let result = handle_config_command(
                    &cmd_name, 
                    &args, 
                    &mut app_config, 
                    &session, 
                    &messages, 
                    current_lang,
                    &command_handlers,
                );
                
                if result.contains("Exiting") || result.contains("退出") {
                    renderer.render_message("system", &result)?;
                    app_config.save()?;
                    break;
                }
                
                if cmd_name == "/lang" || cmd_name == "/语言" {
                    app_config.general.language = session.get_config().language.to_str().to_string();
                    renderer.set_language(Language::from_str(&app_config.general.language));
                    renderer.render_welcome()?;
                    app_config.save()?;
                } else if cmd_name == "/tools" || cmd_name == "/工具" {
                    renderer.terminal().write("\nAvailable tools:\n")?;
                    for (name, desc) in tool_registry.list() {
                        renderer.terminal().write(&format!("  {} - {}\n", name, desc))?;
                    }
                    renderer.terminal().write("\n")?;
                } else if cmd_name == "/config" || cmd_name == "/配置" {
                    renderer.terminal().write(&result)?;
                } else if cmd_name == "/set-key" || cmd_name == "/设置密钥" {
                    renderer.terminal().write(&result)?;
                    app_config.save()?;
                } else if cmd_name == "/provider" || cmd_name == "/提供商" {
                    renderer.terminal().write(&result)?;
                    app_config.save()?;
                } else {
                    renderer.terminal().write(&result)?;
                }
            }
            
            commands::ParseResult::Message(content) => {
                // Check if it's a tool command
                if let Some(tool_result) = process_tool_commands(&content, &tool_registry) {
                    renderer.render_message("tool", &tool_result)?;
                    continue;
                }
                
                // Regular AI query
                messages.add(Message {
                    id: utils::generate_uuid(),
                    role: "user".to_string(),
                    content: content.clone(),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                });

                renderer.render_message("user", &content)?;
                
                let thinking_msg = translate(&THINKING, renderer.language());
                renderer.render_message("system", thinking_msg)?;

                let provider = create_provider_from_config(&app_config);
                
                let context = build_context(&messages, &content);
                
                // Create a compatible config for providers
                let provider_config = state::Config {
                    provider: app_config.provider.current_provider.clone(),
                    model: match app_config.provider.current_provider.as_str() {
                        "openai" => app_config.provider.openai.model.clone(),
                        _ => app_config.provider.anthropic.model.clone(),
                    },
                    language: Language::from_str(&app_config.general.language),
                    temperature: 0.7,
                    max_tokens: 4096,
                };

                match provider.generate(&context, &provider_config) {
                    Ok(response) => {
                        renderer.render_message("assistant", &response)?;
                        
                        messages.add(Message {
                            id: utils::generate_uuid(),
                            role: "assistant".to_string(),
                            content: response,
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        });
                    }
                    Err(e) => {
                        renderer.render_error(&format!("{}", e))?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn handle_config_command(
    cmd_name: &str,
    args: &[String],
    config: &mut AppConfig,
    session: &state::session::SharedSessionState,
    messages: &SharedMessageStore,
    language: Language,
    command_handlers: &CommandHandlers,
) -> String {
    match cmd_name {
        "/config" | "/配置" => {
            format!(
                "\nCoderX Configuration:\n\
                 Provider: {}\n\
                 Anthropic Model: {}\n\
                 OpenAI Model: {}\n\
                 Language: {}\n\
                 Auto-save: {}\n\n",
                config.provider.current_provider,
                config.provider.anthropic.model,
                config.provider.openai.model,
                config.general.language,
                config.general.auto_save,
            )
        }
        "/set-key" | "/设置密钥" => {
            if args.len() < 2 {
                return "Usage: /set-key <provider> <key>\nProviders: anthropic, openai\n".to_string();
            }
            
            let provider = args[0].to_lowercase();
            let key = args[1..].join(" ");
            
            match provider.as_str() {
                "anthropic" => {
                    config.provider.anthropic.api_key = Some(key);
                    "Anthropic API key saved!".to_string()
                }
                "openai" => {
                    config.provider.openai.api_key = Some(key);
                    "OpenAI API key saved!".to_string()
                }
                _ => "Unknown provider. Use 'anthropic' or 'openai'".to_string(),
            }
        }
        "/provider" | "/提供商" => {
            if let Some(provider) = args.first() {
                let provider = provider.to_lowercase();
                match provider.as_str() {
                    "anthropic" | "openai" => {
                        config.provider.current_provider = provider.clone();
                        format!("Provider set to: {}", provider)
                    }
                    _ => "Unknown provider. Use 'anthropic' or 'openai'".to_string(),
                }
            } else {
                format!("Current provider: {}\nAvailable: anthropic, openai\n", 
                    config.provider.current_provider)
            }
        }
        _ => {
            command_handlers.execute(cmd_name, args, session, messages, language)
        }
    }
}
