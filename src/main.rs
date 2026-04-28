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
use state::{SessionState, MessageStore, Message, HistoryManager};
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
    let mut app_config = AppConfig::load()?;
    let language = Language::from_str(&app_config.general.language);
    
    let session = SessionState::new();
    let mut renderer = Renderer::new(language);
    let messages = MessageStore::new(100);
    let command_handlers = CommandHandlers::new();
    let tool_registry = ToolRegistry::new();
    let history_manager = HistoryManager::new()?;
    let current_session_id = utils::generate_uuid();

    renderer.render_welcome()?;

    while session.is_running() {
        let input = renderer.render_prompt()?;

        match CommandParser::parse(&input) {
            commands::ParseResult::Empty => continue,
            
            commands::ParseResult::Command(cmd_name, args) => {
                let current_lang = Language::from_str(&app_config.general.language);
                let result = handle_command(
                    &cmd_name, 
                    &args, 
                    &mut app_config, 
                    &session, 
                    &messages, 
                    current_lang,
                    &command_handlers,
                    &history_manager,
                    &current_session_id,
                );
                
                if result.contains("Exiting") || result.contains("退出") {
                    renderer.render_message("system", &result)?;
                    if let Err(e) = history_manager.save_history(&current_session_id, &messages) {
                        renderer.render_error(&format!("Failed to save history: {}", e))?;
                    }
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
                } else if cmd_name == "/save" || cmd_name == "/保存" {
                    renderer.terminal().write(&result)?;
                } else if cmd_name == "/history" || cmd_name == "/历史" {
                    renderer.terminal().write(&result)?;
                } else if cmd_name == "/load" || cmd_name == "/加载" {
                    renderer.terminal().write(&result)?;
                } else if cmd_name == "/delete-history" || cmd_name == "/删除历史" {
                    renderer.terminal().write(&result)?;
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

fn handle_command(
    cmd_name: &str,
    args: &[String],
    config: &mut AppConfig,
    session: &state::session::SharedSessionState,
    messages: &SharedMessageStore,
    language: Language,
    command_handlers: &CommandHandlers,
    history_manager: &HistoryManager,
    current_session_id: &str,
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
        "/save" | "/保存" => {
            match history_manager.save_history(current_session_id, messages) {
                Ok(_) => format!("History saved to session: {}\n", current_session_id),
                Err(e) => format!("Failed to save history: {}\n", e),
            }
        }
        "/history" | "/历史" => {
            match history_manager.list_sessions() {
                Ok(sessions) => {
                    if sessions.is_empty() {
                        "No saved sessions found.\n".to_string()
                    } else {
                        let mut result = "\nSaved Sessions:\n".to_string();
                        for session_id in sessions {
                            if let Ok(Some(info)) = history_manager.get_session_info(&session_id) {
                                let preview = if info.preview.len() > 50 {
                                    &info.preview[..50]
                                } else {
                                    &info.preview
                                };
                                result.push_str(&format!(
                                    "  {} - {} messages - {}\n    {}\n",
                                    info.format_time(),
                                    info.message_count,
                                    &session_id[..8],
                                    preview
                                ));
                            }
                        }
                        result
                    }
                }
                Err(e) => format!("Failed to list sessions: {}\n", e),
            }
        }
        "/load" | "/加载" => {
            if args.is_empty() {
                return "Usage: /load <session_id>\nUse /history to list available sessions.\n".to_string();
            }
            
            let session_id = &args.join("");
            match history_manager.load_history(session_id) {
                Ok(loaded_messages) => {
                    let count = loaded_messages.len();
                    for msg in loaded_messages {
                        messages.add(msg);
                    }
                    format!("Loaded {} messages from session {}\n", count, session_id)
                }
                Err(e) => format!("Failed to load session: {}\n", e),
            }
        }
        "/delete-history" | "/删除历史" => {
            if args.is_empty() {
                return "Usage: /delete-history <session_id>\n".to_string();
            }
            
            let session_id = &args.join("");
            match history_manager.delete_session(session_id) {
                Ok(_) => format!("Session {} deleted.\n", session_id),
                Err(e) => format!("Failed to delete session: {}\n", e),
            }
        }
        _ => {
            command_handlers.execute(cmd_name, args, session, messages, language)
        }
    }
}
