use super::{Provider, ProviderType, ProviderError, DEFAULT_SYSTEM_PROMPT};
use crate::infrastructure::{HttpClient, JsonParser, JsonValue};

pub struct AnthropicProvider {
    http_client: HttpClient,
}

impl AnthropicProvider {
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
        }
    }

    fn get_api_key() -> Result<String, ProviderError> {
        std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| ProviderError::AuthError(
                "ANTHROPIC_API_KEY environment variable not set. Please set it with: export ANTHROPIC_API_KEY=your-key".to_string()
            ))
    }

    fn build_request_body(model: &str, max_tokens: usize, temperature: f32, system: &str, messages: &[JsonValue]) -> String {
        let mut message_array = Vec::new();
        for msg in messages {
            message_array.push(msg.clone());
        }

        JsonParser::serialize(&JsonParser::object(&[
            ("model", JsonParser::string(model)),
            ("max_tokens", JsonParser::number(max_tokens as f64)),
            ("temperature", JsonParser::number(temperature as f64)),
            ("system", JsonParser::string(system)),
            ("messages", JsonParser::array(&message_array)),
        ]))
    }

    fn extract_content(response: &JsonValue) -> Result<String, ProviderError> {
        if let Some(content) = response.get("content").and_then(|c| c.as_array()) {
            let mut result = String::new();
            for item in content {
                if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                    result.push_str(text);
                }
            }
            if !result.is_empty() {
                return Ok(result);
            }
        }

        if let Some(error) = response.get("error") {
            let error_msg = error.get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown API error");
            return Err(ProviderError::ApiError(error_msg.to_string()));
        }

        Err(ProviderError::ParseError("Could not extract content from response".to_string()))
    }
}

impl Provider for AnthropicProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::Anthropic
    }

    fn generate(&self, context: &str, config: &crate::state::Config) -> Result<String, ProviderError> {
        self.generate_with_system(DEFAULT_SYSTEM_PROMPT, context, config)
    }

    fn generate_with_system(&self, system: &str, context: &str, config: &crate::state::Config) -> Result<String, ProviderError> {
        let api_key = Self::get_api_key()?;

        let messages = vec![
            JsonParser::object(&[
                ("role", JsonParser::string("user")),
                ("content", JsonParser::string(context)),
            ])
        ];

        let body = Self::build_request_body(
            &config.model,
            config.max_tokens,
            config.temperature,
            system,
            &messages
        );

        let headers = &[
            ("Content-Type", "application/json"),
            ("x-api-key", &api_key),
            ("anthropic-version", "2023-06-01"),
        ];

        let response = self.http_client.post(
            "https://api.anthropic.com/v1/messages",
            &body,
            headers,
        )?;

        if !response.is_success() {
            let error_json = JsonParser::parse(&response.body)
                .unwrap_or(JsonValue::Null);
            let error_msg = error_json.get("error")
                .and_then(|e| e.get("message"))
                .and_then(|m| m.as_str())
                .unwrap_or(&response.body);
            return Err(ProviderError::ApiError(format!("HTTP {}: {}", response.status_code, error_msg)));
        }

        let json = JsonParser::parse(&response.body)?;
        Self::extract_content(&json)
    }
}

impl Default for AnthropicProvider {
    fn default() -> Self {
        Self::new()
    }
}
