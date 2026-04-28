use super::{Provider, ProviderType, ProviderError, DEFAULT_SYSTEM_PROMPT};
use crate::infrastructure::{HttpClient, JsonParser, JsonValue};

pub struct CustomProvider {
    http_client: HttpClient,
    api_key: Option<String>,
    base_url: Option<String>,
}

impl CustomProvider {
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
        std::env::var("CUSTOM_API_KEY")
            .map_err(|_| ProviderError::AuthError(
                "CUSTOM_API_KEY not set. Use /set-key custom <key> or set environment variable".to_string()
            ))
    }

    fn get_base_url(&self) -> Result<String, ProviderError> {
        if let Some(url) = &self.base_url {
            return Ok(url.clone());
        }
        std::env::var("CUSTOM_API_URL")
            .map_err(|_| ProviderError::ConfigError(
                "CUSTOM_API_URL not set. Set base_url via config or environment variable".to_string()
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

        if let Some(text) = response.get("text").and_then(|t| t.as_str()) {
            return Ok(text.to_string());
        }

        if let Some(result) = response.get("result").and_then(|r| r.as_str()) {
            return Ok(result.to_string());
        }

        if let Some(error) = response.get("error") {
            let error_msg = error.get("message")
                .and_then(|m| m.as_str())
                .or_else(|| error.as_str())
                .unwrap_or("Unknown API error");
            return Err(ProviderError::ApiError(error_msg.to_string()));
        }

        Err(ProviderError::ParseError("Could not extract content from response".to_string()))
    }
}

impl Provider for CustomProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::Custom
    }

    fn generate(&self, context: &str, config: &crate::state::Config) -> Result<String, ProviderError> {
        self.generate_with_system(DEFAULT_SYSTEM_PROMPT, context, config)
    }

    fn generate_with_system(&self, system: &str, context: &str, config: &crate::state::Config) -> Result<String, ProviderError> {
        let api_key = self.get_api_key()?;
        let base_url = self.get_base_url()?;

        let messages = vec![
            JsonParser::object(&[
                ("role", JsonParser::string("user")),
                ("content", JsonParser::string(context)),
            ])
        ];

        let model = if config.model.is_empty() || config.model.starts_with("gpt") || config.model.starts_with("claude") {
            "default"
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
            let error_msg = error_json.get("error")
                .and_then(|e| e.get("message"))
                .and_then(|m| m.as_str())
                .or_else(|| error_json.get("message").and_then(|m| m.as_str()))
                .unwrap_or(&response.body);
            return Err(ProviderError::ApiError(format!("HTTP {}: {}", response.status_code, error_msg)));
        }

        let json = JsonParser::parse(&response.body)?;
        Self::extract_content(&json)
    }
}

impl Default for CustomProvider {
    fn default() -> Self {
        Self::new()
    }
}
