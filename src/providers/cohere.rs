use super::{Provider, ProviderType, ProviderError, DEFAULT_SYSTEM_PROMPT};
use crate::infrastructure::{HttpClient, JsonParser, JsonValue};

pub struct CohereProvider {
    http_client: HttpClient,
    api_key: Option<String>,
    base_url: Option<String>,
}

impl CohereProvider {
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
            api_key: None,
            base_url: None,
        }
    }

    pub fn new_with_config(api_key: Option<String>, base_url: Option<String>) -> Self {
        Self {
            http_client: HttpClient::new(),
            api_key,
            base_url,
        }
    }

    fn get_api_key(&self) -> Result<String, ProviderError> {
        if let Some(key) = &self.api_key {
            return Ok(key.clone());
        }
        std::env::var("COHERE_API_KEY")
            .map_err(|_| ProviderError::AuthError(
                "COHERE_API_KEY not set. Use /set-key cohere <key> or set environment variable".to_string()
            ))
    }

    fn get_base_url(&self) -> String {
        self.base_url.clone()
            .unwrap_or("https://api.cohere.ai/v1/chat".to_string())
    }

    fn build_request_body(model: &str, max_tokens: usize, temperature: f32, system: &str, messages: &[JsonValue]) -> String {
        let mut chat_history = Vec::new();
        chat_history.push(JsonParser::object(&[
            ("role", JsonParser::string("SYSTEM")),
            ("message", JsonParser::string(system)),
        ]));
        
        for msg in messages {
            if let Some(role) = msg.get("role").and_then(|r| r.as_str()) {
                if let Some(content) = msg.get("content").and_then(|c| c.as_str()) {
                    chat_history.push(JsonParser::object(&[
                        ("role", JsonParser::string(if role == "user" { "USER" } else { "CHATBOT" })),
                        ("message", JsonParser::string(content)),
                    ]));
                }
            }
        }

        JsonParser::serialize(&JsonParser::object(&[
            ("model", JsonParser::string(model)),
            ("max_tokens", JsonParser::number(max_tokens as f64)),
            ("temperature", JsonParser::number(temperature as f64)),
            ("chat_history", JsonParser::array(&chat_history)),
        ]))
    }

    fn extract_content(response: &JsonValue) -> Result<String, ProviderError> {
        if let Some(text) = response.get("text").and_then(|t| t.as_str()) {
            return Ok(text.to_string());
        }

        if let Some(error) = response.get("message").and_then(|m| m.as_str()) {
            return Err(ProviderError::ApiError(error.to_string()));
        }

        Err(ProviderError::ParseError("Could not extract content from response".to_string()))
    }
}

impl Provider for CohereProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::Cohere
    }

    fn generate(&self, context: &str, config: &crate::state::Config) -> Result<String, ProviderError> {
        self.generate_with_system(DEFAULT_SYSTEM_PROMPT, context, config)
    }

    fn generate_with_system(&self, system: &str, context: &str, config: &crate::state::Config) -> Result<String, ProviderError> {
        let api_key = self.get_api_key()?;
        let base_url = self.get_base_url();

        let messages = vec![
            JsonParser::object(&[
                ("role", JsonParser::string("user")),
                ("content", JsonParser::string(context)),
            ])
        ];

        let model = if config.model.starts_with("gpt") || config.model.starts_with("claude") || config.model.starts_with("llama") || config.model.starts_with("mistral") {
            "command-r-plus"
        } else {
            &config.model
        };

        let body = Self::build_request_body(
            model,
            config.max_tokens,
            config.temperature,
            system,
            &messages
        );

        let auth_header = format!("Bearer {}", api_key);
        let headers = &[
            ("Content-Type", "application/json"),
            ("Authorization", &auth_header),
        ];

        let response = self.http_client.post(
            &base_url,
            &body,
            headers,
        )?;

        if !response.is_success() {
            let error_json = JsonParser::parse(&response.body)
                .unwrap_or(JsonValue::Null);
            let error_msg = error_json.get("message")
                .and_then(|m| m.as_str())
                .unwrap_or(&response.body);
            return Err(ProviderError::ApiError(format!("HTTP {}: {}", response.status_code, error_msg)));
        }

        let json = JsonParser::parse(&response.body)?;
        Self::extract_content(&json)
    }
}

impl Default for CohereProvider {
    fn default() -> Self {
        Self::new()
    }
}
