pub mod anthropic;
pub mod openai;

pub use anthropic::AnthropicProvider;
pub use openai::OpenAIProvider;

use crate::state::Config;

pub trait Provider {
    fn provider_type(&self) -> ProviderType;
    fn generate(&self, context: &str, config: &Config) -> Result<String, ProviderError>;
    fn generate_with_system(&self, system: &str, context: &str, config: &Config) -> Result<String, ProviderError>;
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

    pub fn display_name(&self) -> &str {
        match self {
            ProviderType::Anthropic => "Anthropic",
            ProviderType::OpenAI => "OpenAI",
            ProviderType::Bedrock => "AWS Bedrock",
            ProviderType::Vertex => "Google Vertex AI",
            ProviderType::Foundry => "Anthropic Foundry",
        }
    }
}

#[derive(Debug)]
pub enum ProviderError {
    HttpError(String),
    AuthError(String),
    ParseError(String),
    ApiError(String),
    ConfigError(String),
    NetworkError(String),
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderError::HttpError(s) => write!(f, "HTTP error: {}", s),
            ProviderError::AuthError(s) => write!(f, "Authentication error: {}", s),
            ProviderError::ParseError(s) => write!(f, "Parse error: {}", s),
            ProviderError::ApiError(s) => write!(f, "API error: {}", s),
            ProviderError::ConfigError(s) => write!(f, "Configuration error: {}", s),
            ProviderError::NetworkError(s) => write!(f, "Network error: {}", s),
        }
    }
}

impl From<crate::infrastructure::HttpError> for ProviderError {
    fn from(e: crate::infrastructure::HttpError) -> Self {
        ProviderError::HttpError(e.to_string())
    }
}

impl From<crate::infrastructure::JsonParseError> for ProviderError {
    fn from(e: crate::infrastructure::JsonParseError) -> Self {
        ProviderError::ParseError(e.to_string())
    }
}

pub const DEFAULT_SYSTEM_PROMPT: &str = "You are CoderX, an AI coding assistant. You help developers with coding tasks, debugging, code review, and software development questions. Be concise, accurate, and helpful.";

pub fn create_provider(provider_type: ProviderType) -> Box<dyn Provider> {
    match provider_type {
        ProviderType::OpenAI => Box::new(OpenAIProvider::new()),
        _ => Box::new(AnthropicProvider::new()),
    }
}
