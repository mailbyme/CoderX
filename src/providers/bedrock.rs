use super::{Provider, ProviderType, ProviderError, DEFAULT_SYSTEM_PROMPT};
use crate::infrastructure::{HttpClient, JsonParser};

pub struct BedrockProvider {
    http_client: HttpClient,
    access_key: Option<String>,
    secret_key: Option<String>,
    region: Option<String>,
}

impl BedrockProvider {
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
            access_key: None,
            secret_key: None,
            region: None,
        }
    }

    pub fn new_with_config(
        access_key: Option<String>,
        secret_key: Option<String>,
        region: Option<String>,
    ) -> Self {
        Self {
            http_client: HttpClient::new(),
            access_key,
            secret_key,
            region,
        }
    }

    fn get_access_key(&self) -> Result<String, ProviderError> {
        if let Some(key) = &self.access_key {
            return Ok(key.clone());
        }
        std::env::var("AWS_ACCESS_KEY_ID").map_err(|_| {
            ProviderError::AuthError("AWS_ACCESS_KEY_ID not set. Use /config to set or set environment variable.".to_string())
        })
    }

    fn get_secret_key(&self) -> Result<String, ProviderError> {
        if let Some(key) = &self.secret_key {
            return Ok(key.clone());
        }
        std::env::var("AWS_SECRET_ACCESS_KEY").map_err(|_| {
            ProviderError::AuthError("AWS_SECRET_ACCESS_KEY not set. Use /config to set or set environment variable.".to_string())
        })
    }

    fn get_region(&self) -> String {
        self.region.clone().unwrap_or_else(|| {
            std::env::var("AWS_DEFAULT_REGION").unwrap_or("us-east-1".to_string())
        })
    }

    fn build_request_body(model: &str, max_tokens: usize, temperature: f32, prompt: &str) -> String {
        JsonParser::serialize(&JsonParser::object(&[
            ("modelId", JsonParser::string(model)),
            ("contentType", JsonParser::string("application/json")),
            ("accept", JsonParser::string("application/json")),
            ("body", JsonParser::string(&JsonParser::serialize(&JsonParser::object(&[
                ("anthropic_version", JsonParser::string("bedrock-2023-05-31")),
                ("max_tokens", JsonParser::number(max_tokens as f64)),
                ("temperature", JsonParser::number(temperature as f64)),
                ("messages", JsonParser::array(&[JsonParser::object(&[
                    ("role", JsonParser::string("user")),
                    ("content", JsonParser::array(&[JsonParser::object(&[
                        ("type", JsonParser::string("text")),
                        ("text", JsonParser::string(prompt))
                    ])]))
                ])]))
            ]))))
        ]))
    }

    fn extract_content(response_body: &str) -> Result<String, ProviderError> {
        let value = JsonParser::parse(response_body)?;
        
        if let Some(content) = value.get("content").and_then(|v| v.as_array()) {
            for item in content {
                if let Some(text) = item.get("text").and_then(|v| v.as_str()) {
                    return Ok(text.to_string());
                }
            }
        }
        
        if let Some(error) = value.get("message") {
            return Err(ProviderError::ApiError(format!("Bedrock error: {}", error.as_str().unwrap_or("unknown"))));
        }
        
        Err(ProviderError::ParseError("Failed to parse Bedrock response".to_string()))
    }
}

impl Provider for BedrockProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::Bedrock
    }

    fn generate(&self, context: &str, config: &crate::state::Config) -> Result<String, ProviderError> {
        self.generate_with_system(DEFAULT_SYSTEM_PROMPT, context, config)
    }

    fn generate_with_system(&self, system: &str, context: &str, config: &crate::state::Config) -> Result<String, ProviderError> {
        let _access_key = self.get_access_key()?;
        let _secret_key = self.get_secret_key()?;
        let region = self.get_region();

        let model = if config.model.starts_with("claude") {
            &config.model
        } else {
            "anthropic.claude-sonnet-4-20240229-v1:0"
        };

        let endpoint = format!("https://bedrock-runtime.{}.amazonaws.com/model/{}/invoke", region, model);
        let body = Self::build_request_body(model, config.max_tokens, config.temperature, &format!("{}\n\n{}", system, context));

        let headers = &[
            ("Content-Type", "application/json"),
            ("Accept", "application/json"),
        ];

        let response = self.http_client.post(&endpoint, &body, headers)?;

        if !response.is_success() {
            return Err(ProviderError::ApiError(format!(
                "Bedrock API error: HTTP {} - {}",
                response.status_code,
                response.body
            )));
        }

        Self::extract_content(&response.body)
    }
}

impl Default for BedrockProvider {
    fn default() -> Self {
        Self::new()
    }
}
