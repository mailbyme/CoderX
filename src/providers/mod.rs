pub mod anthropic;
pub mod openai;
pub mod bedrock;
pub mod vertex;
pub mod meta;
pub mod mistral;
pub mod qwen;
pub mod wenxin;
pub mod hunyuan;
pub mod glm;
pub mod deepseek;
pub mod yi;
pub mod cohere;
pub mod xiaomi;
pub mod custom;

pub use anthropic::AnthropicProvider;
pub use openai::OpenAIProvider;
pub use bedrock::BedrockProvider;
pub use vertex::VertexProvider;
pub use meta::MetaProvider;
pub use mistral::MistralProvider;
pub use qwen::QwenProvider;
pub use wenxin::WenxinProvider;
pub use hunyuan::HunyuanProvider;
pub use glm::GlmProvider;
pub use deepseek::DeepSeekProvider;
pub use yi::YiProvider;
pub use cohere::CohereProvider;
pub use xiaomi::XiaomiProvider;
pub use custom::CustomProvider;

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
    Meta,
    Mistral,
    Qwen,
    Wenxin,
    Hunyuan,
    Glm,
    DeepSeek,
    Yi,
    Cohere,
    Xiaomi,
    Custom,
}

impl ProviderType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "openai" => Self::OpenAI,
            "bedrock" => Self::Bedrock,
            "vertex" => Self::Vertex,
            "foundry" => Self::Foundry,
            "meta" => Self::Meta,
            "mistral" => Self::Mistral,
            "qwen" => Self::Qwen,
            "wenxin" => Self::Wenxin,
            "hunyuan" => Self::Hunyuan,
            "glm" => Self::Glm,
            "deepseek" => Self::DeepSeek,
            "yi" => Self::Yi,
            "cohere" => Self::Cohere,
            "xiaomi" => Self::Xiaomi,
            "custom" => Self::Custom,
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
            ProviderType::Meta => "meta",
            ProviderType::Mistral => "mistral",
            ProviderType::Qwen => "qwen",
            ProviderType::Wenxin => "wenxin",
            ProviderType::Hunyuan => "hunyuan",
            ProviderType::Glm => "glm",
            ProviderType::DeepSeek => "deepseek",
            ProviderType::Yi => "yi",
            ProviderType::Cohere => "cohere",
            ProviderType::Xiaomi => "xiaomi",
            ProviderType::Custom => "custom",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            ProviderType::Anthropic => "Anthropic",
            ProviderType::OpenAI => "OpenAI",
            ProviderType::Bedrock => "AWS Bedrock",
            ProviderType::Vertex => "Google Vertex AI",
            ProviderType::Foundry => "Anthropic Foundry",
            ProviderType::Meta => "Meta (Llama)",
            ProviderType::Mistral => "Mistral AI",
            ProviderType::Qwen => "Alibaba Cloud (Qwen)",
            ProviderType::Wenxin => "Baidu (Wenxin)",
            ProviderType::Hunyuan => "Tencent (Hunyuan)",
            ProviderType::Glm => "Zhipu AI (GLM)",
            ProviderType::DeepSeek => "DeepSeek",
            ProviderType::Yi => "01.AI (Yi)",
            ProviderType::Cohere => "Cohere",
            ProviderType::Xiaomi => "Xiaomi AI",
            ProviderType::Custom => "Custom API",
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
        ProviderType::Bedrock => Box::new(BedrockProvider::new()),
        ProviderType::Vertex => Box::new(VertexProvider::new()),
        ProviderType::Meta => Box::new(MetaProvider::new()),
        ProviderType::Mistral => Box::new(MistralProvider::new()),
        ProviderType::Qwen => Box::new(QwenProvider::new()),
        ProviderType::Wenxin => Box::new(WenxinProvider::new()),
        ProviderType::Hunyuan => Box::new(HunyuanProvider::new()),
        ProviderType::Glm => Box::new(GlmProvider::new()),
        ProviderType::DeepSeek => Box::new(DeepSeekProvider::new()),
        ProviderType::Yi => Box::new(YiProvider::new()),
        ProviderType::Cohere => Box::new(CohereProvider::new()),
        ProviderType::Xiaomi => Box::new(XiaomiProvider::new()),
        ProviderType::Custom => Box::new(CustomProvider::new()),
        _ => Box::new(AnthropicProvider::new()),
    }
}
