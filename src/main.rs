use std::io;

mod terminal;
mod state;
mod commands;
mod tools;
mod providers;
mod infrastructure;
mod utils;
mod i18n;

use terminal::Renderer;
use state::{SessionState, MessageStore, Message};
use state::message_store::SharedMessageStore;
use commands::{CommandParser, CommandHandlers};
use providers::Provider;
use i18n::{Language, translate, THINKING};

fn create_provider(provider_name: &str) -> Box<dyn Provider> {
    match provider_name {
        "openai" => Box::new(providers::OpenAIProvider::new()),
        _ => Box::new(providers::AnthropicProvider::new()),
    }
}

fn build_context(messages: &SharedMessageStore, prompt: &str) -> String {
    let recent = messages.get_recent(5);
    let mut context = String::new();
    
    for msg in recent {
        context.push_str(&format!("{}: {}\n", msg.role, msg.content));
    }
    context.push_str(&format!("user: {}", prompt));
    context
}

fn main() -> io::Result<()> {
    let session = SessionState::new();
    let config = session.get_config();
    let language = config.language;
    
    let mut renderer = Renderer::new(language);
    let messages = MessageStore::new(100);
    let command_handlers = CommandHandlers::new();

    renderer.render_welcome()?;

    while session.is_running() {
        let input = renderer.render_prompt()?;

        match CommandParser::parse(&input) {
            commands::ParseResult::Empty => continue,
            
            commands::ParseResult::Command(cmd_name, args) => {
                let current_lang = session.get_config().language;
                let result = command_handlers.execute(&cmd_name, &args, &session, &messages, current_lang);
                
                if result.contains("Exiting") || result.contains("退出") {
                    renderer.render_message("system", &result)?;
                    break;
                }
                
                if cmd_name == "/lang" || cmd_name == "/语言" {
                    let new_lang = session.get_config().language;
                    renderer.set_language(new_lang);
                    renderer.render_welcome()?;
                } else {
                    renderer.terminal().write(&result)?;
                }
            }
            
            commands::ParseResult::Message(content) => {
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

                let config = session.get_config();
                let provider = create_provider(&config.provider);
                
                let context = build_context(&messages, &content);

                match provider.generate(&context, &config) {
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
                        renderer.render_error(&format!("{:?}", e))?;
                    }
                }
            }
        }
    }

    Ok(())
}
