use super::{Provider, ProviderType, ProviderError, DEFAULT_SYSTEM_PROMPT};
use crate::infrastructure::{HttpClient, JsonParser, JsonValue};

pub struct WenxinProvider {
    http_client: HttpClient,
    api_key: Option<String>,
    base_url: Option<String>,
}

impl WenxinProvider {
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
        std::env::var("WENXIN_API_KEY")
            .map_err(|_| ProviderError::AuthError(
                "WENXIN_API_KEY not set. Use /set-key wenxin <key> or set environment variable".to_string()
            ))
    }

    fn get_base_url(&self) -> String {
        self.base_url.clone()
            .unwrap_or("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/completions".to_string())
    }

    fn build_request_body(model: &str, max_tokens: usize, temperature: f32, system: &str, messages: &[JsonValue]) -> String {
        let mut all_messages = vec![
            JsonParser::object(&[
                ("role", JsonParser::string("user")),
                ("content", JsonParser::string(system)),
            ]),
            JsonParser::object(&[
                ("role", JsonParser::string("assistant")),
                ("content", JsonParser::string("Ok, I understand.")),
            ])
        ];

        for msg in messages {
            all_messages.push(msg.clone());
        }

        JsonParser::serialize(&JsonParser::object(&[
            ("messages", JsonParser::array(&all_messages)),
            ("temperature", JsonParser::number(temperature as f64)),
            ("max_output_tokens", JsonParser::number(max_tokens as f64)),
        ]))
    }

    fn extract_content(response: &JsonValue) -> Result<String, ProviderError> {
        if let Some(result) = response.get("result").and_then(|r| r.as_str()) {
            return Ok(result.to_string());
        }

        if let Some(error_msg) = response.get("error_msg").and_then(|m| m.as_str()) {
            return Err(ProviderError::ApiError(error_msg.to_string()));
        }

        Err(ProviderError::ParseError("Could not extract content from response".to_string()))
    }
}

impl Provider for WenxinProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::Wenxin
    }

    fn generate(&self, context: &str, config: &crate::state::Config) -> Result<String, ProviderError> {
        self.generate_with_system(DEFAULT_SYSTEM_PROMPT, context, config)
    }

    fn generate_with_system(&self, system: &str, context: &str, config: &crate::state::Config) -> Result<String, ProviderError> {
        let api_key = self.get_api_key()?;
        let base_url = self.get_base_url();
        let url_with_key = format!("{}?access_token={}", base_url, api_key);

        let messages = vec![
            JsonParser::object(&[
                ("role", JsonParser::string("user")),
                ("content", JsonParser::string(context)),
            ])
        ];

        let _model = if config.model.starts_with("gpt") || config.model.starts_with("claude") || config.model.starts_with("llama") {
            "ernie-4.0"
        } else {
            &config.model
        };

        let body = Self::build_request_body(
            &config.model,
            config.max_tokens,
            config.temperature,
            system,
            &messages
        );

        let headers = &[
            ("Content-Type", "application/json"),
        ];

        let response = self.http_client.post(
            &url_with_key,
            &body,
            headers,
        )?;

        if !response.is_success() {
            let error_json = JsonParser::parse(&response.body)
                .unwrap_or(JsonValue::Null);
            let error_msg = error_json.get("error_msg")
                .and_then(|m| m.as_str())
                .unwrap_or(&response.body);
            return Err(ProviderError::ApiError(format!("HTTP {}: {}", response.status_code, error_msg)));
        }

        let json = JsonParser::parse(&response.body)?;
        Self::extract_content(&json)
    }
}

impl Default for WenxinProvider {
    fn default() -> Self {
        Self::new()
    }
}
