use crate::infrastructure::json_parser::{JsonParser, JsonValue};
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub provider: ProviderConfig,
    pub general: GeneralConfig,
}

#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub current_provider: String,
    pub anthropic: AnthropicConfig,
    pub openai: OpenAIConfig,
}

#[derive(Debug, Clone)]
pub struct AnthropicConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OpenAIConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GeneralConfig {
    pub language: String,
    pub auto_save: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            provider: ProviderConfig::default(),
            general: GeneralConfig::default(),
        }
    }
}

impl Default for ProviderConfig {
    fn default() -> Self {
        ProviderConfig {
            current_provider: "anthropic".to_string(),
            anthropic: AnthropicConfig::default(),
            openai: OpenAIConfig::default(),
        }
    }
}

impl Default for AnthropicConfig {
    fn default() -> Self {
        AnthropicConfig {
            api_key: None,
            model: "claude-sonnet-4-6".to_string(),
            base_url: None,
        }
    }
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        OpenAIConfig {
            api_key: None,
            model: "gpt-5.4".to_string(),
            base_url: None,
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        GeneralConfig {
            language: "en".to_string(),
            auto_save: true,
        }
    }
}

impl Config {
    pub fn load() -> io::Result<Self> {
        let config_path = Self::get_config_path()?;
        
        if !config_path.exists() {
            let config = Config::default();
            config.save()?;
            return Ok(config);
        }
        
        let mut file = File::open(&config_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        Self::from_json(&content)
    }
    
    pub fn save(&self) -> io::Result<()> {
        let config_path = Self::get_config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let json = self.to_json();
        let mut file = File::create(&config_path)?;
        file.write_all(json.as_bytes())?;
        
        Ok(())
    }
    
    fn get_config_path() -> io::Result<PathBuf> {
        let home = env::var("HOME").or_else(|_| env::var("USERPROFILE"));
        let config_dir = match home {
            Ok(home) => PathBuf::from(home).join(".config").join("coderex"),
            Err(_) => env::current_dir()?.join(".coderex"),
        };
        
        Ok(config_dir.join("config.json"))
    }
    
    fn from_json(json: &str) -> io::Result<Self> {
        let value = JsonParser::parse(json).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("JSON parse error: {}", e))
        })?;
        
        let mut config = Config::default();
        
        if let Some(obj) = value.as_object() {
            if let Some(provider_obj) = obj.get("provider").and_then(|v| v.as_object()) {
                if let Some(current) = provider_obj.get("current_provider").and_then(|v| v.as_str()) {
                    config.provider.current_provider = current.to_string();
                }
                
                if let Some(anthropic_obj) = provider_obj.get("anthropic").and_then(|v| v.as_object()) {
                    if let Some(key) = anthropic_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.anthropic.api_key = Some(key.to_string());
                    }
                    if let Some(model) = anthropic_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.anthropic.model = model.to_string();
                    }
                    if let Some(url) = anthropic_obj.get("base_url").and_then(|v| v.as_str()) {
                        config.provider.anthropic.base_url = Some(url.to_string());
                    }
                }
                
                if let Some(openai_obj) = provider_obj.get("openai").and_then(|v| v.as_object()) {
                    if let Some(key) = openai_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.openai.api_key = Some(key.to_string());
                    }
                    if let Some(model) = openai_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.openai.model = model.to_string();
                    }
                    if let Some(url) = openai_obj.get("base_url").and_then(|v| v.as_str()) {
                        config.provider.openai.base_url = Some(url.to_string());
                    }
                }
            }
            
            if let Some(general_obj) = obj.get("general").and_then(|v| v.as_object()) {
                if let Some(lang) = general_obj.get("language").and_then(|v| v.as_str()) {
                    config.general.language = lang.to_string();
                }
                if let Some(auto_save) = general_obj.get("auto_save").and_then(|v| v.as_bool()) {
                    config.general.auto_save = auto_save;
                }
            }
        }
        
        Ok(config)
    }
    
    fn to_json(&self) -> String {
        let mut anthropic_fields = Vec::new();
        if let Some(key) = &self.provider.anthropic.api_key {
            anthropic_fields.push(("api_key", JsonParser::string(key)));
        }
        anthropic_fields.push(("model", JsonParser::string(&self.provider.anthropic.model)));
        if let Some(url) = &self.provider.anthropic.base_url {
            anthropic_fields.push(("base_url", JsonParser::string(url)));
        }
        let anthropic_obj = JsonParser::object(&anthropic_fields);
        
        let mut openai_fields = Vec::new();
        if let Some(key) = &self.provider.openai.api_key {
            openai_fields.push(("api_key", JsonParser::string(key)));
        }
        openai_fields.push(("model", JsonParser::string(&self.provider.openai.model)));
        if let Some(url) = &self.provider.openai.base_url {
            openai_fields.push(("base_url", JsonParser::string(url)));
        }
        let openai_obj = JsonParser::object(&openai_fields);
        
        let provider_obj = JsonParser::object(&[
            ("current_provider", JsonParser::string(&self.provider.current_provider)),
            ("anthropic", anthropic_obj),
            ("openai", openai_obj),
        ]);
        
        let general_obj = JsonParser::object(&[
            ("language", JsonParser::string(&self.general.language)),
            ("auto_save", JsonParser::bool(self.general.auto_save)),
        ]);
        
        let root_obj = JsonParser::object(&[
            ("provider", provider_obj),
            ("general", general_obj),
        ]);
        
        JsonParser::serialize(&root_obj)
    }
}
