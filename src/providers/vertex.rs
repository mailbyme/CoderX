use super::{Provider, ProviderType, ProviderError, DEFAULT_SYSTEM_PROMPT};
use crate::infrastructure::{HttpClient, JsonParser};

pub struct VertexProvider {
    http_client: HttpClient,
    project_id: Option<String>,
    location: Option<String>,
    api_key: Option<String>,
}

impl VertexProvider {
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
            project_id: None,
            location: None,
            api_key: None,
        }
    }

    pub fn new_with_config(
        project_id: Option<String>,
        location: Option<String>,
        api_key: Option<String>,
    ) -> Self {
        Self {
            http_client: HttpClient::new(),
            project_id,
            location,
            api_key,
        }
    }

    fn get_project_id(&self) -> Result<String, ProviderError> {
        if let Some(id) = &self.project_id {
            return Ok(id.clone());
        }
        std::env::var("GOOGLE_PROJECT_ID").map_err(|_| {
            ProviderError::AuthError("GOOGLE_PROJECT_ID not set. Use /config to set or set environment variable.".to_string())
        })
    }

    fn get_location(&self) -> String {
        self.location.clone().unwrap_or_else(|| {
            std::env::var("GOOGLE_LOCATION").unwrap_or("us-central1".to_string())
        })
    }

    fn get_api_key(&self) -> Result<String, ProviderError> {
        if let Some(key) = &self.api_key {
            return Ok(key.clone());
        }
        std::env::var("GOOGLE_API_KEY").map_err(|_| {
            ProviderError::AuthError("GOOGLE_API_KEY not set. Use /config to set or set environment variable.".to_string())
        })
    }

    fn build_request_body(_model: &str, max_tokens: usize, temperature: f32, prompt: &str) -> String {
        JsonParser::serialize(&JsonParser::object(&[
            ("contents", JsonParser::array(&[JsonParser::object(&[
                ("parts", JsonParser::array(&[JsonParser::object(&[
                    ("text", JsonParser::string(prompt))
                ])]))
            ])])),
            ("generationConfig", JsonParser::object(&[
                ("temperature", JsonParser::number(temperature as f64)),
                ("maxOutputTokens", JsonParser::number(max_tokens as f64)),
                ("topK", JsonParser::number(40.0)),
                ("topP", JsonParser::number(0.95)),
            ])),
        ]))
    }

    fn extract_content(response_body: &str) -> Result<String, ProviderError> {
        let value = JsonParser::parse(response_body)?;
        
        if let Some(candidates) = value.get("candidates").and_then(|v| v.as_array()) {
            if let Some(candidate) = candidates.first() {
                if let Some(content) = candidate.get("content") {
                    if let Some(parts) = content.get("parts").and_then(|v| v.as_array()) {
                        if let Some(part) = parts.first() {
                            if let Some(text) = part.get("text").and_then(|v| v.as_str()) {
                                return Ok(text.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        if let Some(error) = value.get("error") {
            let message = error.get("message").and_then(|v| v.as_str()).unwrap_or("unknown");
            return Err(ProviderError::ApiError(format!("Vertex AI error: {}", message)));
        }
        
        Err(ProviderError::ParseError("Failed to parse Vertex AI response".to_string()))
    }
}

impl Provider for VertexProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::Vertex
    }

    fn generate(&self, context: &str, config: &crate::state::Config) -> Result<String, ProviderError> {
        self.generate_with_system(DEFAULT_SYSTEM_PROMPT, context, config)
    }

    fn generate_with_system(&self, system: &str, context: &str, config: &crate::state::Config) -> Result<String, ProviderError> {
        let project_id = self.get_project_id()?;
        let location = self.get_location();
        let api_key = self.get_api_key()?;

        let model = if config.model.starts_with("gemini") {
            &config.model
        } else {
            "gemini-2.0-flash"
        };

        let endpoint = format!(
            "https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:generateContent?key={}",
            location, project_id, location, model, api_key
        );
        let body = Self::build_request_body(model, config.max_tokens, config.temperature, &format!("{}\n\n{}", system, context));

        let headers = &[
            ("Content-Type", "application/json"),
        ];

        let response = self.http_client.post(&endpoint, &body, headers)?;

        if !response.is_success() {
            return Err(ProviderError::ApiError(format!(
                "Vertex AI API error: HTTP {} - {}",
                response.status_code,
                response.body
            )));
        }

        Self::extract_content(&response.body)
    }
}

impl Default for VertexProvider {
    fn default() -> Self {
        Self::new()
    }
}
