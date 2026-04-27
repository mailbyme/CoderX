use super::{Provider, ProviderType, ProviderError};
use crate::infrastructure::{HttpClient, JsonParser};
use crate::state::Config;

pub struct OpenAIProvider {
    http_client: HttpClient,
}

impl OpenAIProvider {
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

impl Provider for OpenAIProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::OpenAI
    }

    fn generate(&self, context: &str, config: &Config) -> Result<String, ProviderError> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| ProviderError::AuthError("OPENAI_API_KEY not set".to_string()))?;

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
            ("Authorization", &format!("Bearer {}", api_key)),
        ];

        let response = self.http_client.post(
            "api.openai.com",
            443,
            "/v1/chat/completions",
            &body,
            headers,
        ).map_err(ProviderError::HttpError)?;

        Self::parse_response(&response)
    }
}
