pub mod anthropic;
pub mod openai;

pub use anthropic::AnthropicProvider;
pub use openai::OpenAIProvider;

use crate::state::Config;

pub trait Provider {
    fn provider_type(&self) -> ProviderType;
    fn generate(&self, context: &str, config: &Config) -> Result<String, ProviderError>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProviderType {
    Anthropic,
    OpenAI,
    Bedrock,
    Vertex,
    Foundry,
}

impl ProviderType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "openai" => Self::OpenAI,
            "bedrock" => Self::Bedrock,
            "vertex" => Self::Vertex,
            "foundry" => Self::Foundry,
            _ => Self::Anthropic,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            ProviderType::Anthropic => "anthropic",
            ProviderType::OpenAI => "openai",
            ProviderType::Bedrock => "bedrock",
            ProviderType::Vertex => "vertex",
            ProviderType::Foundry => "foundry",
        }
    }
}

#[derive(Debug)]
pub enum ProviderError {
    HttpError(std::io::Error),
    AuthError(String),
    ParseError(String),
    ApiError(String),
}
