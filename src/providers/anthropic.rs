use super::{Provider, ProviderType, ProviderError};
use crate::infrastructure::{HttpClient, JsonParser};
use crate::state::Config;

pub struct AnthropicProvider {
    http_client: HttpClient,
}

impl AnthropicProvider {
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
        }
    }

    fn parse_response(response: &str) -> Result<String, ProviderError> {
        let parts: Vec<&str> = response.split("\r\n\r\n").collect();
        if parts.len() < 2 {
            return Err(ProviderError::ParseError("Invalid response format".to_string()));
        }

        let json_str = parts[1];
        Ok(json_str.to_string())
    }
}

impl Provider for AnthropicProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::Anthropic
    }

    fn generate(&self, context: &str, config: &Config) -> Result<String, ProviderError> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| ProviderError::AuthError("ANTHROPIC_API_KEY not set".to_string()))?;

        let body = JsonParser::serialize(&JsonParser::object(&[
            ("model", JsonParser::string(&config.model)),
            ("max_tokens", JsonParser::number(config.max_tokens as f64)),
            ("temperature", JsonParser::number(config.temperature as f64)),
            ("messages", JsonParser::array(&[
                JsonParser::object(&[
                    ("role", JsonParser::string("user")),
                    ("content", JsonParser::string(context)),
                ])
            ])),
        ]));

        let headers = &[
            ("Content-Type", "application/json"),
            ("x-api-key", &api_key),
        ];

        let response = self.http_client.post(
            "api.anthropic.com",
            443,
            "/v1/messages",
            &body,
            headers,
        ).map_err(ProviderError::HttpError)?;

        Self::parse_response(&response)
    }
}
