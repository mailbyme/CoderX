use super::{Provider, ProviderType, ProviderError, DEFAULT_SYSTEM_PROMPT};
use crate::infrastructure::{HttpClient, JsonParser, JsonValue};

pub struct OpenAIProvider {
    http_client: HttpClient,
}

impl OpenAIProvider {
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
        }
    }

    fn get_api_key() -> Result<String, ProviderError> {
        std::env::var("OPENAI_API_KEY")
            .map_err(|_| ProviderError::AuthError(
                "OPENAI_API_KEY environment variable not set. Please set it with: export OPENAI_API_KEY=your-key".to_string()
            ))
    }

    fn build_request_body(model: &str, max_tokens: usize, temperature: f32, system: &str, messages: &[JsonValue]) -> String {
        let mut all_messages = vec![
            JsonParser::object(&[
                ("role", JsonParser::string("system")),
                ("content", JsonParser::string(system)),
            ])
        ];

        for msg in messages {
            all_messages.push(msg.clone());
        }

        JsonParser::serialize(&JsonParser::object(&[
            ("model", JsonParser::string(model)),
            ("max_tokens", JsonParser::number(max_tokens as f64)),
            ("temperature", JsonParser::number(temperature as f64)),
            ("messages", JsonParser::array(&all_messages)),
        ]))
    }

    fn extract_content(response: &JsonValue) -> Result<String, ProviderError> {
        if let Some(choices) = response.get("choices").and_then(|c| c.as_array()) {
            if let Some(first_choice) = choices.first() {
                if let Some(message) = first_choice.get("message") {
                    if let Some(content) = message.get("content").and_then(|c| c.as_str()) {
                        return Ok(content.to_string());
                    }
                }
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

impl Provider for OpenAIProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::OpenAI
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

        let model = if config.model.starts_with("claude") {
            "gpt-4o"
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
            "https://api.openai.com/v1/chat/completions",
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

impl Default for OpenAIProvider {
    fn default() -> Self {
        Self::new()
    }
}
